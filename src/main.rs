
// Expression AST type
enum Expr<T> {
    Value(T),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Mult(Box<Expr<T>>, Box<Expr<T>>),
}

// Catamorphism trait for Expression AST (this will be generated automatically in future)
trait ExprTransformer<T> {
    type Synth;

    fn fold_value(&self, v: &T) -> Self::Synth;
    fn fold_add(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;
    fn fold_mult(&self, e1: &Box<Expr<T>>, e2: &Box<Expr<T>>) -> Self::Synth;

    fn transform(&self, x: &Expr<T>) -> Self::Synth {
        match x {
            Expr::Value(ref v) => self.fold_value(v),
            Expr::Add(ref e1, ref e2) => self.fold_add(e1, e2),
            Expr::Mult(ref e1, ref e2) => self.fold_mult(e1, e2),
        }
    }
}

// Evaluator is a `newtype` for unit.
// We use it only to derive `impl ExprCata` on it.
struct Evaluator(());

impl ExprTransformer<i32> for Evaluator {
    type Synth = i32;

    fn fold_value(&self, v: &i32) -> Self::Synth {
        *v
    }

    fn fold_add(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        self.transform(&**e1) + self.transform(&**e2)
    }

    fn fold_mult(&self, e1: &Box<Expr<i32>>, e2: &Box<Expr<i32>>) -> Self::Synth {
        Evaluator::transform(self, &**e1) * Evaluator::transform(self, &**e2)
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
    let v = Evaluator::transform(&Evaluator(()), &e);
    println!("result={}", v);
}