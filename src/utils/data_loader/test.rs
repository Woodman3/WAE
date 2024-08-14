use super::*;

#[test]
fn test_loader() {
    let l = Loader::new("./ArknightsGameData").unwrap();
}

#[test]
fn load_operator_test() {
    let l = Loader::new("./ArknightsGameData").unwrap();
    let mut fail_set= Vec::new();
    for ( key,v ) in l.character_table.as_object().unwrap(){
        let name = v["name"].as_str().unwrap().to_string();
        if let Ok(o) = l.load_operator(name.clone(),0,1,1,1){
            continue;
        }else{
            fail_set.push((key.clone(),name));
        }
    }
    println!("operator load failed: {:?}",fail_set);
    assert_eq!(fail_set.len(),0);
}

#[test]
fn load_enemy_test(){
    let l = Loader::new("./ArknightsGameData").unwrap();
    let mut fail_set= Vec::new();
    for ( key,v ) in l.enemy_database.iter(){
        for i in 0..v.len(){
            if let Ok(e) = l.load_enemy(key,i){
                continue;
            }else{
                fail_set.push((key.clone(),i)); 
            }
        }
    }
    println!("enemy load failed: {:?}",fail_set);
    assert_eq!(fail_set.len(),0);
}


fn find_all_file_in_dir<F>(dir: &Path, f:&mut F)
where F:FnMut(&PathBuf)
{
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                // get_value_by_key(&path, list);
                f(&path)
            } else if path.is_dir() {
                find_all_file_in_dir(&path,f);
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
fn load_level_test() {
    let path = "./ArknightsGameData";
    let loader = Loader::new(path).unwrap();
    let mut fail_set = Vec::new();
    let f = |path|{
        if let Err(_) = loader.load_level_by_path(path){
            fail_set.push(path);
        }
    };
    let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels");
    find_all_file_in_dir(path,&mut f);
    let level = loader.load_level_by_name("main_01-01".to_string()).unwrap();
}