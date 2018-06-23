#![feature(proc_macro)]

//extern crate rustacata_macro;

//use rustacata_macro::derive_transformer;

trait Foldable<T, I, S> {
    fn transform(&self, inh: I, x: &T) -> S;

//    fn map<F: Fn(U) -> T>(Self, f: F) -> impl Transformer<U, I, S>;
}

// Expression AST type
//#[derive_transformer]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

struct ExprFold<'a, I, S> {
    fold_value : Box<'a + Fn(&Foldable<Expr, I, S>, I, &i32) -> S>,
    fold_add : Box<'a + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S>,
    fold_mult : Box<'a + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S>,
}

impl<'a, I, S> ExprFold<'a, I, S> {

    fn new<FValue, FAdd, FMult>(
        fold_value: FValue,
        fold_add: FAdd,
        fold_mult: FMult,
    ) -> ExprFold<'a, I, S>
        where
            FValue: 'a + Fn(&Foldable<Expr, I, S>, I, &i32) -> S,
            FAdd: 'a + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S,
            FMult: 'a + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S,
    {
        ExprFold {
            fold_value: Box::new(fold_value),
            fold_add: Box::new(fold_add),
            fold_mult: Box::new(fold_mult),
        }
    }

    fn set_fold_value<'b: 'a, F>(&mut self, f: F) -> &mut Self
    where
        F: 'b + Fn(&Foldable<Expr, I, S>, I, &i32) -> S
    {
        self.fold_value = Box::new(f); self
    }

    fn set_fold_add<'b: 'a, F>(&mut self, f: F) -> &mut Self
    where
        F: 'b + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S
    {
        self.fold_add = Box::new(f); self
    }

    fn set_fold_mult<'b: 'a, F>(&mut self, f: F) -> &mut Self
        where
            F: 'b + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S
    {
        self.fold_mult = Box::new(f); self
    }
}

impl<'a, I, S> Default for ExprFold<'a, I, S> {
    fn default() -> Self {
        ExprFold {
            fold_value: Box::new(|tr, inh, v| unimplemented!()),
            fold_add: Box::new(|tr, inh, e1, e2| unimplemented!()),
            fold_mult: Box::new(|tr, inh, e1, e2| unimplemented!()),
        }
    }
}

impl<'a, I, S> Foldable<Expr, I, S> for ExprFold<'a, I, S> {
    fn transform(&self, inh: I, x: &Expr) -> S {
        match x {
            Expr::Value(ref v) => (self.fold_value)(&*self, inh, v),
            Expr::Add(ref e1, ref e2) => (self.fold_add)(&*self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => (self.fold_mult)(&*self, inh, e1, e2),
        }
    }
}

struct Evaluator<'a>(ExprFold<'a, (), i32>);

impl<'a> Evaluator<'a> {
    fn new() -> Evaluator<'a> {
        let mut fold = ExprFold::default();
        fold
            .set_fold_value(|tr: &Foldable<Expr, (), i32>, inh: (), v: &i32| {
                *v
            })
            .set_fold_add(|tr: &Foldable<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
            })
            .set_fold_mult(|tr: &Foldable<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
            });
        Evaluator(fold)
    }

    fn eval(&self, e: &Expr) -> i32 {
        self.0.transform((), e)
    }
}

struct Map<'a>(ExprFold<'a, (), Expr>);

impl<'a> Map<'a> {
    fn new() -> Self {
        let mut fold = ExprFold::default();
        fold.set_fold_value(|tr: &Foldable<Expr, (), Expr>, inh: (), v: &i32| {
            Expr::Value(*v)
        })
            .set_fold_add(|tr: &Foldable<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Add(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            })
            .set_fold_mult(|tr: &Foldable<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Mult(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            });
        Map(fold)
    }

    fn set_fold_value<'b: 'a, F>(&mut self, f: F) -> &mut Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &i32) -> Expr
    {
        self.0.set_fold_value(f); self
    }

    fn set_fold_add<'b: 'a, F>(&mut self, f: F) -> &mut Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &Box<Expr>, &Box<Expr>) -> Expr
    {
        self.0.set_fold_add(f); self
    }

    fn set_fold_mult<'b: 'a, F>(&mut self, f: F) -> &mut Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &Box<Expr>, &Box<Expr>) -> Expr
    {
        self.0.set_fold_mult(f); self
    }

    fn map(&self, e: &Expr) -> Expr {
        self.0.transform((), e)
    }
}

fn main() {
    // 2 * (7 + 1) = 16
    let e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let evaluator = Evaluator::new();

    let v = evaluator.eval(&e);
    println!("result={}", v);

    let mut inc_mapper = Map::new();
    inc_mapper.set_fold_value(|tr: &Foldable<Expr, (), Expr>, inh: (), v: &i32| {
        Expr::Value(*v + 1)
    });
    let e_inc = inc_mapper.map(&e);
    let v_inc = evaluator.eval(&e_inc);
    println!("result={}", v_inc);
}