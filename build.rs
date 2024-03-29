use http_req::{self, request};

use std::{env, path::Path};
use std::{
    fs::File,
    io::Write,
    process::{self, Command},
};
// Example custom build script.
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir);
    let mut x = Command::new("git")
        .arg("clone")
        .current_dir(&out_dir)
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
        .current_dir(out_dir.clone())
        .args(
            "-c --exclude-from=bewmc/.gitignore --exclude .git -zvf bewmc.tar.gz bewmc".split(' '),
        )
        .spawn()
        .unwrap_or_else(|_e| panic!("Error: tar failed."));
    y.wait().unwrap();
    let mut h = Command::new("rm")
        .arg("-rf")
        .current_dir(out_dir.clone())
        .arg("bewmc")
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Error: rm failed.");
            eprintln!("{:?}", err);
            process::exit(1)
        });

    h.wait().unwrap();
    let mut bbox = File::create(Path::new(&out_dir.clone()).join("busybox.exe")).unwrap();
    let mut buf = vec![];
    request::get("http://frippery.org/files/busybox/busybox.exe", &mut buf)
        .map_err(|e| e.to_string())
        .unwrap();
    bbox.write_all(&buf).unwrap();
    // let mut g = Command::new("wget")
    //     .arg("-N")
    //     .arg("'https://frippery.org/files/busybox/busybox.exe'")
    //     .arg("-O")
    //     .arg("busybox.exe")
    //     .spawn()
    //     .unwrap_or_else(|err| {
    //         eprintln!("Error: wget failed.");
    //         eprintln!("{:?}", err);
    //         process::exit(1)
    //     });
    // g.wait().unwrap();
}
