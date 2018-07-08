trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: T) -> S;
}

trait Foldable1<T, TS, I, S> : Sized {
    type Tr: for<'a> Transformer<&'a Self, I, S>;

    fn transformer<Tr_T>(tr: Tr_T) -> Self::Tr
    where Tr_T: 'static + for<'a> Transformer<&'a T, I, TS> + Sized;
}

struct OptionFold<'a, T, TS, I, S> {
    fold_t : Box<'a + for<'b> Transformer<&'b T, I, TS>>,
//    fold_t : &'a + for<'b> Transformer<&'b T, I, TS>,
    fold_some : Box<'a + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I, &'b T) -> S>,
    fold_none : Box<'a + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I) -> S>,
}

impl<'a, T, TS, I, S> OptionFold<'a, T, TS, I, S> {

    fn with_fold_some<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I, &'b T) -> S
    {
        OptionFold { fold_some: Box::new(f), ..self }
    }

    fn with_fold_none<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I) -> S
    {
        OptionFold { fold_none: Box::new(f), ..self }
    }
}

impl<'a, 'b, T, TS, I, S> Transformer<&'b Option<T>, I, S> for OptionFold<'a, T, TS, I, S> {
    fn transform(&self, inh: I, x: &'b Option<T>) -> S {
        match x {
            Some(ref t) => (self.fold_some)(&*self, &*self.fold_t, inh, t),
            None => (self.fold_none)(&*self, &*self.fold_t, inh),
        }
    }
}

impl<T, TS, I, S> Foldable1<T, TS, I, S> for Option<T> {
    type Tr = OptionFold<'static, T, TS, I, S>;

    fn transformer<Tr_T>(tr: Tr_T) -> Self::Tr
    where Tr_T: 'static + for<'a> Transformer<&'a T, I, TS> + Sized {

//    fn transformer(tr: &for<'a> Transformer<&'a T, I, TS>) -> Self::Tr {
        OptionFold {
            fold_t: Box::new(tr),
            fold_some: Box::new(|tr, tr_t, inh, v| unimplemented!()),
            fold_none: Box::new(|tr, tr_t, inh| unimplemented!()),
        }
    }
}

impl<'a, I, S, F: Fn(I, i32) -> S> Transformer<&'a i32, I, S> for F {
    fn transform(&self, inh:I, x: &'a i32) -> S {
        (self)(inh, *x)
    }
}

fn const_i32<I>(inh: I, i: i32) -> i32 {
    i
}

fn main() {
    let tr = <Option<i32> as Foldable1<i32, i32, (), i32>>::transformer(const_i32)
        .with_fold_some(|tr, tri, inh, x| tri.transform(inh, x))
        .with_fold_none(|tr, tri, inh| tri.transform(inh, &0));

    println!("{}", tr.transform((), &Some(1)));
    println!("{}", tr.transform((), &None));
}