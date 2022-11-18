use crate::DKIMError;

/// A trait for entities that perform DNS resolution.
pub trait Lookup: Sync + Send {
    fn lookup_txt<'a>(&'a self, name: &'a str) -> Result<Vec<String>, DKIMError>;
}
