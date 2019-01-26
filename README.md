# Merkle Tree for Static Data

[![Build status](https://ci.appveyor.com/api/projects/status/smv1jr8mrbf5a8is?svg=true)](https://ci.appveyor.com/project/doitian/merkle-tree)
[![Build Status](https://travis-ci.com/nervosnetwork/merkle-tree.svg?branch=master)](https://travis-ci.com/nervosnetwork/merkle-tree)

## Complete Binary Merkle Tree

Complete Binary Merkle Tree(CBMT) can be used to to generate *Merkle Root*  and *Merkle Proof* for a static list of items. Currently, CBMT is used to calculate *Transactions Root*. Basically, CBMT is a ***complete binary tree***, in which every level, except possibly the last, is completely filled, and all nodes are as far left as possible. And it is also a ***full binary tree***, in which every node other than the leaves has two children. Compare with other Merkle trees, the hash computation of CBMT is minimal, as well as the proof size.

## Nodes Organization

For the sake of illustration, we order the tree nodes from ***top to bottom*** and ***left to right*** starting at zero. In CBMT with *n* items, root is the *first* node, and the first item's hash is *node 0*, second is *node n+1*, etc. We choose this nodes organization because it is easy to calculate the node order for an item.

For example, CBMT with 6 items(suppose the hashes are `[T0, T1, T2, T3, T4, T5]`) and CBMT with 7 items(suppose the hashes are `[T0, T1, T2, T3, T4, T5, T6]`) is shown below:

```
        with 6 items                       with 7 items

              B0 -- node 0                       B0 -- node 0
             /  \                               /  \
           /      \                           /      \
         /          \                       /          \
       /              \                   /              \
      B1 -- node 1    B2 -- node 2       B1 -- node 1    B2 -- node 2
     /  \            /  \               /  \            /  \
    /    \          /    \             /    \          /    \
   /      \        /      \           /      \        /      \
  B3(3)   B4(4)  TO(5)    T1(6)      B3(3)   B4(4)   B5(5)   T0(6)
 /  \    /  \                       /  \    /  \    /  \
T2  T3  T4  T5                     T1  T2  T3  T4  T5  T6
(7) (8) (9) (10)                   (7) (8) (9)(10)(11) (12)
```

Specially, the tree with 0 item is empty(0 node) and its root is `T::default()`.

## Tree Struct

CBMT can be represented in a very space-efficient way, using an array alone. Nodes in the array are presented in ascending order.

For example, the two trees above can be represented as:

```
// an array with 11 elements, the first element is node 0(BO), second is node 1, etc.
[B0, B1, B2, B3, B4, T0, T1, T2, T3, T4, T5]

// an array with 13 elements, the first element is node 0(BO), second is node 1, etc.
[B0, B1, B2, B3, B4, B5, T0, T1, T2, T3, T4, T5, T6]
```

Suppose a CBMT with *n* items, the size of the array would be *2n-1*, the index of item i(start at 0) is *i+n-1*. For node at *i*, the index of its parent is *(i-1)/2*, the index of its sibling is *(i+1)^1-1*(*^* is xor) and the indexes of its children are *[2i+1, 2i+2]*.

## Merkle Proof

Merkle Proof can provide a proof for existence of one or more items. Only sibling of the nodes along the path that form leaves to root, excluding the nodes already in the path, should be included in the proof. We also specify that ***the nodes in the proof is presented in descending order***(with this, algorithms of proof's generation and verification could be much simple). Indexes of item that need to prove are essential to complete the root calculation, since the index is not the inner feature of item, so the indexes are also included in the proof, and in order to get the correct correspondence, we specify that the indexes are ***presented in ascending order by corresponding hash***. For example, if we want to show that `[T1, T4]` is in the list of 6 items above, only nodes `[T5, T0, B3]` and indexes `[9, 6]` should be included in the proof.

## Usage

You can use any type to generate a Merkle Tree, if `Merge`, `Ord` and `Default` is implemented for it. like:

```
use merkle_tree::CBMT;

impl Merge for i32 {
    fn merge(left: &Self, right: &Self) -> Self {
        right.wrapping_sub(*left)
    }
}

let leaves = vec![2i32, 3, 5, 7, 11];
let indices = vec![0, 4];
let proof_leaves = vec![2i32, 11];
let root = CBMT::build_merkle_root(&leaves);
let proof = CBMT::build_merkle_proof(&leaves, &indices).unwrap();
let tree = CBMT::build_merkle_tree(leaves);
proof.verify(&proof_leaves, &root);
```
