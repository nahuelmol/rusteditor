

pub fn check_type_target(flags:Vec<String>) -> &str {
    for flg in flags.iter(){
        if flg == "-f" {
            return "file";
        }
        if flg == "-p" {
            return "project";
        }
        if flg == "-c" {
            return "carpet";
        }

    }
    
}
