
use std::str::FromStr;

// Expression AST type
enum Expr<T> {
    Value(T),
    UMinus(Box<Expr<T>>),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mult(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>)
}

// Catamorphism trait for Expression AST (this will be generated automatically in future)
trait ExprCata<T> {
    type Synth;

    fn fold_value(&self, v: &T) -> Self::Synth;
    fn fold_uminus(&self, e: &Box<Expr<T>>) -> Self::Synth;
    fn fold_add(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;
    fn fold_sub(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;
    fn fold_mult(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;
    fn fold_div(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;
}

// Transformation (this also should be generated automatically)
fn transform<T, Cata: ExprCata<T>>(cata: &Cata, e: &Expr<T>) -> Cata::Synth {
    match e {
        Expr::Value(ref v) => cata.fold_value(v),
        Expr::UMinus(ref e1) => cata.fold_uminus(e1),
        Expr::Add(ref e1, ref e2) => cata.fold_add(e1, e2),
        Expr::Sub(ref e1, ref e2) => cata.fold_sub(e1, e2),
        Expr::Mult(ref e1, ref e2) => cata.fold_mult(e1, e2),
        Expr::Div(ref e1, ref e2) => cata.fold_div(e1, e2),
    }
}

// Evaluator is a `newtype` for unit.
// We use it only to derive `impl ExprCata` on it.
struct Evaluator(());

impl ExprCata<i32> for Evaluator {
    type Synth = i32;

    fn fold_value(&self, v: &i32) -> Self::Synth {
        *v
    }

    fn fold_uminus(&self, e: &Box<Expr<i32>>) -> Self::Synth {
        -transform(self, &**e)
    }

    fn fold_add(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        transform(self, &**e1) + transform(self, &**e2)
    }

    fn fold_sub(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        transform(self, &**e1) - transform(self, &**e2)
    }

    fn fold_mult(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        transform(self, &**e1) * transform(self, &**e2)
    }

    fn fold_div(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        transform(self, &**e1) / transform(self, &**e2)
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
    let v = transform(&Evaluator(()), &e);
    println!("result={}", v);
}