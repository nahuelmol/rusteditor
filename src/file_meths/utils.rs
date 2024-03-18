pub fn tool_presentation(){
    println!("you need to study to understand this tool")
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

