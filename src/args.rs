use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
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
    #[clap(long)]
    pub branch: Option<String>,

    // color
    #[clap(long, default_value_t = String::from("auto"))]
    color: String,
}
