use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use base64::decode;
use erlang_term::Term;
use std::process::Stdio;
use std::fs::File;
use std::io::Write;
use serde_json::Value;

// i stole this wholesale from tokio command's documentation and made like two changes

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("mitmdump");
    cmd.arg("-q");
    cmd.arg("-s mitm_discord.py");
    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        let decoded = decode(&line)?.clone();
        let term = Term::from_bytes(&decoded).unwrap();
        let json = serde_json::from_str::<Value>(&serde_json::to_string(&term).unwrap()).unwrap();
        println!("{}", json);
    }

    Ok(())
}
