#![feature(proc_macro)]

// Expression AST type

enum Bop { Add, Sub, Mul, Div, }

enum Expr {
    Value(i32),
    Binop(Bop, Box<Expr>, Box<Expr>),
}

trait Gear<A, B> {
    fn apply(&mut self, x: A) -> B;
}

mod expr {
    use std::marker::PhantomData;

    pub trait Folder<A> {
        fn value(&mut self, x: &i32) -> A;
        fn binop(&mut self, op: &::Bop, a: A, b: A) -> A;
    }

    struct GearBox<A, F>
        where F : Folder<A> {
        folder : F,
        pd : PhantomData<A>,
    }

    impl<A, F> ::Gear<&::Expr, A> for GearBox<A, F>
        where F : Folder<A> {
        fn apply(&mut self, x : &::Expr) -> A {
            match *x {
                ::Expr::Value(ref v) =>
                    self.folder.value(v),
                ::Expr::Binop(ref op, ref a, ref b) => {
                    let a = self.apply(a);
                    let b = self.apply(b);
                    self.folder.binop(op, a, b)
                }
            }
        }
    }

    pub fn gear<A, F>(folder : F) -> impl for<'a> ::Gear<&'a ::Expr, A>
        where F : Folder<A> {
        GearBox { folder, pd : PhantomData }
    }
}

struct Evaluator();

impl expr::Folder<i32> for Evaluator {

    fn value(&mut self, x: &i32) -> i32 {
        *x
    }

    fn binop(&mut self, op: &::Bop, a: i32, b: i32) -> i32 {
        match op {
            Bop::Add => a + b,
            Bop::Sub => a - b,
            Bop::Mul => a * b,
            Bop::Div => a / b,
        }
    }
}

fn main() {
    let mut eval = expr::gear(Evaluator());

    // 2 * (7 + 1) = 16
    let e = Expr::Binop(
        Bop::Mul,
        Box::new(Expr::Value(2)),
        Box::new(Expr::Binop(
            Bop::Add,
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    println!("result: {}", eval.apply(&e));
}