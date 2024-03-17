use ::std::ops::{Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive};
use crate::{
    logic::{ops::IndexLogic, Mapping},
    std::ops::RangeInclusiveExt, *,
};

#[cfg_attr(creusot, creusot::builtins = "seq.Seq.seq")]
pub struct Seq<T: ?Sized>(std::marker::PhantomData<T>);

impl<T> Seq<T> {
    #[cfg(creusot)]
    #[trusted]
    #[creusot::builtins = "seq.Seq.empty"]
    pub const EMPTY: Self = { Seq(std::marker::PhantomData) };

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.create"]
    pub fn new(_: Int, _: Mapping<Int, T>) -> Self {
        absurd
    }

    #[logic]
    #[open]
    pub fn get(self, ix: Int) -> Option<T> {
        if 0 <= ix && ix < self.len() {
            Some(self.index_logic(ix))
        } else {
            None
        }
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq_ext.SeqExt.subsequence"]
    pub fn subsequence(self, _: Int, _: Int) -> Self {
        absurd
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.singleton"]
    pub fn singleton(_: T) -> Self {
        absurd
    }

    #[logic]
    #[open]
    pub fn tail(self) -> Self {
        self.subsequence(1, self.len())
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_len"]
    #[creusot::builtins = "seq.Seq.length"]
    pub fn len(self) -> Int {
        absurd
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.set"]
    pub fn set(self, _: Int, _: T) -> Self {
        absurd
    }

    #[trusted]
    #[predicate]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.(==)"]
    pub fn ext_eq(self, _: Self) -> bool {
        absurd
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.snoc"]
    pub fn push(self, _: T) -> Self {
        absurd
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Seq.(++)"]
    pub fn concat(self, _: Self) -> Self {
        absurd
    }

    #[trusted]
    #[logic]
    #[open(self)]
    #[creusot::builtins = "seq.Reverse.reverse"]
    pub fn reverse(self) -> Self {
        absurd
    }

    #[predicate]
    #[open]
    pub fn permutation_of(self, o: Self) -> bool {
        self.permut(o, 0, self.len())
    }

    #[trusted]
    #[predicate]
    #[open(self)]
    #[creusot::builtins = "seq.Permut.permut"]
    pub fn permut(self, _: Self, _: Int, _: Int) -> bool {
        absurd
    }

    #[trusted]
    #[predicate]
    #[open(self)]
    #[creusot::builtins = "seq.Permut.exchange"]
    pub fn exchange(self, _: Self, _: Int, _: Int) -> bool {
        absurd
    }

    #[open]
    #[predicate]
    pub fn contains(self, e: T) -> bool {
        pearlite! { exists<i : Int> 0 <= i &&  i <self.len() && self[i] == e }
    }

    #[open]
    #[predicate]
    pub fn sorted_range(self, l: Int, u: Int) -> bool
    where
        T: OrdLogic,
    {
        pearlite! {
            forall<i : Int, j : Int> l <= i && i <= j && j < u ==> self[i] <= self[j]
        }
    }

    #[open]
    #[predicate]
    pub fn sorted(self) -> bool
    where
        T: OrdLogic,
    {
        self.sorted_range(0, self.len())
    }
}

impl<T> Seq<&T> {
    #[logic]
    #[open]
    #[trusted]
    #[creusot::builtins = "prelude.Seq.to_owned"]
    pub fn to_owned_seq(self) -> Seq<T> {
        pearlite! {absurd}
    }
}

impl<T> IndexLogic<Int> for Seq<T> {
    type Item = T;

    #[logic]
    #[trusted]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_index"]
    #[creusot::builtins = "seq.Seq.get"]
    fn index_logic(self, _: Int) -> Self::Item {
        absurd
    }
}

impl<T> IndexLogic<Range<Int>> for Seq<T> {
    type Item = Seq<T>;

    #[logic]
    #[trusted]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_index_range"]
    #[ensures(result == self.subsequence(r.start, r.end))]
    fn index_logic(self, r: Range<Int>) -> Self::Item {
        absurd
    }
}

impl<T> IndexLogic<RangeInclusive<Int>> for Seq<T> {
    type Item = Seq<T>;

    #[logic]
    #[trusted]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_index_range_inclusive"]
    #[ensures(result == self.subsequence(r.start_log(), r.end_log() + 1))]
    fn index_logic(self, r: RangeInclusive<Int>) -> Self::Item {
        absurd
    }
}

impl<T> IndexLogic<RangeFrom<Int>> for Seq<T> {
    type Item = Seq<T>;

    #[logic]
    #[trusted]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_index_range_from"]
    #[ensures(result == self.subsequence(r.start, self.len()))]
    fn index_logic(self, r: RangeFrom<Int>) -> Self::Item {
        absurd
    }
}

impl<T> IndexLogic<RangeTo<Int>> for Seq<T> {
    type Item = Seq<T>;

    #[logic]
    #[trusted]
    #[open(self)]
    #[rustc_diagnostic_item = "seq_index_range_to"]
    #[ensures(result == self.subsequence(0, r.end))]
    fn index_logic(self, r: RangeTo<Int>) -> Self::Item {
        absurd
    }
}

impl<T> IndexLogic<RangeToInclusive<Int>> for Seq<T> {
    type Item = Seq<T>;

    #[logic]
    #[trusted]
    #[open]
    #[rustc_diagnostic_item = "seq_index_range_to_inclusive"]
    #[ensures(result == self.subsequence(0, r.end + 1))]
    fn index_logic(self, r: RangeToInclusive<Int>) -> Self::Item {
        absurd
    }
}
