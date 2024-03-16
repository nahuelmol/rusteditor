use crate::Command;

pub fn convert(command:&Command){
    for flg in command.flags.iter(){
        let mut input: bool = false;
        let mut output:bool = false;

        let mut in_format = String::new();
        let mut out_format = String::new();

        if input {
            in_format = flg.to_string();
            println!("the input format is {}", in_format);
        } else if flg == "-inp" {
            input = true;
        }

        if output {
            out_format = flg.to_string();
            println!("the output format is {}", out_format);
        } else if flg == "-out" {
            output= true; 
        }
    };
}


fn parser(content:String, inputf:String, outputf:String){
    let avail_formats:Vec<&str> = vec![".json", ".dat", ".txt", ".xml", ".csv"];
    let mut input_right:bool = false;
    let mut output_right:bool= false;

    for format in avail_formats.iter() {
        if format.to_string() == inputf {
            input_right = true;
        }

        if format.to_string() == outputf {
            output_right= true;
        }
    }

    if input_right && output_right {
        println!("convertsion is possible");
        println!("content: {}", content);
    }
}
