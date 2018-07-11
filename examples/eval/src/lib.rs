#![feature(proc_macro)]

extern crate rustacata_macro;

use rustacata_macro::cata;

pub trait Transformer<T, R, Env> {
    fn transform(&self, env: Env, x: T) -> R;
}

pub trait Foldable<R, Env> : Sized {
    type Tr: for<'a> Transformer<&'a Self, R, Env>;

    fn transformer() -> Self::Tr;
}

#[cata]
pub enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

//pub struct ExprFold<'a, I, S> {
//    fold_value : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S>,
//    fold_add : Box<'a + for <'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
//    fold_mult : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
//}
//
//impl<'a, I, S> ExprFold<'a, I, S> {
//
//    pub fn with_fold_value<'c: 'a, F>(self, f: F) -> Self
//        where
//            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S
//    {
//        ExprFold { fold_value: Box::new(f), ..self }
//    }
//
//    pub fn with_fold_add<'c: 'a, F>(self, f: F) -> Self
//        where
//            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
//    {
//        ExprFold { fold_add: Box::new(f), ..self }
//    }
//
//    pub fn with_fold_mult<'c: 'a, F>(self, f: F) -> Self
//        where
//            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
//    {
//        ExprFold { fold_mult: Box::new(f), ..self }
//    }
//}

impl<'a, 'b, R, Env> Transformer<&'b Expr, R, Env> for ExprAlgebra<'a, R, Env> {
    fn transform(&self, env: Env, x: &'b Expr) -> R {
        match x {
            Expr::Value(ref v) => (self.tr_Value)(&*self, env, v),
            Expr::Add(ref e1, ref e2) => (self.tr_Add)(&*self, env, e1, e2),
            Expr::Mult(ref e1, ref e2) => (self.tr_Mult)(&*self, env, e1, e2),
        }
    }
}

impl<R, Env> Foldable<R, Env> for Expr {
    type Tr = ExprAlgebra<'static, R, Env>;

    fn transformer() -> Self::Tr {
        ExprAlgebra {
            tr_Value: Box::new(|tr, inh, v| unimplemented!()),
            tr_Add: Box::new(|tr, inh, e1, e2| unimplemented!()),
            tr_Mult: Box::new(|tr, inh, e1, e2| unimplemented!()),
        }
    }
}

