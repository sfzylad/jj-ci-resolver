use anyhow::{Ok, Result};
use std::process::Stdio;
use tokio::{process::Command, sync::mpsc::Sender};

#[derive(Debug)]
pub enum Status {
    Pending(String),
    Success(String),
    Failure(String),
    Unknown(String),
}

pub async fn get_ci_status(
    remote: String,
    git_refs: Vec<String>,
    tx: Sender<Status>,
    // ) -> Result<Status> {
) -> Result<()> {
    println!("Running: get_ci_status()");
    let mut tasks = Vec::with_capacity(git_refs.len());

    for git_ref in &git_refs {
        tasks.push(tokio::spawn({
            let args = vec![
                "ci",
                "-R",
                remote.trim(),
                "get",
                "--branch",
                git_ref.trim(),
                "--output",
                "json",
            ];
            let arg = format!(
                "ci -R {} get --branch {} --output json",
                remote.trim(),
                git_ref.trim()
            );
            println!("{:#?}", arg);
            let mut gitlab_cmd = Command::new("glab")
                .args(args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Couldn't run glab command");

            // TODO: Parse this json properly or talk directly to gitlab
            let jq_stdin: Stdio = gitlab_cmd
                .stdout
                .take()
                .unwrap()
                .try_into()
                .expect("faild to convert to Stdio");

            let jq_cmd = Command::new("jq")
                .args(vec!["-r", ".status"])
                .stdin(jq_stdin)
                .stdout(Stdio::piped())
                .spawn()
                .expect("jq failed");
            jq_cmd.wait_with_output()
        }));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    let gr = git_refs.to_vec();
    for (i, o) in outputs.into_iter().enumerate() {
        let output = o.expect("cannot wait for jq");
        let result = str::from_utf8(&output.stdout).expect("cannot read jq output");

        println!("R: {:#?}", result.trim());

        match result.trim() {
            "success" => {
                let _ = tx.send(Status::Success(gr[i].clone())).await;
            }
            "failed" => {
                let _ = tx.send(Status::Failure(gr[i].clone())).await;
            }
            "running" => {
                let _ = tx.send(Status::Pending(gr[i].clone())).await;
            }
            _ => {
                eprintln!("RESULT: {}", result);
                let _ = tx.send(Status::Unknown(gr[i].clone())).await;
            }
        };
    }

    Ok(())
}
