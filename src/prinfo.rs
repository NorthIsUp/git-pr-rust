use git2::Repository;
use indoc::formatdoc;
use recap::Recap;
use serde::Deserialize;

use crate::{
    git_commands::{current_branch_name, get_main_branch, get_merge_base},
    shell,
};

/// fetch the pr info for the given branch
const _SEP: &str = "@@@@@@";

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(
    regex = r"@@@@@@(?P<number>.*)@@@@@@(?P<number_hash>.*)@@@@@@(?P<url>.*)@@@@@@(?P<state>.*)@@@@@@(?P<state_long>.*)@@@@@@(?P<pr_color>.*)@@@@@@(?P<pr_color_long>.*)@@@@@@(?P<title>.*)@@@@@@(?P<body>.*)@@@@@@(?P<base>.*)@@@@@@(?P<base_sha>.*)@@@@@@(?P<head>.*)@@@@@@(?P<head_sha>.*)@@@@@@(?P<merge_commit>.*)@@@@@@(?P<author>.*)@@@@@@(?P<assignees>.*)@@@@@@(?P<reviewers>.*)@@@@@@(?P<milestone_number>.*)@@@@@@(?P<milestone_title>.*)@@@@@@(?P<milestone_comments>.*)@@@@@@(?P<milestone_comments_pretty>.*)@@@@@@(?P<created_date>.*)@@@@@@(?P<created_rel>.*)@@@@@@(?P<created_ts>.*)@@@@@@(?P<created>.*)@@@@@@(?P<updated_date>.*)@@@@@@(?P<updated_rel>.*)@@@@@@(?P<updated_ts>.*)@@@@@@(?P<updated>.*)@@@@@@(?P<merged_date>.*)@@@@@@(?P<merged_rel>.*)@@@@@@(?P<merged_ts>.*)@@@@@@(?P<merged>.*)@@@@@@"
)]
pub struct PrInfo {
    /// pull request number
    pub number: String,
    /// pull request number prefixed with "#"
    pub number_hash: String,
    /// the URL of this pull request
    pub url: String,
    /// state ("open" or "closed")
    pub state: String,
    /// pull request state ("open", "draft", "merged", or "closed")
    pub state_long: String,
    /// set color to red or green, depending on state
    pub pr_color: String,
    /// set color according to pull request state
    pub pr_color_long: String,
    /// title
    pub title: String,
    /// body
    pub body: String,
    /// base branch
    pub base: String,
    /// base commit SHA
    pub base_sha: String,
    /// head branch
    pub head: String,
    /// head commit SHA
    pub head_sha: String,
    /// merge commit SHA
    pub merge_commit: String,
    /// login name of author
    pub author: String,
    /// comma-separated list of assignees
    pub assignees: String,
    /// comma-separated list of requested reviewers
    pub reviewers: String,
    /// milestone number
    pub milestone_number: String,
    /// milestone title
    pub milestone_title: String,
    /// number of comments
    pub milestone_comments: String,
    /// number of comments wrapped in parentheses, or blank string if zero.
    pub milestone_comments_pretty: String,
    /// created date-only (no time of day)
    pub created_date: String,
    /// created date, relative
    pub created_rel: String,
    /// created date, UNIX timestamp
    pub created_ts: String,
    /// created date, ISO 8601 format
    pub created: String,
    /// updated date-only (no time of day)
    pub updated_date: String,
    /// updated date, relative
    pub updated_rel: String,
    /// updated date, UNIX timestamp
    pub updated_ts: String,
    /// updated date, ISO 8601 format
    pub updated: String,
    /// merged date-only (no time of day)
    pub merged_date: String,
    /// merged date, relative
    pub merged_rel: String,
    /// merged date, UNIX timestamp
    pub merged_ts: String,
    /// merged date, ISO 8601 format
    pub merged: String,
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
            ",
            number_hash = self.number_hash,
            title = self.title,
            url = self.url,
            body = self.body,
            author = self.author,
            created_at = self.created_rel,
            updated_at = self.updated_rel,
            state = self.state,
            sha = self.head_sha,
        );
    }
}

/// fetch the pr info from github via their api
pub fn fetch_pr_info<S: Into<String>>(branch: S) -> Option<PrInfo> {
    // todo: migrate to the gh structured format
    // gh pr list --json
    // additions,assignees,author,baseRefName,body,changedFiles,closed,closedAt,comments,commits,
    // createdAt,deletions,files,headRefName,headRepository,headRepositoryOwner,id,
    // isCrossRepository,isDraft,labels,latestReviews,maintainerCanModify,mergeCommit,
    // mergeStateStatus,mergeable,mergedAt,mergedBy,milestone,number,potentialMergeCommit,
    // projectCards,reactionGroups,reviewDecision,reviewRequests,reviews,state,statusCheckRollup,
    // title,updatedAt,url
    let branch = branch.into();
    let format_str = [
        "%I", "%i", "%U", "%S", "%pS", "%sC", "%pC", "%t", "%b", "%B", "%sB", "%H", "%sH", "%sm",
        "%au", "%as", "%rs", "%Mn", "%Mt", "%NC", "%Nc", "%cD", "%cr", "%ct", "%cI", "%uD", "%ur",
        "%ut", "%uI", "%mD", "%mr", "%mt", "%mI",
    ]
    .join(_SEP);

    shell::run(format!(
        "gh pr list -f '@@@@@@{format_str}@@@@@@' -h '{branch}'"
    ))
    .map_or(None, |capture| match capture.stdout_str() {
        s if s.is_empty() => None,
        s => s.parse::<PrInfo>().ok(),
    })
}

/// use the gh cli tool to create a pr
pub fn create_pr(repo: &Repository, draft: bool) -> Option<PrInfo> {
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
    None
}
