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

    fn with_fold_value<'b: 'a, F>(self, f: F) -> Self
    where
        F: 'b + Fn(&Foldable<Expr, I, S>, I, &i32) -> S
    {
        ExprFold { fold_value: Box::new(f), ..self }
    }

    fn with_fold_add<'b: 'a, F>(self, f: F) -> Self
    where
        F: 'b + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S
    {
        ExprFold { fold_add: Box::new(f), ..self }
    }

    fn with_fold_mult<'b: 'a, F>(self, f: F) -> Self
        where
            F: 'b + Fn(&Foldable<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S
    {
        ExprFold { fold_mult: Box::new(f), ..self }
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
    fn eval(&self, e: &Expr) -> i32 {
        self.0.transform((), e)
    }
}

impl<'a> Default for Evaluator<'a> {
    fn default() -> Self {
        Evaluator(ExprFold::default()
            .with_fold_value(|tr: &Foldable<Expr, (), i32>, inh: (), v: &i32| {
                *v
            })
            .with_fold_add(|tr: &Foldable<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
            })
            .with_fold_mult(|tr: &Foldable<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
            }))
    }
}

struct Map<'a>(ExprFold<'a, (), Expr>);

impl<'a> Default for Map<'a> {
    fn default() -> Self {
        Map(ExprFold::default()
            .with_fold_value(|tr: &Foldable<Expr, (), Expr>, inh: (), v: &i32| {
                Expr::Value(*v)
            })
            .with_fold_add(|tr: &Foldable<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Add(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            })
            .with_fold_mult(|tr: &Foldable<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Mult(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            })
        )
    }
}

impl<'a> Map<'a> {
    fn with_map_value<'b: 'a, F>(self, f: F) -> Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &i32) -> Expr
    {
        Map(ExprFold { fold_value: Box::new(f), ..self.0 })
    }

    fn with_map_add<'b: 'a, F>(self, f: F) -> Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &Box<Expr>, &Box<Expr>) -> Expr
    {
        Map(ExprFold { fold_add: Box::new(f), ..self.0 })
    }

    fn with_map_mult<'b: 'a, F>(self, f: F) -> Self
        where
            F: 'b + Fn(&Foldable<Expr, (), Expr>, (), &Box<Expr>, &Box<Expr>) -> Expr
    {
        Map(ExprFold { fold_mult: Box::new(f), ..self.0 })
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

    let evaluator = Evaluator::default();

    let v = evaluator.eval(&e);
    println!("result={}", v);

    let mut inc_mapper = Map::default().with_map_value(
        |tr: &Foldable<Expr, (), Expr>, inh: (), v: &i32| {
            Expr::Value(*v + 1)
        });
    let e_inc = inc_mapper.map(&e);
    let v_inc = evaluator.eval(&e_inc);
    println!("result={}", v_inc);
}