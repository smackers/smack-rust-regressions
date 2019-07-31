//pub use std::marker::PhantomData;
use label::*;
use smack::*;
pub use smack::PhantomData;

//#[derive(Debug)]
#[derive(Default)]
pub struct SVec<T: Default> {
    pub l: Label,
    pub phantom: PhantomData<T>
}

impl<T: Default> SVec<T> {
    pub fn append(&mut self, other: &mut Self) {
        self.l = combine_labels(self.l, other.l);
    }
}

#[macro_export]
macro_rules! svec {
    ( $elem : expr ; $n : expr => $l : expr) => {SVec{l: $l, phantom: Default::default()}};
    ( $ ( $x : expr ) , * => $l : expr) => {SVec{l:$l, phantom: Default::default()}};
    ( $ ( $x : expr , ) * => $l : expr) => {SVec{l:$l, phantom: Default::default()}};
}

#[macro_export]
macro_rules! check_label {
    ( $x : expr, $l : expr ) => {assert!($x.l == $l)};
}
