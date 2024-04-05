/*
use crate::presentations::presens::new_presentation;
use crate::presentations::presens::edt_presentation;
use crate::presentations::presens::del_presentation;
use crate::presentations::presens::gen_presentation;
*/

pub fn one_single_target(args:&Vec<String>) -> bool {
    let possible_tgs:Vec<String> = vec!["-p".to_string(),"-f".to_string(), "-exp".to_string(), "-docker".to_string(), "-wasm".to_string(), "-cpp".to_string()];
    let mut multiple:usize= 0;

    for out in possible_tgs.iter() {
        for inn in args.iter() {
            if multiple >= 2 {
                return false;
            }

            if inn == out {
                multiple+=1;
            };
        }
    }
    true
}

pub fn flag_taker(args:&Vec<String>, flag:String) -> String {
    if one_single_target(args) {
        println!("single target")
    } else {
        println!("multiple target");
    }

    let mut open_name = false;
    let mut target  = String::new();
    for arg in args.iter() {
        if *arg == flag {
            open_name = true;
            continue;
        }

        if open_name {
            target = arg.to_string();
            open_name = false;
        }
    }
    target
}

