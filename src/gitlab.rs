use anyhow::{Error, Ok, Result};
use serde_json::Value;
use std::process::Stdio;
use tokio::{process::Command, sync::mpsc::Sender};

#[derive(Debug)]
pub enum Status {
    Pending(String),
    Success(String),
    Failure(String),
    Canceled(String),
    Unknown(String),
}

pub async fn get_ci_status(
    remote: String,
    git_refs: Vec<String>,
    tx: Sender<Status>,
    // ) -> Result<Status> {
) -> Result<(), Error> {
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
            println!("{:#?}", args.join(" "));
            let gitlab_cmd = Command::new("glab")
                .args(args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Couldn't run glab command");
            gitlab_cmd.wait_with_output()
        }));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    for (i, o) in outputs.into_iter().enumerate() {
        let output = o.expect("cannot wait for glab");
        let result = str::from_utf8(&output.stdout).expect("cannot read glab output");
        let v: Value = serde_json::from_str(result)?;

        match v["status"].as_str().unwrap() {
            "success" => {
                let _ = tx.send(Status::Success(git_refs[i].clone())).await;
            }
            "failed" => {
                let _ = tx.send(Status::Failure(git_refs[i].clone())).await;
            }
            "running" => {
                let _ = tx.send(Status::Pending(git_refs[i].clone())).await;
            }
            "canceled" => {
                let _ = tx.send(Status::Canceled(git_refs[i].clone())).await;
            }
            _ => {
                let _ = tx.send(Status::Unknown(git_refs[i].clone())).await;
            }
        };
    }

    Ok(())
}
