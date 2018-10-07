
pub trait Transformer<A, B> {
    fn transform(&self, a: A) -> B;

//    fn compose<C, Tr>(Self, tr: Tr) -> impl Transformer<A, C>
//    where
//        Tr: Transformer<B, C>;
}

pub trait Foldable<R> : Sized {
    type Tr: Transformer<Self, R>;

    fn transformer() -> Self::Tr;
}

pub trait Foldable1<T, TR, R> : Sized {
    type Tr: Transformer<Self, R>;

    fn transformer<TrT: Transformer<T, TR>>(tr_t: TrT) -> Self::Tr;
}