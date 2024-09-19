// use std::cell::RefCell;
// use std::rc::Rc

use serde_json::json;

use super::*;

#[test]
fn test_loader() {
    let _l = Loader::new("./ArknightsGameData").unwrap();
}

#[test]
fn load_operator_test() {
    let l = Loader::new("./ArknightsGameData").unwrap();
    let mut fail_set = Vec::new();
    let mut success_count = 0;
    for (key, v) in l.character_table.as_object().unwrap() {
        let name = v["name"].as_str().unwrap().to_string();
        if let Ok(_o) = l.load_operator(name.clone(), 0, 1, 1, 1) {
            success_count += 1;
        } else {
            fail_set.push((key.clone(), name));
        }
    }
    println!("operator load success count:{}", success_count);
    println!("load failed count {}: {:?}", fail_set.len(), fail_set);
    assert_eq!(fail_set.len(), 0);
}

#[test]
fn load_enemy_test() {
    let l = Loader::new("./ArknightsGameData").unwrap();
    let mut fail_set = Vec::new();
    let mut success_count = 0;
    let mut tv = Vec::new();
    for (key, v) in l.enemy_database.iter() {
        for i in 0..v.len() {
            if let Ok(_e) = l.load_enemy(key, i) {
                success_count += 1;
                if v[i].enemy_data.attributes.block_cnt.m_defined {
                    let t = v[i].enemy_data.attributes.block_cnt.m_value.unwrap();
                    tv.push((t, v[i].enemy_data.name.clone()));
                }
            } else {
                fail_set.push((key.clone(), i));
            }
        }
    }
    println!("enemy load success count:{}", success_count);
    println!("enemy load failed count {}: {:?}", fail_set.len(), fail_set);
    println!("{:?}", tv);
    assert_eq!(fail_set.len(), 0);
}

fn find_all_file_in_dir<F>(dir: &Path, f: &mut F)
where
    F: FnMut(PathBuf),
{
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                // get_value_by_key(&path, list);
                f(path);
            } else if path.is_dir() {
                find_all_file_in_dir(&path, f);
            }
        }
    }
}

// fn get_value_by_key(path: &Path, list: &mut Vec<String>) {
//     let json = load_json_file(path).unwrap();
//     if let Ok(data) = from_value::<level_loader::OfficialLevelData>(json) {
//         for w in data.waves.iter() {
//             for f in w.fragments.iter() {
//                 for a in f.actions.iter() {
//                     if !list.contains(&a.action_type){
//                         list.push(a.action_type.clone());
//                     }
//                 }
//             }
//         }
//     }
// }

// #[test]
// fn find_all_value() {
//     // let mut value_list=Vec::<(TileBuildable,TileHeight)>::new();
//     let mut value_list = Vec::new();
//     let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels");
//     find_all_file_in_dir(path, &mut value_list);
//     println!("{:?}", value_list);
// }

#[test]
fn load_level_test() {
    let path = "./ArknightsGameData";
    let loader = Loader::new(path).unwrap();
    let mut fail_set = Vec::new();
    let mut success_count = 0;
    let mut f = |path| {
        if let Err(_) = loader.load_level_by_path(&path) {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            fail_set.push(file_name);
        } else {
            success_count += 1;
        }
    };
    // still has 800+ levels failed , fuck
    let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels/activities");
    find_all_file_in_dir(path, &mut f);
    let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels/obt");
    find_all_file_in_dir(path, &mut f);
    println!(
        "success count:{success_count},failed count:{}, {:?}",
        fail_set.len(),
        fail_set
    );
    assert_eq!(fail_set.len(), 0);

    // loader.debug_level("act10d5_01".into()).unwrap();
}

#[test]
fn test_skill() {
    let mut l: Loader = Loader::new("./ArknightsGameData").unwrap();
    let mut list = HashMap::new();
    let mut fail_skill = vec![];
    let mut fail_o = vec![];
    for (_, v) in l.character_table.as_object().unwrap() {
        if let Ok(o) = from_value::<operator_loader::OfficialOperator>(v.clone()) {
            for sk in o.skills {
                match serde_path_to_error::deserialize::<Value, Vec<operator_loader::OfficialSkill>>(
                l.skill_table[sk.skill_id.clone()]["levels"].clone()){
                    Ok(sv) => {
                        for s in sv {
                            for b in s.blackboard {
                                let key = b.key;
                                if list.contains_key(&key) {
                                    *list.get_mut(&key).unwrap() += 1;
                                } else {
                                    list.insert(key, 1);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        fail_skill.push((sk.skill_id,e.path().to_string()));
                    }
                };
            }
        } else {
            fail_o.push(v["name"].as_str().unwrap().to_string());
        }
    }
    let mut list: Vec<(_, i32)> = list.into_iter().collect();
    list.sort_by(|a, b| b.1.cmp(&a.1));
    println!("skill blackboard :{:?}", list);
    println!("-------------------");
    println!("fail skill :{:?}", fail_skill);
    println!("-------------------");
    println!("fail operator :{:?}", fail_o);
}

#[test]
fn single_test() {
    let path = "./ArknightsGameData";
    let loader = Loader::new(path).unwrap();
    loader.load_operator("芙蓉".into(), 0, 1, 1, 1).unwrap();

}

