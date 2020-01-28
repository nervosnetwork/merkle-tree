#![cfg_attr(not(feature = "std"), no_std)]

pub mod merkle_tree;

pub use crate::merkle_tree::{MerkleProof, MerkleTree, CBMT};

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections;
        use std::vec;
    } else {
        extern crate alloc;
        use alloc::collections;
        use alloc::vec;
    }
}
