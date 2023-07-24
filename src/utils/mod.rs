pub mod config;

pub fn get_short_type_name<'a, T>()->&'a str{
    let tn=std::any::type_name::<T>();

    if let Some(index) = tn.rfind(":")  {
        let slice = &tn[index+1 ..];
            return slice
        
    } 
    tn 
}
