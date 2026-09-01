// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Internal tree node types backing the collection types.
//!
//! The [`btree`] (backing `OrdMap`/`OrdSet`) and [`hamt`] (backing
//! `HashMap`/`HashSet`) modules are public; they are exposed for advanced
//! read-only use and come with no stability guarantees.

/// The B+Tree nodes backing `OrdMap` and `OrdSet`.
pub mod btree;
/// The hash array mapped trie nodes backing `HashMap` and `HashSet`.
pub mod hamt;
pub(crate) mod rrb;

pub(crate) mod chunk {
    pub(crate) use crate::config::VECTOR_CHUNK_SIZE as CHUNK_SIZE;
    use imbl_sized_chunks as sc;

    pub(crate) type Chunk<A> = sc::sized_chunk::Chunk<A, CHUNK_SIZE>;
}
