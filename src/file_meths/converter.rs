use crate::CliCommand;
use crate::file_meths::utils::{ flag_taker };

pub fn convert(command:&CliCommand){
    let mut input: bool = false;
    let mut output:bool = false;
 
    for flg in command.flags.iter(){
   
        if flg == "-inp" {
            input = true;
        }

        if flg == "-oup" {
            output = true;
        }
    };

    if input && output {
        let target = flag_taker(&command.flags,"-t".to_string());
        let inputf = flag_taker(&command.flags,"-inp".to_string());
        let outputf= flag_taker(&command.flags,"-oup".to_string());
        parser(target, inputf, outputf);
    }
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
