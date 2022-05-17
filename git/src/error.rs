/// An enumeration of possible errors that can happen when working with a git
/// repository.
// Note: We omit a few native error codes, as they are unlikely to be propagated
// to the library user. Currently:
//
// * GIT_EPASSTHROUGH
// * GIT_ITEROVER
// * GIT_RETRY
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    /// Generic error
    GenericError,
    /// Requested object could not be found
    NotFound,
    /// Object exists preventing operation
    Exists,
    /// More than one object matches
    Ambiguous,
    /// Output buffer too short to hold data
    BufSize,
    /// User-generated error
    User,
    /// Operation not allowed on bare repository
    BareRepo,
    /// HEAD refers to branch with no commits
    UnbornBranch,
    /// Merge in progress prevented operation
    Unmerged,
    /// Reference was not fast-forwardable
    NotFastForward,
    /// Name/ref spec was not in a valid format
    InvalidSpec,
    /// Checkout conflicts prevented operation
    Conflict,
    /// Lock file prevented operation
    Locked,
    /// Reference value does not match expected
    Modified,
    /// Authentication error
    Auth,
    /// Server certificate is invalid
    Certificate,
    /// Patch/merge has already been applied
    Applied,
    /// The requested peel operation is not possible
    Peel,
    /// Unexpected EOF
    Eof,
    /// Invalid operation or input
    Invalid,
    /// Uncommitted changes in index prevented operation
    Uncommitted,
    /// Operation was not valid for a directory
    Directory,
    /// A merge conflict exists and cannot continue
    MergeConflict,
    /// Hashsum mismatch in object
    HashsumMismatch,
    /// Unsaved changes in the index would be overwritten
    IndexDirty,
    /// Patch application failed
    ApplyFail,
}

/// An enumeration of possible categories of things that can have
/// errors when working with a git repository.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorClass {
    /// Uncategorized
    None,
    /// Out of memory or insufficient allocated space
    NoMemory,
    /// Syscall or standard system library error
    Os,
    /// Invalid input
    Invalid,
    /// Error resolving or manipulating a reference
    Reference,
    /// ZLib failure
    Zlib,
    /// Bad repository state
    Repository,
    /// Bad configuration
    Config,
    /// Regex failure
    Regex,
    /// Bad object
    Odb,
    /// Invalid index data
    Index,
    /// Error creating or obtaining an object
    Object,
    /// Network error
    Net,
    /// Error manpulating a tag
    Tag,
    /// Invalid value in tree
    Tree,
    /// Hashing or packing error
    Indexer,
    /// Error from SSL
    Ssl,
    /// Error involing submodules
    Submodule,
    /// Threading error
    Thread,
    /// Error manipulating a stash
    Stash,
    /// Checkout failure
    Checkout,
    /// Invalid FETCH_HEAD
    FetchHead,
    /// Merge failure
    Merge,
    /// SSH failure
    Ssh,
    /// Error manipulating filters
    Filter,
    /// Error reverting commit
    Revert,
    /// Error from a user callback
    Callback,
    /// Error cherry-picking commit
    CherryPick,
    /// Can't describe object
    Describe,
    /// Error during rebase
    Rebase,
    /// Filesystem-related error
    Filesystem,
    /// Invalid patch data
    Patch,
    /// Error involving worktrees
    Worktree,
    /// Hash library error or SHA-1 collision
    Sha1,
    /// HTTP error
    Http,
}

pub struct Error {
    pub code: ErrorCode,
    pub class: ErrorClass,
    pub message: String,
}