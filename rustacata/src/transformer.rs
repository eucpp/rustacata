
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