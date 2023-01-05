use git2::{Branch, Repository};

pub fn current_repo() -> Repository {
    return match Repository::init(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}

pub fn remote_gh_name(repo: &Repository) -> String {
    let found_remote = repo.find_remote("origin").unwrap();
    let url = found_remote.url().unwrap();
    let remote_name = &url[url.find("/").unwrap() + 1..url.find(".git").unwrap()];
    return remote_name.to_string();
}

pub fn current_branch(repo: &Repository) -> Option<Branch> {
    let head = repo.head().ok()?;
    let name = head.shorthand()?;
    return repo.find_branch(name, git2::BranchType::Local).ok();
}

pub fn current_branch_name(repo: &Repository) -> Option<String> {
    return current_branch(repo)?
        .name()
        .map_or(None, |s| Some(String::from(s?)));
}

pub fn get_main_branch(repo: &Repository) -> Result<Branch, &'static str> {
    for branch in ["main", "master"] {
        match repo.find_branch(branch, git2::BranchType::Local) {
            Ok(b) => return Ok(b),
            Err(_) => continue,
        }
    }
    Err("no branch found")
}

pub fn get_merge_base(repo: &Repository, main_branch: &Branch) -> git2::Oid {
    let head_oid = repo.head().unwrap().target().unwrap();
    let main_oid = main_branch.get().target().unwrap();
    repo.merge_base(head_oid, main_oid).unwrap()
}
