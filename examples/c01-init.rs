use git2::{Repository, RepositoryInitOptions};
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

const REPO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/repo1/");
const REMOTE_ORIGIN: &str = "https://github.com/CTFcrozone/xp-git2-test-repo.git";

fn main() -> Result<()> {
	let path = PathBuf::from(REPO_PATH);

	let mut opts = RepositoryInitOptions::new();

	opts.mkpath(true).no_reinit(true).origin_url(REMOTE_ORIGIN);

	Repository::init_opts(&path, &opts)?;

	Ok(())
}
