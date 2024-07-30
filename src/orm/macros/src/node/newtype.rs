use crate::{
    helper::{quote_option, quote_vec},
    imp,
    node::{
        Def, Guide, MacroNode, Node, Trait, TraitNode, Traits, TypeSanitizer, TypeValidator, Value,
    },
};
use darling::FromMeta;
use orm::types::{Cardinality, Primitive, PrimitiveType};
use proc_macro2::TokenStream;
use schema::Schemable;

///
/// Newtype
///

#[derive(Debug, FromMeta)]
pub struct Newtype {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

    pub value: Value,
    pub primitive: Option<Primitive>,

    #[darling(default)]
    pub guide: Option<Guide>,

    #[darling(multiple, rename = "sanitizer")]
    pub sanitizers: Vec<TypeSanitizer>,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Newtype {
    fn expand(&self) -> TokenStream {
        let Self { value, .. } = self;
        let Def {
            ident, generics, ..
        } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub struct #ident #generics(#value);
            #imp
        };

        // debug
        assert!(!self.debug, "{q}");

        q
    }
}

impl MacroNode for Newtype {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Newtype {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_db_traits();
        traits.extend(vec![
            Trait::AsRef,
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::From,
        ]);

        match &self.value.cardinality() {
            Cardinality::One | Cardinality::Opt => {
                traits.extend(vec![Trait::Ord, Trait::PartialOrd]);
            }
            Cardinality::Many => {}
        }

        // inner
        if self.primitive.is_some() {
            traits.add(Trait::Inner);
        }

        // primitive
        match self.primitive.map(|p| p.ty()) {
            Some(PrimitiveType::Integer | PrimitiveType::Decimal) => {
                traits.extend(vec![
                    Trait::Add,
                    Trait::AddAssign,
                    Trait::Copy,
                    Trait::Display,
                    Trait::FromStr,
                    Trait::Mul,
                    Trait::MulAssign,
                    Trait::NumCast,
                    Trait::NumFromPrimitive,
                    Trait::NumToPrimitive,
                    Trait::Sub,
                    Trait::SubAssign,
                ]);
            }
            Some(PrimitiveType::String) => {
                traits.extend(vec![Trait::Display, Trait::FromStr]);
            }
            _ => {}
        }

        traits.list()
    }

    fn map_derive(&self, t: Trait) -> bool {
        match t {
            // derive default if no default value
            Trait::Default => self.value.default.is_none(),
            _ => true,
        }
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Default if self.value.default.is_some() => imp::default::newtype(self, t),
            Trait::Display => imp::display::newtype(self, t),
            Trait::Filterable => imp::filterable::newtype(self, t),
            Trait::From => imp::from::newtype(self, t),
            Trait::Inner => imp::inner::newtype(self, t),
            Trait::NumCast => imp::num::cast::newtype(self, t),
            Trait::NumToPrimitive => imp::num::to_primitive::newtype(self, t),
            Trait::NumFromPrimitive => imp::num::from_primitive::newtype(self, t),
            Trait::Orderable => imp::orderable::newtype(self, t),
            Trait::PrimaryKey => imp::primary_key::newtype(self, t),
            Trait::SanitizeAuto => imp::sanitize_auto::newtype(self, t),
            Trait::ValidateAuto => imp::validate_auto::newtype(self, t),
            Trait::Visitable => imp::visitable::newtype(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Newtype {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let value = self.value.schema();
        let primitive = quote_option(&self.primitive, Primitive::schema);
        let guide = quote_option(&self.guide, Guide::schema);
        let sanitizers = quote_vec(&self.sanitizers, TypeSanitizer::schema);
        let validators = quote_vec(&self.validators, TypeValidator::schema);

        quote! {
            ::mimic::schema::node::SchemaNode::Newtype(::mimic::schema::node::Newtype {
                def: #def,
                value: #value,
                primitive: #primitive,
                guide: #guide,
                sanitizers: #sanitizers,
                validators: #validators,
            })
        }
    }
}
