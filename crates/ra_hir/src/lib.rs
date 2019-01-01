//! HIR (previsouly known as descriptors) provides a high-level OO acess to Rust
//! code.
//!
//! The principal difference between HIR and syntax trees is that HIR is bound
//! to a particular crate instance. That is, it has cfg flags and features
//! applied. So, there relation between syntax and HIR is many-to-one.

macro_rules! ctry {
    ($expr:expr) => {
        match $expr {
            None => return Ok(None),
            Some(it) => it,
        }
    };
}

pub mod db;
#[cfg(test)]
mod mock;
mod query_definitions;
mod path;
mod arena;
pub mod source_binder;

mod ids;
mod macros;
mod name;
mod krate;
mod module;
mod function;
mod adt;
mod type_ref;
mod ty;

use crate::{
    db::HirDatabase,
    name::{AsName, KnownName},
    ids::{DefKind, SourceItemId, SourceFileItemId, SourceFileItems},
};

pub use self::{
    path::{Path, PathKind},
    name::Name,
    krate::Crate,
    ids::{HirFileId, DefId, DefLoc},
    macros::{MacroDef, MacroInput, MacroExpansion, MacroCallId, MacroCallLoc},
    module::{Module, ModuleId, Problem, nameres::{ItemMap, PerNs, Namespace}, ModuleScope, Resolution},
    function::{Function, FnScopes},
    adt::{Struct, Enum},
    ty::Ty,
};

pub use self::function::FnSignatureInfo;

pub enum Def {
    Module(Module),
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Item,
}
