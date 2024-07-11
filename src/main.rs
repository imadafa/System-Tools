//use std::io;
use sysinfo::{System, SystemExt};
use std::process::Command;


fn main() {
    let mut option = String::new();
    
    println!("\n[1] ACTIVATE WINDOWS\n[2] SYSTEM INFO\n");

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: u32 = option.trim().parse().expect("Please enter a valid number!");

    match option {
        1 => println!("You chose to ACTIVATE WINDOWS"),
        2 => {
            let mut sys = System::new_all();
            sys.refresh_all();

            println!("=> system:");
            // RAM and swap information:
            println!("total memory: {} bytes", sys.total_memory());
            println!("used memory : {} bytes", sys.used_memory());
            println!("total swap  : {} bytes", sys.total_swap());
            println!("used swap   : {} bytes", sys.used_swap());

            println!("System name:             {:?}", sys.name());
            println!("System kernel version:   {:?}", sys.kernel_version());
            println!("System OS version:       {:?}", sys.os_version());
            println!("System host name:        {:?}", sys.host_name());

            println!("NB CPUs: {}", sys.cpus().len());
            Command::new("ls")
                .arg("")
                .output();
        },
        _ => println!("Invalid option"),
    }
    
    use std::io::{self, Write};
    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

}
