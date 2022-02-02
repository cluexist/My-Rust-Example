use std::{env, fs, process};
use std::io::{self, Write};

use traverse_cargo::*;

mod file_info;
use file_info::*;

fn main() {

    print!("\x1B[2J\x1B[1;1H");

    let mut path = user_input();
    while path != "exit"
    {
        let path_meta = match fs::metadata(&path){
            Ok(v) => v,
            Err(error) => {
                println!("WARNING! {:?}",error.to_string());
                println!("WARNING! Please Re-Enter PATH...");
                println!("=====================================================================");
                path = user_input();
                continue;
            }
        };

        if path_meta.is_dir(){
            let list = file_info::get_file_list(&path).unwrap();

            for l in list{
                let p :file_info::FilePermission = file_info::get_file_perm(l.to_string()).unwrap();
                let p_s = p.print();
                println!("{}", p_s);
                println!("=====================================================================");
            }
        }
        path = user_input();
    }

}

fn user_input() -> String {


    println!("Welcome... ");
    print!("Input path you wanna search... >> ");
    io::stdout().flush().unwrap();


    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to readline");
    path = path.replace("\n", "");

    if path != "exit"{
        println!("Searching \"{}\" ...", path);
        println!("=====================================================================");
    } else {
        println!("Exit Program ...");
    }

    path
}
