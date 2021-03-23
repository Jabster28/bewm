use std::process::Command;

fn main() {
    println!("BEWM!!!");
    // #[cfg(feature = "reverse_shell")]
    #[cfg(unix)]
    println!("yay unix");

    #[cfg(windows)]
    println!("eww windows");
}
