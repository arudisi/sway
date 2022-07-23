//! The main handle to an IR instance.
//!
//! [`Context`] contains several
//! [generational_arena](https://github.com/fitzgen/generational-arena) collections to maintain the
//! IR ECS.
//!
//! It is passed around as a mutable reference to many of the Sway-IR APIs.

use std::collections::HashMap;

use generational_arena::Arena;

use crate::{
    asm::AsmBlockContent,
    block::BlockContent,
    function::FunctionContent,
    irtype::AggregateContent,
    metadata::{MetadataIndex, Metadatum, StorageOperation},
    module::ModuleContent,
    module::ModuleIterator,
    pointer::PointerContent,
    value::ValueContent,
};

/// The main IR context handle.
///
/// Every module, function, block and value is stored here.  Some aggregate metadata is also
/// managed by the context.
#[derive(Default)]
pub struct Context {
    pub modules: Arena<ModuleContent>,
    pub functions: Arena<FunctionContent>,
    pub blocks: Arena<BlockContent>,
    pub values: Arena<ValueContent>,
    pub pointers: Arena<PointerContent>,
    pub aggregates: Arena<AggregateContent>,
    pub asm_blocks: Arena<AsmBlockContent>,

    // The metadata indices for locations need a fast lookup, hence the metadata_reverse_map.
    // Using a HashMap might be overkill as most projects have only a handful of source files.
    pub metadata: Arena<Metadatum>,
    pub metadata_reverse_map: HashMap<*const std::path::PathBuf, MetadataIndex>,
    pub(crate) metadata_storage_indices: HashMap<StorageOperation, MetadataIndex>,

    next_unique_sym_tag: u64,
}

impl Context {
    /// Return an interator for every module in this context.
    pub fn module_iter(&self) -> ModuleIterator {
        ModuleIterator::new(self)
    }

    /// Get a globally unique symbol.
    ///
    /// The name will be in the form `"anon_N"`, where `N` is an incrementing decimal.
    pub fn get_unique_name(&mut self) -> String {
        format!("anon_{}", self.get_unique_id())
    }

    /// Get a globally unique symbol id.
    pub fn get_unique_id(&mut self) -> u64 {
        let sym = self.next_unique_sym_tag;
        self.next_unique_sym_tag += 1;
        sym
    }
}

use std::fmt::{Display, Error, Formatter};

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", crate::printer::to_string(self))
    }
}

impl From<Context> for String {
    fn from(context: Context) -> Self {
        crate::printer::to_string(&context)
    }
}
