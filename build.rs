use std::process::{self, Command};

// Example custom build script.
fn main() {
    let mut x = Command::new("git")
        .arg("clone")
        .arg("https://github.com/Jabster28/bewmc/")
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Error: git failed.");
            eprintln!("{:?}", err);
            process::exit(1)
        });
    x.wait().unwrap();
    println!("Adding copy of BEWMC...");
    let mut y = Command::new("tar")
        .args(
            "-c --exclude-from=bewmc/.gitignore --exclude .git -zvf bewmc.tar.gz bewmc".split(' '),
        )
        .spawn()
        .unwrap_or_else(|_e| panic!("Error: tar failed."));
    y.wait().unwrap();
    let mut h = Command::new("rm")
        .arg("-rf")
        .arg("bewmc")
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Error: rm failed.");
            eprintln!("{:?}", err);
            process::exit(1)
        });
    h.wait().unwrap();
    let mut g = None;
    if cfg!(unix) {
        println!("unix yay");
        g = Some(
            Command::new("wget")
                .arg("-N")
                .arg("'https://frippery.org/files/busybox/busybox.exe'")
                .arg("-O")
                .arg("busybox.exe")
                .spawn()
                .unwrap_or_else(|err| {
                    eprintln!("Error: wget failed.");
                    eprintln!("{:?}", err);
                    process::exit(1)
                }),
        );
    } else {
        g = Some(
            Command::new("curl.exe")
                .arg("-o")
                .arg("busybox.exe")
                .arg("'https://frippery.org/files/busybox/busybox.exe'")
                .spawn()
                .unwrap_or_else(|err| {
                    eprintln!("Error: curl failed.");
                    eprintln!("{:?}", err);
                    process::exit(1)
                }),
        );
    }
    g.unwrap().wait().unwrap();
}
