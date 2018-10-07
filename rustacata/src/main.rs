#![feature(proc_macro)]

// Expression AST type
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

trait Transformer<A, B> {
    fn transform(&self, a: A) -> B;

//    fn compose<Tr1, Tr2>(tr1: Tr1, tr2: Tr2) -> impl Tr3
//        where
//            Tr1: Transformer<A, B>,
//            Tr2: Transformer<B, C>,
//            Tr3: Transformer<A, C>;
}

//trait ExprAlg<A> {
//    fn tr_value(&self, x: &i32) -> A;
//    fn tr_add<F>(&self, fa: F, fb: F) -> A where F: FnOnce() -> A;
//    fn tr_mult<F>(&self, fa: F, fb: F) -> A where F: FnOnce() -> A;
//}

struct ExprFold<A> {
    fold_value : for<'b> fn(&'b i32) -> A,
    fold_add : for <'b> fn(&'b dyn Fn() -> A, &'b dyn Fn() -> A) -> A,
    fold_mult : for <'b> fn(&'b dyn Fn() -> A, &'b dyn Fn() -> A) -> A,
}

impl<A> ExprFold<A> {

    fn new() -> Self {
        ExprFold {
            fold_value: |&i| unimplemented!(),
            fold_add: |e1, e2| unimplemented!(),
            fold_mult: |e1, e2| unimplemented!(),
        }
    }

    fn with_fold_value(self, f: for<'b> fn(&'b i32) -> A) -> Self
    {
        ExprFold { fold_value: f, ..self }
    }

    fn with_fold_add(self, f: for <'b> fn(&'b dyn Fn() -> A, &'b dyn Fn() -> A) -> A) -> Self
    {
        ExprFold { fold_add: f, ..self }
    }

    fn with_fold_mult(self, f: for <'b> fn(&'b dyn Fn() -> A, &'b dyn Fn() -> A) -> A) -> Self
    {
        ExprFold { fold_mult: f, ..self }
    }
}

impl<'b, A> Transformer<&'b Expr, A> for ExprFold<A> {
    fn transform(&self, x: &'b Expr) -> A {
        match *x {
            Expr::Value(ref v) => (self.fold_value)(v),
            Expr::Add(ref e1, ref e2) => (self.fold_add)(&|| self.transform(&*e1), &|| self.transform(&*e2)),
            Expr::Mult(ref e1, ref e2) => (self.fold_mult)(&|| self.transform(&*e1), &|| self.transform(&*e2)),
        }
    }
}

fn main() {
    //    // 2 * (7 + 1) = 16
    let e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let evaluator = ExprFold::new()
        .with_fold_value(|v| {
            *v
        })
        .with_fold_add(|e1, e2| {
            e1() + e2()
        })
        .with_fold_mult(|e1, e2| {
            e1() * e2()
        });

    println!("result={}", evaluator.transform(&e));
}