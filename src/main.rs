use std::error::Error;

use crate::{
    git_commands::current_branch_name,
    prinfo::{create_pr, fetch_pr_info, format_pr_info},
};

mod git_commands;
mod prinfo;
mod shell;

use {
    crate::git_commands::{current_repo, get_main_branch},
    clap::Parser,
};

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

    // color
    #[clap(long, default_value_t = String::from("auto"))]
    color: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let _args = Args::parse();
    let repo = current_repo();
    let branch = current_branch_name(&repo).ok_or("xno branch found")?;

    // disallow pr's against main branch
    let main_branch = get_main_branch(&repo)?;
    let main_branch_name = main_branch.name()?.expect("need a branch bro");
    if ["master", "master"].contains(&&branch[..]) || branch == main_branch_name {
        panic!("can't pr against {}", branch)
    }

    print!("--> checking for branch {:#?}", branch);

    let _ = create_pr(&repo, false);
    let pr_info = fetch_pr_info(branch)
        .or_else(|| create_pr(&repo, false))
        .ok_or("no pr info found")?;

    println!("{}", format_pr_info(pr_info));

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
