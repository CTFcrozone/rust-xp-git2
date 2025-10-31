use git2::{Repository, RepositoryInitOptions, Signature, Time};
use std::path::{Path, PathBuf};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const REPO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/repo1/");
const REMOTE_ORIGIN: &str = "https://github.com/CTFcrozone/xp-git2-test-repo.git";

fn main() -> Result<()> {
	let repo = Repository::open(REPO_PATH)?;
	let msg = "Test-commit-1";
	let sig = Signature::now("χ", "voidbyte@null.net")?;
	let mut index = repo.index()?;
	let tree_id = index.write_tree()?;
	let tree = repo.find_tree(tree_id)?;

	let branch = repo.find_branch("test-branch", git2::BranchType::Local)?;
	let branch_ref = branch.get().name().unwrap();

	let parent_commit = branch.get().peel_to_commit()?;

	let commit_id = repo.commit(Some(branch_ref), &sig, &sig, msg, &tree, &[&parent_commit])?;

	println!("Committed {} to branch test-branch", commit_id);

	Ok(())
}
