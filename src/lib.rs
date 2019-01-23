pub mod merkle_tree;

/// A trait for creating parent node.
pub trait Merge {
    /// Returns parent node of two nodes
    fn merge(left: &Self, right: &Self) -> Self;
}
