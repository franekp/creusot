use crate as creusot_contracts;
use crate::Resolve;
use creusot_contracts_proc::*;

/// `FnOnceSpec` is an extension trait for the `FnOnce` trait, used for
/// adding a specification to closures. It should not be used directly.
#[rustc_diagnostic_item = "fn_once_spec"]
pub trait FnOnceSpec<Args>: FnOnce<Args> {
    #[predicate]
    fn precondition(self, a: Args) -> bool;

    #[predicate]
    fn postcondition_once(self, a: Args, res: Self::Output) -> bool;
}

/// `FnMutSpec` is an extension trait for the `FnMut` trait, used for
/// adding a specification to closures. It should not be used directly.
#[rustc_diagnostic_item = "fn_mut_spec"]
pub trait FnMutSpec<Args>: FnMut<Args> + FnOnceSpec<Args> {
    #[predicate]
    fn postcondition_mut(&mut self, _: Args, _: Self::Output) -> bool;

    #[law]
    #[ensures(self.postcondition_once(args, res) == exists<s: &mut Self> *s == self && s.postcondition_mut(args, res) && (^s).resolve())]
    fn fn_mut_once(self, args: Args, res: Self::Output)
    where
        Self: Sized;
}

/// `FnSpec` is an extension trait for the `Fn` trait, used for
/// adding a specification to closures. It should not be used directly.
#[rustc_diagnostic_item = "fn_spec"]
pub trait FnSpec<Args>: Fn<Args> + FnMutSpec<Args> {
    #[predicate]
    fn postcondition(&self, _: Args, _: Self::Output) -> bool;

    #[law]
    #[ensures(self.postcondition_mut(args, res) == self.resolve() && self.postcondition(args, res))]
    fn fn_mut(&mut self, args: Args, res: Self::Output);

    #[law]
    #[ensures(self.postcondition_once(args, res) == self.resolve() && self.postcondition(args, res))]
    fn fn_once(self, args: Args, res: Self::Output)
    where
        Self: Sized;
}

impl<Args, F: FnOnce<Args>> FnOnceSpec<Args> for F {
    #[predicate]
    #[trusted]
    #[rustc_diagnostic_item = "fn_once_impl_precond"]
    fn precondition(self, _: Args) -> bool {
        absurd
    }

    #[predicate]
    #[trusted]
    #[rustc_diagnostic_item = "fn_once_impl_postcond"]
    fn postcondition_once(self, _: Args, _: Self::Output) -> bool {
        absurd
    }
}

impl<Args, F: FnMut<Args>> FnMutSpec<Args> for F {
    #[predicate]
    #[trusted]
    #[rustc_diagnostic_item = "fn_mut_impl_postcond"]
    fn postcondition_mut(&mut self, _: Args, _: Self::Output) -> bool {
        absurd
    }

    #[law]
    #[trusted]
    #[ensures(self.postcondition_once(args, res) == exists<s: &mut Self> *s == self && s.postcondition_mut(args, res) && (^s).resolve())]
    fn fn_mut_once(self, args: Args, res: Self::Output) {}
}

impl<Args, F: Fn<Args>> FnSpec<Args> for F {
    #[predicate]
    #[trusted]
    #[rustc_diagnostic_item = "fn_impl_postcond"]
    fn postcondition(&self, _: Args, _: Self::Output) -> bool {
        absurd
    }

    #[law]
    #[trusted]
    #[ensures(self.postcondition_mut(args, res) == self.resolve() && self.postcondition(args, res))]
    fn fn_mut(&mut self, args: Args, res: Self::Output) {}

    #[law]
    #[trusted]
    #[ensures(self.postcondition_once(args, res) == self.resolve() && self.postcondition(args, res))]
    fn fn_once(self, args: Args, res: Self::Output) {}
}

extern_spec! {
    mod std {
        mod ops {
            // FIXME: we should not need the `Self :` bounds, because they are implied by the main trait bound.
            // But if we remove them, some test fail.
            trait FnOnce<Args> where Self : FnOnceSpec<Args> {
                #[requires(self.precondition(arg))]
                #[ensures(self.postcondition_once(arg, result))]
                fn call_once(self, arg: Args) -> Self::Output;
            }

            trait FnMut<Args> where Self : FnMutSpec<Args> {
                #[requires((*self).precondition(arg))]
                #[ensures(self.postcondition_mut(arg, result))]
                fn call_mut(&mut self, arg: Args) -> Self::Output;
            }

            trait Fn<Args> where Self : FnSpec<Args> {
                #[requires((*self).precondition(arg))]
                #[ensures(self.postcondition(arg, result))]
                fn call(&self, arg: Args) -> Self::Output;
            }
        }
    }
}