pub fn tool_presentation(command:String){
    if command == "new".to_string() {
        new_presentation();
    } else if command == "delete".to_string() {
        del_presentation();
    } else if command == "edit".to_string() {
        edt_presentation();
    } else {
        gen_presentation();
    }
}

pub fn new_presentation(){
    let message = r#"
        new stands for creating new projects

        lee new -[f or p] -[typeproject]            [projectname]

        "#;
    println!("{}",message);
}
pub fn edt_presentation(){
    let message = r#"
        edt stands for editing file or projects
        edition

        lee edt -[filetype] -n                      [filename] 
        lee edt -[editiontype] -[projecttype]       [projectname] 
        
        again if you don't use -p or -f, lee inferes that you are editing a file
        if you insert a project, lee will require the type of edition
        
        [editiontype]
            -pname                  change name project
            -pversion               change version
            -reintalld              reinstalls dependencies
        "#;
    println!("{}", message)

}
pub fn del_presentation(){
    let message = r#"
        del stands for delete

        lee del -wasm           [projectname]
        lee del -exp            [projectname]
        lee del -docker         [projectname] 
        lee del -cpp            [projectname]

        lee del -[f or p] -[typeproject]            [projectname]
        

        if you enter a typeproject lee inferes that it is project
        if you don't enter a typeproject lee knows it is file
            then the use of -f or -p is redundant

        for cleaning the current directory from any kid of existent project
        
        lee del all
        
        in the case of file deletion

        lee del [filename] 
        "#;

    println!("{}", message);
}

pub fn gen_presentation(){
    let message = r#"
        the general commands can be devided into 

        creation-deletion
        edition

    "#;
    println!("{}", message);

    new_presentation();
    del_presentation();
    edt_presentation();
}
