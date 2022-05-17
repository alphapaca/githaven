use std::path::Path;
use crate::error::Error;
use chrono::{DateTime, FixedOffset};

pub struct Sha1(pub [u8; 20]);

pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: DateTime<FixedOffset>,
}

pub struct BlameHunk {
    pub start_line: usize,
    pub lines_count: usize,
    pub signature: Signature,
    pub commit_id: Sha1,
}

pub struct Blame {
    pub hunks: Vec<BlameHunk>,
}

// needed handles: commit, blame, diff, init --bare, open
pub trait Repository {
    fn commit(&self, message: &str, tree_id: &Sha1) -> Result<Sha1, Error>;

    fn blame(&self, path: &Path) -> Result<Blame, Error>;
}