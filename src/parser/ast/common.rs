use nom_locate::LocatedSpan;
use serde::Serialize;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Serialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeId(pub usize);

pub struct NodeIdGenerator(AtomicUsize);

impl NodeIdGenerator {
    pub fn new() -> Self {
        NodeIdGenerator(AtomicUsize::new(0))
    }

    pub fn next(&self) -> NodeId {
        NodeId(self.0.fetch_add(1, Ordering::SeqCst))
    }
}
