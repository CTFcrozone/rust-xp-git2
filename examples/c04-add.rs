use git2::{Repository, RepositoryInitOptions};
use std::path::{Path, PathBuf};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const REPO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/repo1/");
const REMOTE_ORIGIN: &str = "https://github.com/CTFcrozone/xp-git2-test-repo.git";

fn main() -> Result<()> {
	let repo = Repository::open(REPO_PATH)?;

	let mut index = repo.index()?;

	let paths: &[&str] = &["commit_test.txt"];

	for path in paths {
		index.add_path(Path::new(path))?;
	}

	index.write()?;

	Ok(())
}
