use std::{error::Error, process::exit};

use clap::Parser;

use recap::Recap;
use serde::Deserialize;
use subprocess::{Exec, Redirection};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Also open the pr in a browser
    #[clap(long)]
    open: bool,

    /// Create the pr as a draft
    #[clap(long)]
    no_draft: bool,

    /// Don't create the pr if it doesn't exist yet
    #[clap(long)]
    no_create: bool,

    /// Watch the output
    #[clap(long, default_value_t = 1)]
    watch: u16,

    /// asdf
    #[clap(long, default_value_t = String::from("main"))]
    branch: String,
}

fn main() {
    let args = Args::parse();

    // run(["git", ""])
    println!("sup {:#?}", args);

    fetch_pr_info(args.branch);
}

fn run(cmd: String) -> String {
    println!("running {}", cmd);
    match Exec::shell(cmd).stdout(Redirection::Pipe).capture() {
        Err(_v) => exit(1),
        Ok(v) => v.stdout_str(),
    }
}

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(
    regex = r"@@@@@(?P<number>.*)@@@@@(?P<number_hash>.*)@@@@@(?P<url>.*)@@@@@(?P<state>.*)@@@@@(?P<state_long>.*)@@@@@(?P<pr_color>.*)@@@@@(?P<pr_color_long>.*)@@@@@(?P<title>.*)@@@@@(?P<body>.*)@@@@@(?P<base>.*)@@@@@(?P<base_sha>.*)@@@@@(?P<head>.*)@@@@@(?P<head_sha>.*)@@@@@(?P<merge_commit>.*)@@@@@(?P<author>.*)@@@@@(?P<assignees>.*)@@@@@(?P<reviewers>.*)@@@@@(?P<milestone_number>.*)@@@@@(?P<milestone_title>.*)@@@@@(?P<milestone_comments>.*)@@@@@(?P<milestone_comments_pretty>.*)@@@@@(?P<created_date>.*)@@@@@(?P<created_rel>.*)@@@@@(?P<created_ts>.*)@@@@@(?P<created>.*)@@@@@(?P<updated_date>.*)@@@@@(?P<updated_rel>.*)@@@@@(?P<updated_ts>.*)@@@@@(?P<updated>.*)@@@@@(?P<merged_date>.*)@@@@@(?P<merged_rel>.*)@@@@@(?P<merged_ts>.*)@@@@@(?P<merged>.*)@@@@@"
)]
struct PrInfo {
    /// pull request number
    number: String,
    /// pull request number prefixed with "#"
    number_hash: String,
    /// the URL of this pull request
    url: String,
    /// state ("open" or "closed")
    state: String,
    /// pull request state ("open", "draft", "merged", or "closed")
    state_long: String,
    /// set color to red or green, depending on state
    pr_color: String,
    /// set color according to pull request state
    pr_color_long: String,
    /// title
    title: String,
    /// body
    body: String,
    /// base branch
    base: String,
    /// base commit SHA
    base_sha: String,
    /// head branch
    head: String,
    /// head commit SHA
    head_sha: String,
    /// merge commit SHA
    merge_commit: String,
    /// login name of author
    author: String,
    /// comma-separated list of assignees
    assignees: String,
    /// comma-separated list of requested reviewers
    reviewers: String,
    /// milestone number
    milestone_number: String,
    /// milestone title
    milestone_title: String,
    /// number of comments
    milestone_comments: String,
    /// number of comments wrapped in parentheses, or blank string if zero.
    milestone_comments_pretty: String,
    /// created date-only (no time of day)
    created_date: String,
    /// created date, relative
    created_rel: String,
    /// created date, UNIX timestamp
    created_ts: String,
    /// created date, ISO 8601 format
    created: String,
    /// updated date-only (no time of day)
    updated_date: String,
    /// updated date, relative
    updated_rel: String,
    /// updated date, UNIX timestamp
    updated_ts: String,
    /// updated date, ISO 8601 format
    updated: String,
    /// merged date-only (no time of day)
    merged_date: String,
    /// merged date, relative
    merged_rel: String,
    /// merged date, UNIX timestamp
    merged_ts: String,
    /// merged date, ISO 8601 format
    merged: String,
}

/// fetch the pr info for the given branch
fn fetch_pr_info(branch: String) -> Result<PrInfo, Box<dyn Error>> {
    let sep = "@@@@@";
    let formatstr = [
        "%I", "%i", "%U", "%S", "%pS", "%sC", "%pC", "%t", "%b", "%B", "%sB", "%H", "%sH", "%sm",
        "%au", "%as", "%rs", "%Mn", "%Mt", "%NC", "%Nc", "%cD", "%cr", "%ct", "%cI", "%uD", "%ur",
        "%ut", "%uI", "%mD", "%mr", "%mt", "%mI",
    ]
    .join(sep);

    let cmd = format!("hub pr list -f '{sep}{formatstr}{sep}' -h '{branch}'");
    let output = run(cmd);
    let info: PrInfo = output.parse()?;
    println!("info {:#?}", info);
    return Ok(info);
}
