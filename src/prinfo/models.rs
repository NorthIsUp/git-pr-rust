use std::{fmt, result, time::SystemTime};

use colored::{ColoredString, Colorize};
use log::debug;
use serde::{Deserialize, Serialize, Serializer};
use struct_field_names_as_array::FieldNamesAsArray;

/// fetch the pr info for the given branch
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckStatusState {
    Completed,  // The check suite or run has been completed.
    InProgress, //The check suite or run is in progress.
    Pending,    //The check suite or run is in pending state.
    Queued,     //The check suite or run has been queued.
    Requested,  //The check suite or run has been requested.
    Waiting,    //The check suite or run is in waiting state.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckConclusionState {
    //The possible states for a check suite or run conclusion.
    ActionRequired, // The check suite or run requires action.
    Cancelled,      // The check suite or run has been cancelled.
    Failure,        // The check suite or run has failed.
    Neutral,        // The check suite or run was neutral.
    Skipped,        // The check suite or run was skipped.
    Stale,          /* The check suite or run was marked stale by GitHub. Only GitHub can use
                     * this conclusion. */
    StartupFailure, // The check suite or run has failed at startup.
    Success,        // The check suite or run has succeeded.
    TimedOut,       // The check suite or run has timed out.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatusContextState {
    Error,    //Status is errored.
    Expected, //Status is expected.
    Failure,  //Status is failing.
    Pending,  //Status is pending.
    Success,  //Status is successful.
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
    pub additions: usize,
    pub deletions: usize,
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
pub struct Review {
    id: String,
    author: User,
    authorAssociation: String,
    body: String,
    submittedAt: String,
    includesCreatedEdit: bool,
    reactionGroups: Vec<String>,
    state: String,
}

fn error_as_none<'de, D>(deserializer: D) -> Result<Option<CheckConclusionState>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match CheckConclusionState::deserialize(deserializer) {
        Ok(result) => Ok(Some(result)),
        Err(e) => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "__typename")]
pub enum StatusCheck {
    CheckRun {
        completedAt: String,
        #[serde(deserialize_with = "error_as_none")]
        conclusion: Option<CheckConclusionState>,
        detailsUrl: String,
        name: String,
        startedAt: String,
        status: CheckStatusState,
        workflowName: String,
    },
    StatusContext {
        context: String,
        startedAt: String,
        state: StatusContextState,
        targetUrl: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
pub struct Comment {
    id: String,
    author: User,
    authorAssociation: String,
    body: String,
    createdAt: String,
    includesCreatedEdit: bool,
    isMinimized: bool,
    minimizedReason: String,
    reactionGroups: Vec<String>,
    url: String,
    viewerDidAuthor: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
pub struct PrInfo {
    #[serde(skip)]
    #[field_names_as_array(skip)]
    pub __createdAt: Option<SystemTime>,
    pub additions: u32,
    pub assignees: Vec<String>,
    pub author: User,
    pub baseRefName: String,
    pub body: String,
    pub changedFiles: u32,
    pub closed: bool,
    pub closedAt: Option<String>,
    pub comments: Vec<Comment>,
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
    pub labels: Vec<Label>,
    pub latestReviews: Vec<Review>,
    pub maintainerCanModify: bool,
    pub mergeCommit: Option<Node>,
    pub mergeStateStatus: String,
    pub mergeable: String,
    pub mergedAt: Option<String>,
    pub mergedBy: Option<User>,
    pub milestone: Option<String>,
    pub number: u32,
    pub potentialMergeCommit: Option<Node>,
    pub projectCards: Vec<String>,
    pub reactionGroups: Vec<String>,
    pub reviewDecision: String,
    pub reviewRequests: Vec<String>,
    pub reviews: Vec<Review>,
    pub state: String,
    pub statusCheckRollup: Vec<StatusCheck>,
    pub title: String,
    pub updatedAt: String,
    pub url: String,
}

impl Into<String> for File {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{path} {add}{del}",
            path = self.path,
            add = "+".repeat(self.additions),
            del = "-".repeat(self.deletions)
        )
    }
}

impl Into<String> for StatusCheck {
    fn into(self) -> String {
        self.to_string()
    }
}

impl CheckStatusState {
    pub fn is_complete(&self) -> bool {
        match self {
            CheckStatusState::Completed => true,
            _ => false,
        }
    }
}

impl StatusContextState {
    pub fn is_complete(&self) -> bool {
        match self {
            StatusContextState::Success => true,
            StatusContextState::Failure => true,
            StatusContextState::Error => true,
            _ => false,
        }
    }
}

impl StatusCheck {
    fn type_name(&self) -> &'static str {
        match self {
            StatusCheck::CheckRun { .. } => todo!(),
            StatusCheck::StatusContext { .. } => todo!(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            StatusCheck::CheckRun { name, .. } => name.clone(),
            StatusCheck::StatusContext { context, .. } => context.clone(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            StatusCheck::CheckRun { status, .. } => status.is_complete(),
            StatusCheck::StatusContext { state, .. } => state.is_complete(),
        }
    }

    pub fn short_status_string(&self) -> String {
        self.short_status_str().to_string()
    }
    pub fn short_status_str(&self) -> &str {
        if !self.is_complete() {
            return " .. "
        }

        match self {
            StatusCheck::CheckRun { conclusion, .. } => match conclusion {
                None => " .. ",
                Some(conclusion) => match conclusion {
                    CheckConclusionState::ActionRequired => "Fail",
                    CheckConclusionState::Cancelled => "Pass",
                    CheckConclusionState::Failure => "Fail",
                    CheckConclusionState::Neutral => "Pass",
                    CheckConclusionState::Skipped => "Skip",
                    CheckConclusionState::Stale => "Fail",
                    CheckConclusionState::StartupFailure => "Fail",
                    CheckConclusionState::Success => " OK ",
                    CheckConclusionState::TimedOut => "Fail",
                    _ => panic!("unexpected status context state: {:?}", conclusion),
                },
            },
            StatusCheck::StatusContext { state, .. } => match state {
                StatusContextState::Error => "Fail",
                StatusContextState::Failure => "Fail",
                StatusContextState::Success => "Pass",
                _ => panic!("unexpected status context state: {:?}", state),
            },
        }
    }
    pub fn short_status_string_with_color(&self) -> ColoredString {
        let status = self.short_status_str();
        match status {
            " OK " => status.to_string().green(),
            "Pass" => status.to_string().white(),
            "Fail" => status.to_string().red(),
            "Skip" => status.to_string().yellow(),
            _ => status.to_string().white(),
        }
    }
}

impl fmt::Display for StatusCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{status}] {name}",
            status = self.short_status_string(),
            name = self.name()
        )
    }
}
