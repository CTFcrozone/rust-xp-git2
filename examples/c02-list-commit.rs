use git2::{Repository, RepositoryInitOptions, Sort};
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const REPO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/repo1/");
const REMOTE_ORIGIN: &str = "https://github.com/CTFcrozone/xp-git2-test-repo.git";

fn main() -> Result<()> {
	let repo = Repository::open(REPO_PATH)?;

	// get for head
	// let head = repo.head()?;
	// let commit = head.peel_to_commit()?;

	// get for specific remote branch
	// let branch = repo.find_branch("origin/test-branch", git2::BranchType::Remote)?;
	// let commit = branch.get().peel_to_commit()?;

	// get for specific local branch
	let branch = repo.find_branch("test-branch", git2::BranchType::Local)?;
	let commit = branch.get().peel_to_commit()?;

	let mut revwalk = repo.revwalk()?;

	revwalk.push(commit.id())?;
	revwalk.set_sorting(Sort::TIME)?;

	for commit_id in revwalk {
		let commit_id = commit_id?;
		let commit = repo.find_commit(commit_id)?;
		println!(
			"->> COMMIT_ID: {} AUTHOR: {} SUMMARY: \"{}\"",
			commit_id,
			commit.author().name().unwrap_or("NONE"),
			commit.summary().unwrap_or("NONE")
		)
	}

	Ok(())
}
