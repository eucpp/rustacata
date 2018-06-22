#![feature(proc_macro)]

//extern crate rustacata_macro;

//use rustacata_macro::derive_transformer;

trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: &T) -> S;
}

// Expression AST type
//#[derive_transformer]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

struct ExprTransformer<'a, I, S> {
    fold_value : Box<'a + Fn(&Transformer<Expr, I, S>, I, &i32) -> S>,
    fold_add : Box<'a + Fn(&Transformer<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S>,
    fold_mult : Box<'a + Fn(&Transformer<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S>,
}

impl<'a, I, S> ExprTransformer<'a, I, S> {

    fn new<FValue, FAdd, FMult>(
        fold_value: FValue,
        fold_add: FAdd,
        fold_mult: FMult,
    ) -> ExprTransformer<'a, I, S>
    where
        FValue: 'a + Fn(&Transformer<Expr, I, S>, I, &i32) -> S,
        FAdd: 'a + Fn(&Transformer<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S,
        FMult: 'a + Fn(&Transformer<Expr, I, S>, I, &Box<Expr>, &Box<Expr>) -> S,
    {
        ExprTransformer {
            fold_value: Box::new(fold_value),
            fold_add: Box::new(fold_add),
            fold_mult: Box::new(fold_mult),
        }
    }

    fn set_fold_value<'b: 'a, FValue>(&mut self, fold_value: FValue)
    where
        FValue: 'b + Fn(&Transformer<Expr, I, S>, I, &i32) -> S,
    {
        self.fold_value = Box::new(fold_value);
    }
}

impl<'a, I, S> Transformer<Expr, I, S> for ExprTransformer<'a, I, S> {
    fn transform(&self, inh: I, x: &Expr) -> S {
        match x {
            Expr::Value(ref v) => (self.fold_value)(&*self, inh, v),
            Expr::Add(ref e1, ref e2) => (self.fold_add)(&*self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => (self.fold_mult)(&*self, inh, e1, e2),
        }
    }
}

struct Evaluator<'a>(ExprTransformer<'a, (), i32>);

impl<'a> Evaluator<'a> {
    fn new() -> Evaluator<'a> {
        Evaluator(ExprTransformer::new(
            |tr: &Transformer<Expr, (), i32>, inh: (), v: &i32| {
                *v
            },
            |tr: &Transformer<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
            },
            |tr: &Transformer<Expr, (), i32>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
            }
        ))
    }

    fn eval(e: &Expr) -> i32 {
        Self::new().0.transform((), e)
    }
}

struct Mapper<'a>(ExprTransformer<'a, (), Expr>);

impl<'a> Mapper<'a> {
    fn new() -> Mapper<'a> {
        Mapper(ExprTransformer::new(
            |tr: &Transformer<Expr, (), Expr>, inh: (), v: &i32| {
                Expr::Value(*v)
            },
            |tr: &Transformer<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Add(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            },
            |tr: &Transformer<Expr, (), Expr>, inh: (), e1: &Box<Expr>, e2: &Box<Expr>| {
                Expr::Mult(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            }
        ))
    }

    fn map(e: &Expr) -> Expr {
        Self::new().0.transform((), e)
    }
}

//// Mapper transformer
//struct Mapper(());
//
//// Default implementation for `map`;
//// We should (somehow) be able to override it partly in other implementations
//impl ExprTransformation for Mapper {
//    type Inh = ();
//    type Synth = Expr;
//
//    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth {
//        Expr::Value(*v)
//    }
//
//    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Expr::Add(
//            Box::new(tr.transform(inh, &**e1)),
//            Box::new(tr.transform(inh, &**e2)),
//        )
//    }
//
//    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Expr::Mult(
//            Box::new(tr.transform(inh, &**e1)),
//            Box::new(tr.transform(inh, &**e2)),
//        )
//    }
//}
//
//struct IncMapper(Mapper);
//
//impl ExprTransformation for IncMapper {
//    type Inh = <Mapper as ExprTransformation>::Inh;
//    type Synth = <Mapper as ExprTransformation>::Synth;
//
//    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth {
//        Expr::Value(*v + 1)
//    }
//
//    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Mapper::fold_add(tr, inh, e1, e2)
//    }
//
//    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Mapper::fold_mult(tr, inh, e1, e2)
//    }
//}


fn main() {
    // 2 * (7 + 1) = 16
    let e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let v = Evaluator::eval(&e);
//    let v = transform(&Evaluator(()), (), &e);
//    let v = ExprTransformer.transform((), &e);
//        (Evaluator(())).transform((), &e);
//    let v = ((), &e);
    println!("result={}", v);

    let mut inc_mapper = Mapper::new();
    inc_mapper.0.set_fold_value(|tr: &Transformer<Expr, (), Expr>, inh: (), v: &i32| {
        Expr::Value(*v + 1)
    });
    let e_inc = inc_mapper.0.transform((), &e);
    let v_inc = Evaluator::eval(&e_inc);
    println!("result={}", v_inc);

    // 3 * (8 + 2) = 30
//    let e_inc = inc_mapper.transform((), &e);
//    let v_inc = evaluator.transform((), &e_inc);
//    println!("result={}", v_inc);
}