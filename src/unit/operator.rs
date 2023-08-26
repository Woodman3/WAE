use std::fmt;

pub struct Operator{
    info:super::UnitInfo,
    location:(u32,u32),
    attack_range:u32,
    re_deploy:u32
}