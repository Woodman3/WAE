use std::collections::HashMap;
use std::collections::VecDeque;
use std::i64;
use std::path::Path;
use std::rc::{Rc,Weak};
use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Loader;
use super::Result;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::map::tile::DEPLOY_HIGH;
use crate::map::tile::DEPLOY_LOW;
use crate::map::tile::DEPLOY_NONE;
use crate::map::tile::PASS_ALL;
use crate::map::tile::PASS_FLY;
use crate::map::tile::{LayoutCode, TileBuildable, TileHeight, TileKey, TilePassable};
use crate::map::Map;
use crate::route::CheckPoint;
use crate::route::Route;
use crate::timeline;
use crate::timeline::hostile::EnemyPlaceEvent;
use crate::timeline::EventWithTime;
use crate::unit::enemy::Enemy;
use crate::utils::load_json_file;
use crate::utils::math::Grid;
use crate::utils::math::Point;

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialLevelData {
    pub(super) options: Value,
    pub(super) map_data: OfficialMapData,
    pub(super) routes: Vec<OfficialRoute>,
    pub(super) enemy_db_refs: Vec<OfficialEnemyDbRef>,
    pub(super) waves: Vec<OfficialWave>,
    pub(super) random_seed: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OfficialMapData {
    pub(super) map: Vec<Vec<u64>>,
    pub(super) tiles: Vec<OfficialTile>,
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct OfficialTile {
    pub(super) tile_key: TileKey,
    pub(super) height_type: TileHeight,
    pub(super) buildable_type: TileBuildable,
    pub(super) passable_mask: TilePassable,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
    // it also has a palyerSideMask,but up to 2024/7/29, all its value is "ALL",so i ignore it
}

#[derive(Deserialize, Default, Debug,Clone)]
#[serde(rename_all = "camelCase")]
struct OfficialRoute {
    pub(super) motion_mode: OfficialRouteType,
    pub(super) start_position: Grid,
    pub(super) end_position: Grid,
    pub(super) spawn_random_range: Point,
    pub(super) spawn_offset: Point,
    pub(super) checkpoints: Option<Vec<OfficialCheckPoint>>,
    pub(super) allow_diagonal_move: bool,
    pub(super) visit_every_tile_center: bool,
    pub(super) visit_every_node_center: bool,
    pub(super) visit_every_check_point: bool,
}

#[derive(Deserialize, Default, Debug,Clone)]
#[serde(rename_all = "camelCase")]
struct OfficialCheckPoint {
    #[serde(alias = "type")]
    pub(super) tag: OfficialCheckPointType,
    pub(super) time: f64,
    pub(super) position: Grid,
    pub(super) reach_offset: Point,
    pub(super) reach_distance: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OfficialRouteType {
    // up to 2024/7/29, all of its value are below
    #[default]
    ENum,
    Walk,
    Fly,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OfficialCheckPointType {
    // up to 2024/7/29, all of its value are below
    #[default]
    Move,
    WaitForSeconds,
    Disappear,
    AppearAtPos,
    WaitCurrentFragmentTime,
    WaitCurrentWaveTime,
    PatrolMove,
}
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialEnemyDbRef {
    pub(super) use_db: bool,
    pub(super) id: String,
    pub(super) level: i32,
    pub(super) overwritten_data: Option<Value>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialWave {
    pub(super) pre_delay: f32,
    pub(super) post_delay: f32,
    pub(super) fragments: Vec<OfficialWaveFragment>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialWaveFragment {
    pub(super) pre_delay: f32,
    pub(super) actions: Vec<OfficialWaveAction>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialWaveAction {
    //up to 2024/8/8 ["SPAWN", "STORY", "DISPLAY_ENEMY_INFO", "PREVIEW_CURSOR", "ACTIVATE_PREDEFINED", "PLAY_OPERA", "PLAY_BGM", "DIALOG", "TRIGGER_PREDEFINED", "BATTLE_EVENTS", "WITHDRAW_PREDEFINED"]
    //it seems only "spawn" related to the enemy behavior
    pub(super) action_type: String,
    pub(super) pre_delay: f32,
    pub(super) route_index: u32,
    pub(super) key:String,
}

fn find_file_in_dir(dir: &Path, file_name: &str) -> Result<String> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |f| f == file_name) {
                return Ok(path.to_string_lossy().into_owned());
            } else if path.is_dir() {
                if let Ok(found) = find_file_in_dir(&path, file_name) {
                    return Ok(found);
                }
            }
        }
    }
    Err("File not found".into())
}

impl Into<CheckPoint> for OfficialCheckPoint{
    fn into(self) -> CheckPoint {
        match self.tag {
            OfficialCheckPointType::Move => CheckPoint::Move(self.position.into()),
            _ => CheckPoint::None,
        }
    }
}

impl Into<Route> for OfficialRoute{
    fn into(self) -> Route {
        let mut checkpoints = Vec::new();
        if let Some(c) = self.checkpoints {
            for cp in c {
                checkpoints.push(cp.into());
            }
        }
        Route {
            start: self.start_position.into(),
            end: self.end_position.into(),
            checkpoints,
        }
    }
}


impl Into<LayoutCode> for OfficialTile {
    fn into(self) -> LayoutCode {
        let mut c = 0;
        c |= self.tile_key as u64;
        // up to 2024/7/29,all of (TileBuildable,TilePassable)  have only 4 value :
        // [(None, Highland), (None, Lowland), (Melee, Lowland), (Ranged, Highland)]
        c |= match self.height_type {
            TileHeight::Lowland => DEPLOY_LOW,
            TileHeight::Highland => DEPLOY_HIGH,
        };
        c |= match self.buildable_type {
            TileBuildable::None => DEPLOY_NONE,
            _ => 0,
        };
        c |= match self.passable_mask {
            TilePassable::FlyOnly => PASS_FLY,
            TilePassable::All => PASS_ALL,
        };
        c
    }
}



//todo:there stil have some problem
impl Into<Map> for OfficialMapData {
    fn into(self) -> Map {
        let width = self.map[0].len();
        let height = self.map.len();
        let mut layout = vec![vec![0; width]; height];
        for i in 0..height {
            for j in 0..width {
                layout[i][j] = self.tiles[self.map[i][j] as usize].into();
            }
        }
        let enemy =
            vec![vec![Vec::<Weak<RefCell<Enemy>>>::new(); width as usize]; height as usize];
        let operator = vec![vec![None; width as usize]; height as usize];
        Map {
            width: width as u32,
            height: height as u32,
            layout,
            enemy,
            operator,
        }
    }
}
impl Loader {
    fn find_level(&self, level_name: String) -> Result<OfficialLevelData> {
        let path = self.path.join("levels");
        let level_file = format!("level_{}.json", level_name);
        let file_path = find_file_in_dir(&path, &level_file)?;
        let level_json = load_json_file(file_path)?;
        let level = serde_json::from_value::<OfficialLevelData>(level_json)?;
        Ok(level)
    }
    fn load_map(&self, level: &OfficialLevelData) -> Result<Map> {
        let map: Map = level.map_data.clone().into();
        Ok(map)
    }

    /// all level file is format as "level_*.json", so the level_name should be the * part
    pub(crate) fn load_level(&self, level_name: String) -> Result<Calculator> {
        let level = self.find_level(level_name)?;
        let map = self.load_map(&level)?;
        let mut enemy_initial = HashMap::new();
        for e in level.enemy_db_refs.iter() {
            let enemy = self.load_official_enemy(&e.id, e.level as usize)?;
            enemy_initial.insert(e.id.clone(), enemy);
        };
        let mut route = Vec::new();
        for r in level.routes.iter(){
            route.push(Rc::new(r.clone().into()));
        }
        let mut timeline = VecDeque::new();
        for w in level.waves.iter(){
            for f in w.fragments.iter(){
                for a in f.actions.iter(){
                    if a.route_index>=level.routes.len() as u32{
                        return Err("route index out of range".into());
                    }
                    if !matches!(a.action_type.as_str(),"SPAWN"){
                        continue;
                    }
                    let e = EnemyPlaceEvent{
                        enemy_key:a.key.clone(),
                        enemy_route:a.route_index as usize,
                    };
                    let e = EventWithTime{
                        time_stamp:(f.pre_delay+a.pre_delay) as u64,
                        event:Rc::new(timeline::Event::EnemyPlaceEvent(e)),
                    };
                    timeline.push_back(e);
                }
            }
        };
        let f = Frame {
            map,
            ..Default::default()
        };
        let c = Calculator {
            frame_vec: vec![f],
            time_remain: i64::MAX,
            last_enemy_time: 0,
            star: -1,
            timeline,
            route,
            enemy_initial,
        };
        return Ok(c);
        todo!("load route")
    }
}

#[cfg(test)]
mod test {

    use serde_json::from_value;

    use super::*;

    fn find_all_file_in_dir(dir: &Path, list: &mut Vec<String>) {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    get_value_by_key(&path, list);
                } else if path.is_dir() {
                    find_all_file_in_dir(&path, list);
                }
            }
        }
    }

    fn get_value_by_key(path: &Path, list: &mut Vec<String>) {
        let json = load_json_file(path).unwrap();
        if let Ok(data) = from_value::<OfficialLevelData>(json) {
            for w in data.waves.iter() {
                for f in w.fragments.iter() {
                    for a in f.actions.iter() {
                        if !list.contains(&a.action_type){
                            list.push(a.action_type.clone());
                        }
                    }
                }
            }
        }
    }

    // #[test]
    // fn find_all_value() {
    //     // let mut value_list=Vec::<(TileBuildable,TileHeight)>::new();
    //     let mut value_list = Vec::new();
    //     let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels");
    //     find_all_file_in_dir(path, &mut value_list);
    //     println!("{:?}", value_list);
    // }

    #[test]
    fn test_load_level() {
        let path = "./ArknightsGameData";
        let loader = Loader::new(path).unwrap();
        // let c = loader.load_level("act5d0_ex07".to_string()).unwrap();
        let level = loader.load_level("main_01-01".to_string()).unwrap();
        for t in level.timeline.iter(){
            println!("{:?}",t);
        }
    }
}
