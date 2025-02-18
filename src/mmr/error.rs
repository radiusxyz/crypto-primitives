pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    InvalidArgument,
    GetRootOnEmpty,
    InconsistentStore,
    StoreError(crate::ark_std::string::String),
    /// proof items is not enough to build a tree
    CorruptedProof,
    /// The leaves is an empty list, or beyond the mmr range
    GenProofForInvalidLeaves,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::*;
        match self {
            InvalidArgument => write!(f, "Invalid Argument")?,
            GetRootOnEmpty => write!(f, "Get root on an empty MMR")?,
            InconsistentStore => write!(f, "Inconsistent store")?,
            StoreError(msg) => write!(f, "Store error {}", msg)?,
            CorruptedProof => write!(f, "Corrupted proof")?,
            GenProofForInvalidLeaves => write!(f, "Generate proof ofr invalid leaves")?,
        }
        Ok(())
    }
}

