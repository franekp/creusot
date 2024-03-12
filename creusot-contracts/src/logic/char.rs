use crate::{
    std::ops::{Add, Div, Mul, Neg, Rem, Sub},
    *,
};

#[cfg_attr(creusot, rustc_diagnostic_item = "creusot_char", creusot::builtins = "prelude.Char.char")]
pub struct Char(*mut ());
