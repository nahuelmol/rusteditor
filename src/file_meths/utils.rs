
use crate::presentations::pres;

pub fn one_single_target(&args) -> bool {
    let possible_tgs:Vec<String> = vec!["-f", "-exp", "-docker", "-wasm", "-cpp"];

    let mut counter:usize = 0;//counter fot the inner loop
    let mut outcter:usize = 0;//counter for the outer loop
    let mut multiple:usize= 0;
    'outter :loop {
        possible_tgs[counter];
        'inner :loop {
            if multiple >= 2 {
                return false;
            }
            
            if args[outter] == possible_tgs[counter]{
                multiple+=1;
            };
            outcter+=1;
        }
        counter+=1;
    }
    true
}


pub fn tool_presentation(command:String){
    if command == "new".to_sring() {
        new_presentation();
    } else if command == "delete".to_string() {
        del_presentation();
    } else if command == "edit".to_string() {
        edt_presentation();
    } else {
        gen_presentation();
    }
}

pub fn flag_taker(args:&Vec<String>, flag:String) -> String {
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

