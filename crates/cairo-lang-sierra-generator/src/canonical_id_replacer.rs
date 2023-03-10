#[cfg(test)]
#[path = "canonical_id_replacer_test.rs"]
mod test;

use std::collections::HashMap;

use cairo_lang_sierra::ids::{ConcreteLibfuncId, ConcreteTypeId, FunctionId};

use crate::replace_ids::SierraIdReplacer;

#[derive(Default)]
pub struct CanonicalReplacer {
    type_ids: HashMap<ConcreteTypeId, ConcreteTypeId>,
    function_ids: HashMap<FunctionId, FunctionId>,
    libfunc_ids: HashMap<ConcreteLibfuncId, ConcreteLibfuncId>,
}

/// A replacer that replace the Ids in the program with canonical onces.
/// The canonical ids are defined by the order of the declaration in the program.
/// The first type_id is 0, the second type id is 1, etc.
impl CanonicalReplacer {
    /// Builds a replacer from a program.
    pub fn from_program(program: &cairo_lang_sierra::program::Program) -> Self {
        let mut type_ids = HashMap::default();

        for type_declaration in &program.type_declarations {
            let new_id = ConcreteTypeId::from_usize(type_ids.len());
            type_ids.insert(type_declaration.id.clone(), new_id.clone());
        }

        let mut function_ids = HashMap::default();
        for function in &program.funcs {
            let new_id = FunctionId::from_usize(function_ids.len());
            function_ids.insert(function.id.clone(), new_id.clone());
        }

        let mut libfunc_ids = HashMap::default();
        for libfunc_declaration in &program.libfunc_declarations {
            let new_id = ConcreteLibfuncId::from_usize(libfunc_ids.len());
            libfunc_ids.insert(libfunc_declaration.id.clone(), new_id.clone());
        }

        Self { type_ids, function_ids, libfunc_ids }
    }
}

impl SierraIdReplacer for CanonicalReplacer {
    fn replace_libfunc_id(
        &self,
        id: &cairo_lang_sierra::ids::ConcreteLibfuncId,
    ) -> cairo_lang_sierra::ids::ConcreteLibfuncId {
        self.libfunc_ids.get(id).expect("Unexpected libfunc id.").clone()
    }

    fn replace_type_id(
        &self,
        id: &cairo_lang_sierra::ids::ConcreteTypeId,
    ) -> cairo_lang_sierra::ids::ConcreteTypeId {
        self.type_ids.get(id).expect("Unexpected type id.").clone()
    }

    fn replace_function_id(
        &self,
        id: &cairo_lang_sierra::ids::FunctionId,
    ) -> cairo_lang_sierra::ids::FunctionId {
        self.function_ids.get(id).expect("Unexpected type id.").clone()
    }
}
