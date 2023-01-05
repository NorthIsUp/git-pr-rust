use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant, SystemTime},
};

use clap::Parser;
use colored::Colorize;
use console::Emoji;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use log::debug;

use crate::{
    args::Args,
    git_commands::{current_branch_name, current_repo},
    prinfo::PrInfo,
};
struct App {
    args: Args,
    branch: String,
    mp: MultiProgress,
    progress_bars: Arc<Mutex<HashMap<String, ProgressBar>>>,
}
#[derive(Debug, Default, Clone)]
struct Pb {
    key: String,
    prefix: String,
    message: String,
    template: String,
    tick_chars: String,
    indent: usize,
}

impl Pb {
    fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            ..Default::default()
        }
    }
    fn new_with_pkey<S: Into<String>>(key: S) -> Self {
        let key = key.into();
        Self::new(&key).with_prefix(&key)
    }
    fn new_with_pkey_and_message<S1: Into<String>, S2: Into<String>>(key: S1, message: S2) -> Self {
        let key = key.into();
        Self::new(&key).with_prefix(&key).with_message(message)
    }
    fn new_header<S: Into<String>>(key: S) -> Self {
        Self::new_with_pkey(key).as_header()
    }
    fn new_section<S: Into<String>>(key: S) -> Self {
        Self::new_with_pkey(key).as_section()
    }
    fn with_prefix<S: Into<String>>(&mut self, prefix: S) -> Self {
        self.prefix = prefix.into();
        self.clone()
    }
    fn with_message<S: Into<String>>(&mut self, message: S) -> Self {
        self.message = message.into();
        self.clone()
    }
    fn with_template<S: Into<String>>(&mut self, template: S) -> Self {
        self.template = template.into();
        self.clone()
    }
    fn with_tick_chars<S: Into<String>>(&mut self, tick_chars: S) -> Self {
        self.tick_chars = tick_chars.into();
        self.clone()
    }
    fn with_indent(&mut self, indent: usize) -> Self {
        self.indent = indent;
        self.clone()
    }
    fn as_header(&mut self) -> Self {
        self.with_template(format!(
            "{} {}",
            "====>".magenta(),
            "{prefix:.white.bold}{msg:.white.bold}"
        ))
    }
    fn as_section(&mut self) -> Self {
        self.with_template(format!(
            "{} {}",
            "---->".magenta(),
            "{prefix:.white.bold}{msg:.white.bold}"
        ))
    }
}

impl App {
    fn new() -> Self {
        let args = Args::parse();
        let branch = match &args.branch {
            Some(b) => b.to_string(),
            None => {
                let repo = current_repo();
                current_branch_name(&repo).expect("must have a branch name")
            }
        };

        Self {
            args,
            branch,
            mp: MultiProgress::new(),
            progress_bars: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn pb(&mut self, pb_args: &Pb) -> ProgressBar {
        let mut progress_bars = self.progress_bars.lock().unwrap();
        if !progress_bars.contains_key(&pb_args.key) {
            progress_bars.insert(pb_args.key.clone(), ProgressBar::new(100));
            self.mp.add(progress_bars[&pb_args.key].clone());
        }

        let pb = progress_bars.get(&pb_args.key).unwrap();
        if !pb_args.message.is_empty() {
            pb.set_message(pb_args.message.clone());
        }

        if !pb_args.prefix.is_empty() {
            pb.set_prefix(pb_args.prefix.clone());
        }

        let mut template = match [!pb_args.template.is_empty(), !pb_args.prefix.is_empty()] {
            [true, _] => pb_args.template.clone(),
            [false, true] => format!(
                "{} {} {}",
                "{prefix:10.white}",
                "-->".magenta(),
                "{wide_msg}"
            ),
            [false, false] => "{wide_msg}".to_string(),
        };

        if pb_args.indent > 0 {
            template = format!("{}{}", " ".repeat(pb_args.indent), template);
        }

        let tick_chars = match pb_args.tick_chars.is_empty() {
            true => "⣾⣽⣻⢿⡿⣟⣯⣷",
            false => pb_args.tick_chars.as_str(),
        };

        pb.set_style(
            ProgressStyle::with_template(template.as_str())
                .expect("ok")
                .tick_chars(tick_chars),
        );
        return progress_bars[&pb_args.key].clone()
    }

    fn get_progress_bars(&mut self, pr_info: &PrInfo) -> Vec<ProgressBar> {
        let mut pb_keys: Vec<Pb> = vec![];

        pb_keys.extend([
            Pb::new_header("header")
                .with_prefix(format!("#{} - {}", pr_info.number, pr_info.title)),
            Pb::new_with_pkey_and_message("url", &pr_info.url)
                .with_template("> {msg}")
                .with_indent(4),
            Pb::new_section("body"),
            Pb::new("_body").with_message(&pr_info.body),
        ]);

        if !pr_info.files.is_empty() {
            pb_keys.push(Pb::new_section("files"));
            let files = pr_info.files.clone();
            let longest_file = files
                .iter()
                .max_by(|a, b| a.path.len().cmp(&b.path.len()))
                .unwrap()
                .path
                .len();
            pb_keys.extend(files.iter().map(|f| {
                Pb::new_with_pkey_and_message(
                    f.path.as_str(),
                    format!("{}{}{}{}", f.additions, "+".green(), f.deletions, "-".red()),
                )
                .with_template(format!(
                    "  {} {{prefix:{longest_file}}} {} {{msg}}",
                    "-->".magenta(),
                    "|".magenta(),
                ))
            }));
        }

        pb_keys.extend([
            Pb::new_section("details"),
            Pb::new_with_pkey_and_message("state", &pr_info.state),
            Pb::new_with_pkey_and_message("author", &pr_info.author.login),
            Pb::new_with_pkey_and_message("createdAt", &pr_info.createdAt),
            Pb::new_with_pkey_and_message("updatedAt", &pr_info.updatedAt),
            Pb::new_with_pkey_and_message("state", &pr_info.state),
            Pb::new_with_pkey_and_message("sha", pr_info.sha()),
            Pb::new_with_pkey_and_message("url", pr_info.url.clone()),
        ]);

        if !pr_info.statusCheckRollup.is_empty() {
            pb_keys.push(Pb::new_section("checks"));
            pb_keys.extend(pr_info.statusCheckRollup.iter().map(|sc| {
                let spinner = if sc.is_complete() { " " } else { " {spinner} " };
                Pb::new_with_pkey_and_message(
                    sc.name(),
                    format!("[{}]", sc.short_status_string_with_color()),
                )
                .with_template(format!("{{msg}}{spinner}{{prefix:.bold.dim}}"))
            }));
        }

        let pbs = pb_keys
            .iter()
            .map(|pb_args| self.pb(pb_args))
            .collect::<Vec<ProgressBar>>();
        return pbs
    }

    async fn run_loop(&mut self) {
        let start = SystemTime::now();
        let pr_info = Arc::new(Mutex::new(
            PrInfo::get(self.branch.clone()).expect("must have pr info"),
        ));

        loop {
            let pr_info = pr_info.clone();
            if pr_info.lock().unwrap().is_complete() {
                break
            }

            self.get_progress_bars(&pr_info.lock().unwrap())
                .iter()
                .for_each(|pb| {
                    pb.inc(1);
                });

            tokio::spawn(async move {
                pr_info.lock().unwrap().update();
            });

            sleep(Duration::from_millis(75));
        }

        self.get_progress_bars(&pr_info.lock().unwrap())
            .iter()
            .for_each(|pb| {
                pb.finish();
            });
    }
}

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub(crate) async fn main() -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let mut app = App::new();

    app.run_loop().await;

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}
