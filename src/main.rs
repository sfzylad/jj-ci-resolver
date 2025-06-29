mod gitlab;
mod revsets;

use gitlab::Status;
use tokio::sync::mpsc;

use jj_lib::config::ConfigSource;

use anyhow::{Error, Result};
use clap::Parser;
use revsets::Revsets;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Utility for JJ to fetch status of the git refs from Gitlab"
)]
struct Args {
    /// Path to the file to be modified.
    #[arg(short, long)]
    file: String,
    /// Git refs to obtain information about. Can be used multiple times.
    #[arg(short, long, value_parser, num_args = 1..)]
    git_refs: Vec<String>,
    /// Git remote to connect to.
    #[arg(short, long)]
    remote: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let (tx, mut rx) = mpsc::channel::<Status>(10);

    let mut rs = Revsets::new(ConfigSource::User, args.file.into())?;
    let mut ci_success: Vec<String> = vec![];
    let mut ci_failures: Vec<String> = vec![];
    let mut ci_pending: Vec<String> = vec![];

    let remote = args.remote.clone();

    tokio::spawn(async move {
        let _ = gitlab::get_ci_status(remote.clone(), args.git_refs, tx).await;
    });

    while let Some(result) = rx.recv().await {
        match result {
            gitlab::Status::Pending(id) => {
                println!("{}: pending!", id);
                ci_pending.push(id);
            }
            gitlab::Status::Success(id) => {
                println!("success!");
                ci_success.push(id);
            }
            gitlab::Status::Failure(id) => {
                println!("failure!");
                ci_failures.push(id);
            }
            gitlab::Status::Unknown(id) => {
                eprintln!("{}: unknown state", id);
            }
        };
    }

    println!("{:?}", ci_success);

    if !ci_failures.is_empty() {
        rs.set_ci_failures(ci_failures)?;
    }
    if !ci_success.is_empty() {
        rs.set_ci_success(ci_success)?;
    }
    if !ci_pending.is_empty() {
        rs.set_ci_pending(ci_pending)?;
    }

    Ok(())
}
