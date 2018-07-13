
trait Transformer<A, B> : Sized {
    fn transform(&self, a: A) -> B;

    fn compose<C, Tr>(Self, tr: Tr) -> impl Transformer<A, C>
    where
        Tr: Transformer<B, C>;
}