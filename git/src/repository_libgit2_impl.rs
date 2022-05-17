use std::path::Path;
use chrono::{FixedOffset, NaiveDateTime, TimeZone};
use git2;
use crate::error::{Error, ErrorClass, ErrorCode};
use crate::repository::{Blame, BlameHunk, Repository, Sha1, Signature};

impl From<git2::ErrorCode> for ErrorCode {
    fn from(code: git2::ErrorCode) -> Self {
        match code {
            git2::ErrorCode::GenericError => ErrorCode::GenericError,
            git2::ErrorCode::NotFound => ErrorCode::NotFound,
            git2::ErrorCode::Exists => ErrorCode::Exists,
            git2::ErrorCode::Ambiguous => ErrorCode::Ambiguous,
            git2::ErrorCode::BufSize => ErrorCode::BufSize,
            git2::ErrorCode::User => ErrorCode::User,
            git2::ErrorCode::BareRepo => ErrorCode::BareRepo,
            git2::ErrorCode::UnbornBranch => ErrorCode::UnbornBranch,
            git2::ErrorCode::Unmerged => ErrorCode::Unmerged,
            git2::ErrorCode::NotFastForward => ErrorCode::NotFastForward,
            git2::ErrorCode::InvalidSpec => ErrorCode::InvalidSpec,
            git2::ErrorCode::Conflict => ErrorCode::Conflict,
            git2::ErrorCode::Locked => ErrorCode::Locked,
            git2::ErrorCode::Modified => ErrorCode::Modified,
            git2::ErrorCode::Auth => ErrorCode::Auth,
            git2::ErrorCode::Certificate => ErrorCode::Certificate,
            git2::ErrorCode::Applied => ErrorCode::Applied,
            git2::ErrorCode::Peel => ErrorCode::Peel,
            git2::ErrorCode::Eof => ErrorCode::Eof,
            git2::ErrorCode::Invalid => ErrorCode::Invalid,
            git2::ErrorCode::Uncommitted => ErrorCode::Uncommitted,
            git2::ErrorCode::Directory => ErrorCode::Directory,
            git2::ErrorCode::MergeConflict => ErrorCode::MergeConflict,
            git2::ErrorCode::HashsumMismatch => ErrorCode::HashsumMismatch,
            git2::ErrorCode::IndexDirty => ErrorCode::IndexDirty,
            git2::ErrorCode::ApplyFail => ErrorCode::ApplyFail,
        }
    }
}

impl From<git2::ErrorClass> for ErrorClass {
    fn from(class: git2::ErrorClass) -> Self {
        match class {
            git2::ErrorClass::None => ErrorClass::None,
            git2::ErrorClass::NoMemory => ErrorClass::NoMemory,
            git2::ErrorClass::Os => ErrorClass::Os,
            git2::ErrorClass::Invalid => ErrorClass::Invalid,
            git2::ErrorClass::Reference => ErrorClass::Reference,
            git2::ErrorClass::Zlib => ErrorClass::Zlib,
            git2::ErrorClass::Repository => ErrorClass::Repository,
            git2::ErrorClass::Config => ErrorClass::Config,
            git2::ErrorClass::Regex => ErrorClass::Regex,
            git2::ErrorClass::Odb => ErrorClass::Odb,
            git2::ErrorClass::Index => ErrorClass::Index,
            git2::ErrorClass::Object => ErrorClass::Object,
            git2::ErrorClass::Net => ErrorClass::Net,
            git2::ErrorClass::Tag => ErrorClass::Tag,
            git2::ErrorClass::Tree => ErrorClass::Tree,
            git2::ErrorClass::Indexer => ErrorClass::Indexer,
            git2::ErrorClass::Ssl => ErrorClass::Ssl,
            git2::ErrorClass::Submodule => ErrorClass::Submodule,
            git2::ErrorClass::Thread => ErrorClass::Thread,
            git2::ErrorClass::Stash => ErrorClass::Stash,
            git2::ErrorClass::Checkout => ErrorClass::Checkout,
            git2::ErrorClass::FetchHead => ErrorClass::FetchHead,
            git2::ErrorClass::Merge => ErrorClass::Merge,
            git2::ErrorClass::Ssh => ErrorClass::Ssh,
            git2::ErrorClass::Filter => ErrorClass::Filter,
            git2::ErrorClass::Revert => ErrorClass::Revert,
            git2::ErrorClass::Callback => ErrorClass::Callback,
            git2::ErrorClass::CherryPick => ErrorClass::CherryPick,
            git2::ErrorClass::Describe => ErrorClass::Describe,
            git2::ErrorClass::Rebase => ErrorClass::Rebase,
            git2::ErrorClass::Filesystem => ErrorClass::Filesystem,
            git2::ErrorClass::Patch => ErrorClass::Patch,
            git2::ErrorClass::Worktree => ErrorClass::Worktree,
            git2::ErrorClass::Sha1 => ErrorClass::Sha1,
            git2::ErrorClass::Http => ErrorClass::Http,
        }
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Self {
            code: err.code().into(),
            class: err.class().into(),
            message: err.message().to_owned(),
        }
    }
}

impl From<git2::Oid> for Sha1 {
    fn from(oid: git2::Oid) -> Self {
        let mut bytes = [0; 20];
        bytes.copy_from_slice(oid.as_bytes());
        Sha1(bytes)
    }
}

impl From<&Sha1> for git2::Oid {
    fn from(sha1: &Sha1) -> Self {
        git2::Oid::from_bytes(&sha1.0).unwrap()
    }
}

impl From<git2::Signature<'_>> for Signature {
    fn from(lib_signature: git2::Signature) -> Self {
        Self {
            name: lib_signature.name().unwrap_or_default().to_owned(),
            email: lib_signature.email().unwrap_or_default().to_owned(),
            when: FixedOffset::east(lib_signature.when().offset_minutes())
                .from_utc_datetime(&NaiveDateTime::from_timestamp(lib_signature.when().seconds(), 0)),
        }
    }
}

impl Repository for git2::Repository {
    fn commit(&self, message: &str, tree_id: &Sha1) -> Result<Sha1, Error> {
        let sig = self.signature()?;
        let tree = self.find_tree(tree_id.into())?;
        Ok(self.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?.into())
    }

    fn blame(&self, path: &Path) -> Result<Blame, Error> {
        let hunks: Vec<BlameHunk> = self.blame_file(path, None)?
            .iter()
            .map(|lib_hunk| {
                BlameHunk {
                    start_line: lib_hunk.final_start_line(),
                    lines_count: lib_hunk.lines_in_hunk(),
                    signature: lib_hunk.final_signature().into(),
                    commit_id: lib_hunk.final_commit_id().into(),
                }
            })
            .collect();
        Ok(Blame { hunks })
    }
}

pub fn open_repo<P: AsRef<Path>>(path: P) -> Result<Box<dyn Repository>, Error> {
    Ok(Box::new(git2::Repository::open(path)?))
}