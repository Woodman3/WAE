use std::{collections::HashMap, default};

// const VEC_LEN:usize=1024;
// mod blockinfo;
// mod placeinfo;
use serde::{Serialize,Deserialize};
use serde_json::{Value,from_value};
use super::Map;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// | ... | 1 | 1 | 3 | | 8 |
/// | save | has deploy | can pass | deploy limit | id |
pub(crate) type LayoutCode=u64;

pub(crate) const ID_MASK_LEN:LayoutCode = 8;
pub(crate) const DEPLOY_MASK_LEN:LayoutCode=3;
pub(crate) const HAS_DEPLOY_MASK_LEN:LayoutCode=1;
pub(crate) const PASS_MASK_LEN:LayoutCode =1;

pub(crate) const ID_MASK:LayoutCode=(1<<ID_MASK_LEN)-1;

pub(crate) const DEPLOY_LOW:LayoutCode=0<<ID_MASK_LEN;
pub(crate) const DEPLOY_HIGH:LayoutCode=1<<ID_MASK_LEN;
pub(crate) const DEPLOY_NONE:LayoutCode=2<<ID_MASK_LEN;

pub(crate) const PASS_ALL:LayoutCode=1<<(ID_MASK_LEN+DEPLOY_MASK_LEN);
pub(crate) const PASS_FLY:LayoutCode=0<<(ID_MASK_LEN+DEPLOY_MASK_LEN);


#[derive(Serialize,Deserialize,Debug)]
struct TileInfo{
    id:u64,
    name:String,
    can_deploy:String,
    can_pass:bool  
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,Copy,PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TileHeight{
    #[default]
    Lowland,
    Highland,
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,Copy,PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TileBuildable{
    #[default]
    None,
    Ranged,
    Melee,
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TilePassable{
    #[default]
    All,
    FlyOnly,
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,Copy)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TileKey{
    #[default]
    TileEmpty, 
    TileWall, 
    TileRoad, 
    TileStart, 
    TileEnd, 
    TileForbidden, 
    TileFlystart, 
    TileFloor, 
    TileFence,
    TileRcmCrate, 
    TileTelout, 
    TileTelin, 
    TileRcmOperator, 
    TileHole, 
    TileGazebo, 
    TileBigforce, 
    TileHealing, 
    TileGrass, 
    TileDefup, 
    TileInfection, 
    TileVolcano, 
    TileFenceBound
}



impl From<TileInfo> for LayoutCode{
    fn from(value: TileInfo) -> Self {
        let mut r:u64=0;
        r|=value.id|ID_MASK;
        match value.can_deploy.as_str() {
            "Only_high" => r|=DEPLOY_HIGH,
            "Only_low" => r|=DEPLOY_LOW,
            "All" => r|=DEPLOY_HIGH|DEPLOY_LOW,
            "No" =>() ,
            &_ =>(),
        }
        r|=(value.can_pass as u64)<<(ID_MASK_LEN+DEPLOY_MASK_LEN);
        r
    }
}

pub(super) fn generate_layout(v:&Value,mut m:Map)-> Result<Map>{
    let temp:HashMap<String,TileInfo> = from_value(v["block"].clone())?;
    let mut table = HashMap::<u64,LayoutCode>::new();
    for (_,b) in temp{
        table.insert(b.id,b.into());
    } 
    for row in m.layout.iter_mut(){
        for i in row.iter_mut(){
            *i=table[i];
        } 
    }
    Ok(m)
}