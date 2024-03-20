use crate::*;
pub use ::std::default::*;

pub trait Default: ::std::default::Default {
    #[predicate(prophetic)]
    fn is_default(self) -> bool;
}

#[cfg(not(feature = "no_std_default_spec"))]
extern_spec! {
    mod std {
        mod default {
            trait Default where Self : Default {
                #[ensures(result.is_default())]
                fn default() -> Self;
            }
        }
    }
}

impl Default for bool {
    #[predicate]
    #[open]
    fn is_default(self) -> bool {
        pearlite! { self == false }
    }
}
