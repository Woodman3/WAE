use std::cell::RefCell;
use std::collections::HashMap;
use std::i64;
use std::path::Path;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_path_to_error;

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
use crate::spawner::Spawner;
use crate::spawner::{SubSubWave, SubWave, Wave};
use crate::unit::enemy::Enemy;
use crate::unit::operator::OperatorShared;
use crate::utils::load_json_file;
use crate::utils::math::Grid;
use crate::utils::math::Point;

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialLevelData {
    pub(super) options: OfficialLevelOption,
    pub(super) map_data: OfficialMapData,
    pub(super) routes: Vec<OfficialRoute>,
    pub(super) enemy_db_refs: Vec<OfficialEnemyDbRef>,
    pub(super) waves: Vec<OfficialWave>,
    pub(super) random_seed: i32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialLevelOption {
    pub(super) character_limit: u32,
    pub(super) max_life_point: u32,
    pub(super) initial_cost: u32,
    pub(super) max_cost: u32,
    pub(super) cost_increase_time: f32,
    pub(super) max_play_time: f32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialMapData {
    pub(super) map: Vec<Vec<u64>>,
    pub(super) tiles: Vec<OfficialTile>,
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialTile {
    pub(super) tile_key: TileKey,
    pub(super) height_type: TileHeight,
    pub(super) buildable_type: TileBuildable,
    pub(super) passable_mask: TilePassable,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
    // it also has a palyerSideMask,but up to 2024/7/29, all its value is "ALL",so i ignore it
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialRoute {
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

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialCheckPoint {
    #[serde(alias = "type")]
    pub(super) tag: OfficialCheckPointType,
    pub(super) time: f32,
    pub(super) position: Grid,
    pub(super) reach_offset: Point,
    pub(super) reach_distance: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum OfficialRouteType {
    // up to 2024/7/29, all of its value are below
    #[default]
    ENum,
    Walk,
    Fly,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum OfficialCheckPointType {
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
pub(super) struct OfficialEnemyDbRef {
    pub(super) use_db: bool,
    pub(super) id: String,
    pub(super) level: i32,
    pub(super) overwritten_data: Option<Value>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialWave {
    pub(super) pre_delay: f32,
    pub(super) post_delay: f32,
    pub(super) fragments: Vec<OfficialWaveFragment>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialWaveFragment {
    pub(super) pre_delay: f32,
    pub(super) actions: Vec<OfficialWaveAction>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialWaveAction {
    //up to 2024/8/8 ["SPAWN", "STORY", "DISPLAY_ENEMY_INFO", "PREVIEW_CURSOR", "ACTIVATE_PREDEFINED", "PLAY_OPERA", "PLAY_BGM", "DIALOG", "TRIGGER_PREDEFINED", "BATTLE_EVENTS", "WITHDRAW_PREDEFINED"]
    //it seems only "spawn" related to the enemy behavior
    pub(super) action_type: String,
    pub(super) managed_by_scheduler: bool,
    pub(super) pre_delay: f32,
    pub(super) route_index: u32,
    pub(super) key: String,
    pub(super) count: u32,
    pub(super) interval: f32,
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

impl Into<CheckPoint> for OfficialCheckPoint {
    fn into(self) -> CheckPoint {
        use OfficialCheckPointType::*;
        match self.tag {
            Move => CheckPoint::Move(self.position.into()),
            WaitForSeconds => CheckPoint::WaitForSeconds(self.time),
            Disappear => todo!(),
            AppearAtPos => todo!(),
            WaitCurrentFragmentTime => todo!(),
            WaitCurrentWaveTime => todo!(),
            PatrolMove => todo!(),
        }
    }
}

impl Into<Route> for OfficialRoute {
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
        let enemy = vec![vec![Vec::<Weak<RefCell<Enemy>>>::new(); width as usize]; height as usize];
        let operator = vec![vec![OperatorShared::new(); width as usize]; height as usize];
        Map {
            width: width as u32,
            height: height as u32,
            layout,
            enemy,
            operator,
        }
    }
}

impl Into<SubSubWave> for OfficialWaveAction {
    fn into(self) -> SubSubWave {
        SubSubWave {
            count: self.count as i32,
            interval: self.interval as f32,
            enemy: self.key.clone(),
            route: self.route_index as u32,
            pre_delay: self.pre_delay as f32,
            // cur_delay: self.pre_delay ,
            cur_count: 0,
            // cur_interval: self.interval as f32,
        }
    }
}

impl Into<SubWave> for OfficialWaveFragment {
    fn into(self) -> SubWave {
        let mut wave = Vec::new();
        for a in self.actions.into_iter().rev() {
            if a.action_type == "SPAWN" {
                wave.push(a.into());
            }
        }
        SubWave {
            pre_delay: self.pre_delay,
            wave,
        }
    }
}

impl Into<Wave> for OfficialWave {
    fn into(self) -> Wave {
        let mut wave = Vec::new();
        for f in self.fragments {
            wave.push(f.into());
        }
        wave.reverse();
        Wave {
            pre_delay: self.pre_delay ,
            wave,
        }
    }
}

impl Into<Spawner> for Vec<OfficialWave> {
    fn into(self) -> Spawner {
        let mut wave = Vec::new();
        for w in self.into_iter().rev() {
            wave.push(w.into());
        }
        wave.reverse();
        Spawner { wave }
    }
}

impl Loader {
    fn find_level(&self, level_name: String) -> Result<OfficialLevelData> {
        let path = self.path.join("levels");
        let level_file = format!("level_{}.json", level_name);
        let file_path = find_file_in_dir(&path, &level_file)?;
        let level_json = load_json_file(file_path)?;
        let level = serde_json::from_value::<OfficialLevelData>(level_json)?;

        // use from debug
        // let binding = level_json.to_string();
        // let jd = &mut serde_json::Deserializer::from_str(binding.as_str());
        // let level = serde_path_to_error::deserialize(jd)?;

        Ok(level)
    }

    pub(super) fn debug_level(&self, level_name: String) -> Result<OfficialLevelData> {
        let path = self.path.join("levels");
        let level_file = format!("level_{}.json", level_name);
        let file_path = find_file_in_dir(&path, &level_file)?;
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let jd = &mut serde_json::Deserializer::from_reader(reader);
        let level = serde_path_to_error::deserialize(jd)?;
        Ok(level)
    }

    fn load_map(&self, level: &OfficialLevelData) -> Result<Map> {
        let map: Map = level.map_data.clone().into();
        Ok(map)
    }

    pub(super) fn load_level_by_path(&self, path: &PathBuf) -> Result<Calculator> {
        let level_json = load_json_file(path)?;
        let level = serde_json::from_value::<OfficialLevelData>(level_json)?;
        self.load_level(level)
    }

    /// all level file is format as "level_*.json", so the level_name should be the * part
    pub(crate) fn load_level_by_name(&self, level_name: String) -> Result<Calculator> {
        let level = self.find_level(level_name)?;
        self.load_level(level)
    }

    fn load_level(&self, level: OfficialLevelData) -> Result<Calculator> {
        let map = self.load_map(&level)?;
        let mut enemy_initial = HashMap::new();
        for e in level.enemy_db_refs.iter() {
            let enemy = self.load_enemy(&e.id, e.level as usize)?;
            enemy_initial.insert(e.id.clone(), enemy);
        }
        let mut route = Vec::new();
        for r in level.routes.iter() {
            let r: Route = r.clone().into();
            // r.complete(&map);
            route.push(r);
        }
        let spawner: Spawner = level.waves.clone().into();
        let f = Frame {
            map,
            cost: level.options.initial_cost as f32,
            life_point:level.options.max_life_point as i8,
            ..Default::default()
        };
        let c = Calculator {
            frame_vec: vec![f],
            time_remain: i64::MAX,
            star: -1,
            route,
            enemy_initial,
            spawner,
            ..Default::default()
        };
        return Ok(c);
    }
}
