
pub trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: T) -> S;
}

pub trait Foldable<I, S> : Sized {
    type Tr: for<'a> Transformer<&'a Self, I, S>;

    fn transformer() -> Self::Tr;
}

pub enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

pub struct ExprFold<'a, I, S> {
    fold_value : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S>,
    fold_add : Box<'a + for <'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
    fold_mult : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
}

impl<'a, I, S> ExprFold<'a, I, S> {

    pub fn with_fold_value<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S
    {
        ExprFold { fold_value: Box::new(f), ..self }
    }

    pub fn with_fold_add<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
    {
        ExprFold { fold_add: Box::new(f), ..self }
    }

    pub fn with_fold_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
    {
        ExprFold { fold_mult: Box::new(f), ..self }
    }
}

impl<'a, 'b, I, S> Transformer<&'b Expr, I, S> for ExprFold<'a, I, S> {
    fn transform(&self, inh: I, x: &'b Expr) -> S {
        match x {
            Expr::Value(ref v) => (self.fold_value)(&*self, inh, v),
            Expr::Add(ref e1, ref e2) => (self.fold_add)(&*self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => (self.fold_mult)(&*self, inh, e1, e2),
        }
    }
}

impl<I, S> Foldable<I, S> for Expr {
    type Tr = ExprFold<'static, I, S>;

    fn transformer() -> Self::Tr {
        ExprFold {
            fold_value: Box::new(|tr, inh, v| unimplemented!()),
            fold_add: Box::new(|tr, inh, e1, e2| unimplemented!()),
            fold_mult: Box::new(|tr, inh, e1, e2| unimplemented!()),
        }
    }
}

