use git2::{Repository, RepositoryInitOptions};
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const REPO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/repo1/");
const REMOTE_ORIGIN: &str = "https://github.com/CTFcrozone/xp-git2-test-repo.git";

fn main() -> Result<()> {
	let repo = Repository::open(REPO_PATH)?;

	let mut remote = repo
		.find_remote(REMOTE_ORIGIN)
		.or_else(|_| repo.remote_anonymous(REMOTE_ORIGIN))?;

	let conn = remote.connect_auth(git2::Direction::Fetch, None, None)?;

	for head in conn.list()?.iter() {
		println!("{}\t{}", head.oid(), head.name());
	}

	Ok(())
}
