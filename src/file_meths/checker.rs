

pub fn check_type_target<'apple>(flags:&Vec<String>) -> &'apple str {

    if flags.len() > 0 {
        for flg in flags.iter(){
            if flg == "-f" {
                return "file";
            }
            else if flg == "-p" {
                return "project";
            }
            else if flg == "-c" {
                return "carpet";
            }
        }

        ""
    }
    else {
        "non-flags"
    }
}
