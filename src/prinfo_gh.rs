use git2::Repository;
use indoc::formatdoc;
use recap::Recap;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, from_str, Result};

use crate::{
    git_commands::{current_branch_name, get_main_branch, get_merge_base},
    shell,
};

/// fetch the pr info for the given branch
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum CheckStatusState {
    Completed,  // The check suite or run has been completed.
    InProgress, //The check suite or run is in progress.
    Pending,    //The check suite or run is in pending state.
    Queued,     //The check suite or run has been queued.
    Requested,  //The check suite or run has been requested.
    Waiting,    //The check suite or run is in waiting state.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum CheckConclusionState {
    //The possible states for a check suite or run conclusion.
    Action_Required, // The check suite or run requires action.
    Cancelled,       // The check suite or run has been cancelled.
    Failure,         // The check suite or run has failed.
    Neutral,         // The check suite or run was neutral.
    Skipped,         // The check suite or run was skipped.
    Stale,           /* The check suite or run was marked stale by GitHub. Only GitHub can use
                      * this conclusion. */
    Startup_Failure, // The check suite or run has failed at startup.
    Success,         // The check suite or run has succeeded.
    Timed_Out,       // The check suite or run has timed out.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub login: String,
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub authoredDate: String,
    pub authors: Vec<User>,
    pub committedDate: String,
    pub messageBody: String,
    pub messageHeadline: String,
    pub oid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub oid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusCheck {
    __typename: String,
    completedAt: String,
    conclusion: CheckConclusionState,
    detailsUrl: String,
    name: String,
    startedAt: String,
    status: CheckStatusState,
    workflowName: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrInfo {
    pub additions: u32,
    pub assignees: Vec<String>,
    pub author: User,
    pub baseRefName: String,
    pub body: String,
    pub changedFiles: u32,
    pub closed: bool,
    pub closedAt: Option<String>,
    pub comments: Vec<String>,
    pub commits: Vec<Commit>,
    pub createdAt: String,
    pub deletions: u32,
    pub files: Vec<File>,
    pub headRefName: String,
    pub headRefOid: String,
    pub headRepository: Repo,
    pub headRepositoryOwner: User,
    pub id: String,
    pub isCrossRepository: bool,
    pub isDraft: bool,
    pub labels: Vec<String>,
    pub latestReviews: Vec<String>,
    pub maintainerCanModify: bool,
    pub mergeCommit: Option<String>,
    pub mergeStateStatus: String,
    pub mergeable: String,
    pub mergedAt: Option<String>,
    pub mergedBy: Option<String>,
    pub milestone: Option<String>,
    pub number: u32,
    pub potentialMergeCommit: Option<Node>,
    pub projectCards: Vec<String>,
    pub reactionGroups: Vec<String>,
    pub reviewDecision: String,
    pub reviewRequests: Vec<String>,
    pub reviews: Vec<String>,
    pub state: String,
    pub statusCheckRollup: Vec<StatusCheck>,
    pub title: String,
    pub updatedAt: String,
    pub url: String,
}

impl PrInfo {
    pub fn to_string(&self) -> String {
        return formatdoc!(
            "
                ====> #{number_hash} — {title}
                > {url}
                ----> Summary
                {body}
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
            author = self.author.login,
            created_at = self.createdAt,
            updated_at = self.updatedAt,
            state = self.state,
            sha = self.sha(),
            checks = self
                .statusCheckRollup
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    pub fn sha(&self) -> String {
        return self.commits.last().cloned().unwrap().oid
    }

    /// fetch the pr info from github via their api
    pub fn get<S: Into<String>>(branch: S) -> Option<PrInfo> {
        // todo: migrate to the gh structured format
        let branch = branch.into();
        let format_str = [
            "additions",
            "assignees",
            "author",
            "baseRefName",
            "body",
            "changedFiles",
            "closed",
            "closedAt",
            "comments",
            "commits",
            "createdAt",
            "deletions",
            "files",
            "headRefName",
            "headRefOid",
            "headRepository",
            "headRepositoryOwner",
            "id",
            "isCrossRepository",
            "isDraft",
            "labels",
            "latestReviews",
            "maintainerCanModify",
            "mergeCommit",
            "mergeStateStatus",
            "mergeable",
            "mergedAt",
            "mergedBy",
            "milestone",
            "number",
            "potentialMergeCommit",
            "projectCards",
            "reactionGroups",
            "reviewDecision",
            "reviewRequests",
            "reviews",
            "state",
            "statusCheckRollup",
            "title",
            "updatedAt",
            "url",
        ]
        .join(",");

        let cmd = format!("gh pr list --json {format_str} -H {branch}");
        let stdout = shell::run(cmd).map_or(None, |capture| match capture.stdout_str() {
            s if s.is_empty() => None,
            s => Some(s),
        });
        // let stdout = Some(r#"
        //     [{"additions":13,"assignees":[],"author":{"login":"NorthIsUp"},"baseRefName":"main","body":"","changedFiles":2,"closed":false,"closedAt":null,"comments":[],"commits":[{"authoredDate":"2022-04-24T21:47:26Z","authors":[{"email":"adam@northisup.com","id":"MDQ6VXNlcjEyMjYxMg==","login":"NorthIsUp","name":"Adam Hitchcock"}],"committedDate":"2022-04-24T21:47:26Z","messageBody":"","messageHeadline":"[docs] add some docscrings","oid":"f3ed8881a602e1baefd4d9f247fd40b18adc21ab"},{"authoredDate":"2022-12-16T19:43:06Z","authors":[{"email":"adam@northisup.com","id":"MDQ6VXNlcjEyMjYxMg==","login":"NorthIsUp","name":"Adam Hitchcock"}],"committedDate":"2022-12-16T19:43:06Z","messageBody":"","messageHeadline":"stuff","oid":"eb842b2fca368e855a1a38feae80ec506c6ab868"}],"createdAt":"2022-12-16T19:43:15Z","deletions":13,"files":[{"path":"Cargo.toml","additions":3,"deletions":1},{"path":"src/main.rs","additions":10,"deletions":12}],"headRefName":"test3","headRepository":{"id":"R_kgDOHM-UbA","name":"git-pr-rust"},"headRepositoryOwner":{"id":"MDQ6VXNlcjEyMjYxMg==","name":"Adam Hitchcock","login":"NorthIsUp"},"id":"PR_kwDOHM-UbM5Fq6j2","isCrossRepository":false,"isDraft":false,"labels":[],"latestReviews":[],"maintainerCanModify":false,"mergeCommit":null,"mergeStateStatus":"CLEAN","mergeable":"MERGEABLE","mergedAt":null,"mergedBy":null,"milestone":null,"number":2,"potentialMergeCommit":{"oid":"6e3e26621a24dbdcbbcf4fb57abacbe48d4f145e"},"projectCards":[],"reactionGroups":[],"reviewDecision":"","reviewRequests":[],"reviews":[],"state":"OPEN","statusCheckRollup":[],"title":"stuff","updatedAt":"2022-12-16T19:43:15Z","url":"https://github.com/NorthIsUp/git-pr-rust/pull/2"}]
        //     "#.to_string()
        // );
        println!("{}", stdout.clone()?);
        let pr_info_response: [PrInfo; 1] = from_str(&stdout?).unwrap();
        return pr_info_response.first().cloned()
    }

    /// use the gh cli tool to create a pr
    pub fn new(repo: &Repository, draft: bool) -> Option<PrInfo> {
        let _draft_arg = if draft { "--draft" } else { "" };
        let _title = "";
        let _body = "";

        let current_branch_name = current_branch_name(repo).expect("must have current branch name");
        repo.find_remote("origin")
            .ok()?
            .push(&[current_branch_name], None);

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
        PrInfo::get(current_branch_name)
    }
}

impl StatusCheck {
    pub fn to_string(&self) -> String {
        let status = match self.status {
            CheckStatusState::COMPLETED => match self.conclusion {
                CheckConclusionState::ACTION_REQUIRED => "Fail",
                CheckConclusionState::CANCELLED => "Pass",
                CheckConclusionState::FAILURE => "Fail",
                CheckConclusionState::NEUTRAL => "Pass",
                CheckConclusionState::SKIPPED => "Skip",
                CheckConclusionState::STALE => "Fail",
                CheckConclusionState::STARTUP_FAILURE => "Fail",
                CheckConclusionState::SUCCESS => " OK ",
                CheckConclusionState::TIMED_OUT => "Fail",
            },
            _ => "..",
        };
        formatdoc!("[{status}] {name}", name = self.name.as_str())
    }
}
