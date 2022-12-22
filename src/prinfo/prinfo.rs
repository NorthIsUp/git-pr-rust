use std::time::SystemTime;

use git2::Repository;
use indoc::formatdoc;
use log::{debug, info};
use serde_json::from_str;

use crate::{
    git_commands::{current_branch_name, get_main_branch, get_merge_base},
    prinfo::models::PrInfo,
    shell,
};

fn mocks(s: &str) -> String {
    match s {
        "fix-main/1" => include_str!("d1.json").to_string(),
        "simple" => include_str!("d2.json").to_string(),
        _ => panic!("unknown test case"),
    }
}

pub fn map_to_string<S: Into<String>>(vec: Vec<S>) -> String where {
    vec.into_iter()
        .map(|s| s.into())
        .collect::<Vec<String>>()
        .join("\n")
}

impl PrInfo {
    pub fn to_string(&self) -> String {
        return formatdoc!(
            "
                ====> #{number_hash} — {title}
                {body}
                > {url}
                ----> Summary
                {files}
                ----> Details
                author        --> {author}
                created at    --> {created_at}
                updated at    --> {updated_at}
                state         --> {state}
                sha           --> {sha}
                url           --> {url}
                ----> Checks
                {checks}
            ",
            number_hash = self.number,
            title = self.title,
            url = self.url,
            body = self.body,
            files = map_to_string(self.files.clone()),
            author = self.author.login,
            created_at = self.createdAt,
            updated_at = self.updatedAt,
            state = self.state,
            sha = self.sha(),
            checks = map_to_string(self.statusCheckRollup.clone()),
        )
    }

    pub fn sha(&self) -> String {
        return self.commits.last().cloned().unwrap().oid
    }

    pub fn is_complete(&self) -> bool {
        self.statusCheckRollup.iter().all(|s| s.is_complete())
    }

    /// fetch the pr info from github via their api
    pub fn get<S: Into<String>>(branch: S) -> Option<PrInfo> {
        // todo: migrate to the gh structured format
        let branch = branch.into();
        let format_str = PrInfo::FIELD_NAMES_AS_ARRAY.join(",");
        let cmd = format!("gh pr list --json {format_str} -H {branch}");

        let stdout = match shell::run(cmd).ok() {
            None => None,
            Some(s) if s.stdout.is_empty() => None,
            Some(s) => Some(s.stdout_str()),
        };
        // .map_or(None, |capture| match capture.stdout_str() {
        //     s if s.is_empty() => None,
        //     s => Some(s),
        // });
        // let stdout = Some(mocks("fix-main/1"));
        debug!("{:?}", stdout.clone()?);
        let pr_info = match from_str::<[PrInfo; 1]>(&stdout?) {
            Ok([pr_info]) => PrInfo {
                __createdAt: Some(SystemTime::now()),
                ..pr_info
            },
            Err(_) => return None,
        };
        return Some(pr_info)
    }

    /// use the gh cli tool to create a pr
    pub fn create(repo: &Repository, draft: bool) -> Option<PrInfo> {
        let _draft_arg = if draft { "--draft" } else { "" };
        let _title = "";
        let _body = "";

        let current_branch_name = current_branch_name(repo).expect("must have current branch name");
        info!("pushing remote origin {:?}", current_branch_name);
        let _result = repo
            .find_remote("origin")
            .and_then(|mut remote| remote.push(&[current_branch_name.clone()], None));

        let main_branch = get_main_branch(repo).ok()?;
        let merge_base = get_merge_base(repo, &main_branch);
        let merge_base_commit = repo.find_commit(merge_base).ok()?;

        let (title, body) = merge_base_commit.message()?.split_once('\n')?;

        let draft_arg = match draft {
            true => "--draft",
            false => "",
        };

        let _result = shell::run(format!(
            "gh pr create --title='{title}' --body='{body}' {draft_arg} "
        ));
        PrInfo::get(current_branch_name.clone())
    }

    pub fn update(&mut self) -> Self {
        if SystemTime::now()
            .duration_since(self.__createdAt.unwrap())
            .unwrap()
            .as_secs()
            >= 15
        {
            let pr_info = PrInfo::get(&self.headRefName).expect("must have new info");
            pr_info.clone_into(self);

            // info!("updated pr info");
        } else {
            // info!("cached pr info");
        }
        self.clone()
    }
}
