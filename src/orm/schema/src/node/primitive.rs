use crate::{
    node::{Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Primitive
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Primitive {
    pub def: Def,
    pub ty: String,
}

impl MacroNode for Primitive {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Primitive {}

impl VisitableNode for Primitive {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
