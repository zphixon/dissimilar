use crate::{Chunk, Diff, Range};

#[cfg(feature = "use-serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum OwnedChunk {
    Equal(String),
    Delete(String),
    Insert(String),
}

impl From<&Chunk<'_>> for OwnedChunk {
    fn from(c: &Chunk<'_>) -> Self {
        match c {
            Chunk::Equal(s) => OwnedChunk::Equal(s.to_string()),
            Chunk::Delete(s) => OwnedChunk::Delete(s.to_string()),
            Chunk::Insert(s) => OwnedChunk::Insert(s.to_string()),
        }
    }
}

#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum OwnedDiff {
    Equal(OwnedRange, OwnedRange),
    Delete(OwnedRange),
    Insert(OwnedRange),
}

impl From<&Diff<'_, '_>> for OwnedDiff {
    fn from(c: &Diff<'_, '_>) -> Self {
        match c {
            Diff::Equal(s, r) => OwnedDiff::Equal(s.into(), r.into()),
            Diff::Delete(s) => OwnedDiff::Delete(s.into()),
            Diff::Insert(r) => OwnedDiff::Insert(r.into()),
        }
    }
}

#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct OwnedRange {
    pub doc: String,
    pub offset: usize,
    pub len: usize,
}

impl From<&Range<'_>> for OwnedRange {
    fn from(r: &Range<'_>) -> Self {
        OwnedRange {
            doc: r.doc.to_owned(),
            offset: r.offset,
            len: r.len,
        }
    }
}
