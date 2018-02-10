// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An efficient hash map for node IDs

#![allow(non_snake_case)]

use hir::def_id::DefId;
use hir::{HirId, ItemLocalId};
use syntax::ast;

pub(crate) use rustc_data_structures::fx::FxHashMap;
pub(crate) use rustc_data_structures::fx::FxHashSet;

pub(crate) type NodeMap<T> = FxHashMap<ast::NodeId, T>;
pub(crate) type DefIdMap<T> = FxHashMap<DefId, T>;
pub(crate) type HirIdMap<T> = FxHashMap<HirId, T>;
pub(crate) type ItemLocalMap<T> = FxHashMap<ItemLocalId, T>;

pub(crate) type NodeSet = FxHashSet<ast::NodeId>;
pub(crate) type DefIdSet = FxHashSet<DefId>;
pub(crate) type HirIdSet = FxHashSet<HirId>;
pub(crate) type ItemLocalSet = FxHashSet<ItemLocalId>;

pub(crate) fn NodeMap<T>() -> NodeMap<T> { FxHashMap() }
pub(crate) fn DefIdMap<T>() -> DefIdMap<T> { FxHashMap() }
pub(crate) fn ItemLocalMap<T>() -> ItemLocalMap<T> { FxHashMap() }
pub(crate) fn NodeSet() -> NodeSet { FxHashSet() }
pub(crate) fn DefIdSet() -> DefIdSet { FxHashSet() }
pub(crate) fn ItemLocalSet() -> ItemLocalSet { FxHashSet() }

