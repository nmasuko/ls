use std::fs;
use std::env;
use std::path;
fn traverse(file: &String) {
    let attr = fs::metadata(file);
    if let Ok(a) = attr {
        if a.is_dir() {
            if let Ok(entries) = fs::read_dir(file){
                for entry in entries {
                    let mut path_buf = path::PathBuf::from(file);
                    path_buf.push(entry.unwrap().file_name());
                    traverse(&path_buf.to_str().unwrap().to_string());
                }
            }
        }
        else{
        println!("{}", file);

        }
    } else {
        println!("not found");
    }
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let files = if args.len() == 1 {
        vec![".".to_string()]
    } else {
        args[1..].to_vec()
    };
    for file in &files {
        traverse(file);
    }
}
