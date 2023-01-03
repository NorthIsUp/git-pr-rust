use std::error::Error;

use log::info;

pub mod cli;
mod git_commands;
mod prinfo;
mod shell;

use clap::Parser;
use simple_logger::SimpleLogger;

use crate::{
    git_commands::{current_branch_name, current_repo, get_main_branch},
    prinfo::PrInfo,
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

/// do the actual work
// fn run() -> Result<(), Box<dyn Error>> {
//     let _args = Args::parse();
//     let repo = current_repo();
//     let branch = current_branch_name(&repo).ok_or("no branch found")?;

//     // disallow pr's against main branch
//     let main_branch = get_main_branch(&repo)?;
//     let main_branch_name = main_branch.name()?.expect("need a branch bro");
//     if ["master", "main"].contains(&&branch[..]) || branch == main_branch_name {
//         panic!("can't pr against {}", branch)
//     }

//     info!("--> checking for branch {:#?}", branch);

//     let _ = PrInfo::new(&repo, false);
//     let pr_info = PrInfo::get(branch)
//         .or_else(|| PrInfo::new(&repo, false))
//         .ok_or("no pr info found")?;

//     println!("{}", pr_info.to_string());

//     Ok(())
// }

#[tokio::main]

async fn main() {
    let logger = SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    cli::main().await;
    // std::process::exit(match run() {
    //     Ok(_) => 0,
    //     Err(err) => {
    //         eprintln!("error: {:?}", err);
    //         1
    //     }
    // });
}
