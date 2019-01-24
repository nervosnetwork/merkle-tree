pub mod merkle_tree;

/// A trait for creating parent node.
pub trait Merge {
    /// Returns parent node of two nodes
    fn merge(left: &Self, right: &Self) -> Self;
}

#[cfg(feature = "sha3")]
pub use numext_fixed_hash::H256;

#[cfg(feature = "blake2b")]
pub use numext_fixed_hash::H256;

#[cfg(feature = "sha3")]
impl Merge for H256 {
    fn merge(left: &H256, right: &H256) -> H256 {
        let mut hash = [0u8; 32];
        let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
        sha3.update(left.as_bytes());
        sha3.update(right.as_bytes());
        sha3.finalize(&mut hash);
        hash.into()
    }
}

#[cfg(feature = "blake2b")]
impl Merge for H256 {
    fn merge(left: &H256, right: &H256) -> H256 {
        let mut ret = [0u8; 32];
        let mut context = blake2_rfc::blake2b::Blake2b::new(64);
        context.update(left.as_bytes());
        context.update(right.as_bytes());
        let hash = context.finalize();
        ret.copy_from_slice(&hash.as_bytes()[0..32]);
        ret.into()
    }
}

pub use crate::merkle_tree::new_cbmt;
