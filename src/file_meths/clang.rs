use std::io;
use std::fs;
use std::io::Write;
use std::io::ErrorKind;

fn seed_makefile(){
    /*
     * makef_cnf is "makefile content"
     */
    let _makef_cnt = r#"
        OUT = out/out
        OUTTEST = tests/out

        compile:
            g++ -o $(OUT) main.cpp
        
        run:
            out/out.exe

        test:
            g++ -o $(OUTTEST) tests/test.cpp

        runtest:
            tests/out.exe
        "#;
    match fs::metadata("Makefile"){
        Ok(_) => {
            let file = fs::OpenOptions::new()
                .append(true)
                .open("Makefile");
            
            file.unwrap()
                .write_all(b"some content")
                .expect("error witing makefile");
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("Makefile not found"),
                ErrorKind::PermissionDenied => println!("denied"),
                _ => println!("unknown err"),
            }
        },
    }


}

fn add_ui(){
    /*
     * --ih_content -> interface.h content:
     * --icpp_content -> interface.cpp content;
     * 
     */
    let icpp_content:&str = r#"
        #include "utils.cpp"

        LRESULT CALLBACK wnd_proc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam ){
            switch(msg){
                case WM_CLOSE:
                case WM_DESTROY:{
                } break;
                case WM_SIZE:{
                } break;
                case WM_PAINT:{
                } break;
                default : {
                    return DefWindowProc(hwnd, msg, wparam, lparam);
                }
            }
            return DefWindowProc(hwnd, msg, wparam, lparam);
        }
         
        int WINAPI WinApi(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow){
            HWND hwnd;
            MSG msg;

            WINDCLASS wclass = {};
            wclass.style = CS_HREDRAW | CS_VREDRAM;
            wclass.lpfnWndProc = wnd_proc;
            wclass.plszClassName = "Window Class Name";

            if(!RegisterClass(&wclass)){
                MessageBox(NULL, "Window Registration Failed", "Error!",
                    MB_ICONEXCLAMATION | MB_OK);
                return 0;
            }

            hwnd = CreateWindow(
                    wc_n.lpszClassName,
                    "Adventura",
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    CW_USEDEFAULT, CW_USEDEFAULT, 980, 480,
                    NULL, NULL, GetModuleHandle(NULL), NULL);
            if(hwnd == NULL){
                MessageBox(NULL, "Window Creation Failed! TOOL", "Error!",
                MB_ICONEXCLAMATION | MB_OK);
                return 0;
            }

            ShowWindo(hwnd, nCmdShow);
            UpdateWindow(hwnd);

            while(PeekMessage(&msg, hwdn, 0, 0, PM_REMOVE)){
                TranslateMessage(&message);
                DispatchMessage(&message);
            }

            return 0;
        }
        "#;
    let ih_content:&str = r#"
        #define COLOR_RED 0xff0000
        #define COLOR_BLUE 0x0000ff
        #define COLOR_GREEN 0x00ff00
        #define COLOR_CYAN 0x00ffff
        #define COLOR_ORANGE 0xff8000
        #define COLOR_GRAY 0x808080
        #define COLOR_DARKGRAY 0x404040
        #define COLOR_YELLOW 0xffff00
        #define COLOR_BLACK 0x000000
        #define COLOR_WHITE 0xffffff
        #define COLOR_LILE 0x7611c3

        #include "interface.cpp"
        "#;
    let dir:&str = "UI";
    let files = vec!["interface.h", "interface.cpp"];  
    for file in files.iter() {
        let path:String = format!("{}/{}", dir, file);
        let mut content:String = String::new();
        if file.to_string() == "interface.h" {
            content = ih_content.to_string();
        } else if file.to_string() == "interface.cpp" {
            content = icpp_content.to_string();
        }

        match fs::write(path, content) {
            Ok(_) => println!("{} created", file),
            Err(_) => println!("Error writing {}", file),
        }
    }
}
fn add_db() {}
fn seed_main_file() {}

pub fn cpp_project(){

    match fs::write("Makefile", ""){
        Ok(_) => seed_makefile(),
        Err(e)=> eprintln!("Error writing the makefile: {}", e),
    }

    match fs::write("main.cpp", ""){
        Ok(_) => seed_main_file(),
        Err(e)=> eprintln!("Error writing the main file: {}", e),
    }

    println!("do your want a windows interface? [y/n]");
    let mut project_with_ui = String::new();
    io::stdin()
        .read_line(&mut project_with_ui)
        .expect("error entering UI response");
    
    println!("do you want to add a db access point [y/n]");
    let mut project_with_db = String::new();
    io::stdin()
        .read_line(&mut project_with_db)
        .expect("error add db");

    if project_with_ui == "y"{
        add_ui();
    }
    
    if project_with_db == "y" {
        add_db();
    }
}
