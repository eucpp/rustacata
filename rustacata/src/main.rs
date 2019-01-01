// Expression AST type

#[derive(Copy, Clone)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expr<T> {
    Value(T),
    Binop(Bop, Box<Expr<T>>, Box<Expr<T>>),
}

pub trait Gear<A, B> {
    fn apply(&mut self, x: A) -> B;
}

//impl<A, B> Gear<A, B> for Fn(A) -> B {
//    fn apply(&mut self, x: A) -> B {
//        (self)(x)
//    }
//}

impl<A, B, F> Gear<A, B> for F
    where F : FnMut(A) -> B {
    fn apply(&mut self, x: A) -> B {
        (self)(x)
    }
}

mod expr {
    use std::marker::PhantomData;

    pub trait Folder<A> {
        fn value(&mut self, a: A) -> A;
        fn binop(&mut self, op: ::Bop, a: A, b: A) -> A;
    }

    struct GearBox<A, F, T, GT>
    where
        F: Folder<A>,
        GT: for <'a> ::Gear<&'a T, A>,
    {
        folder: F,
        t_gear: GT,
        pd: PhantomData<(A, T)>,
    }

    impl<A, F, T, GT> ::Gear<&::Expr<T>, A> for GearBox<A, F, T, GT>
        where
            F: Folder<A>,
            GT: for <'a> ::Gear<&'a T, A>,
    {
        fn apply(&mut self, x: &::Expr<T>) -> A {
            match *x {
                ::Expr::Value(ref v) => {
                    let a = self.t_gear.apply(v);
                    self.folder.value(a)
                }
                ::Expr::Binop(op, ref a, ref b) => {
                    let a = self.apply(a);
                    let b = self.apply(b);
                    self.folder.binop(op, a, b)
                }
            }
        }
    }

    pub fn gear<A, F, T, GT>(folder: F, t_gear : GT) -> impl for<'a> ::Gear<&'a ::Expr<T>, A>
    where
        F: Folder<A>,
        GT: for<'a> ::Gear<&'a T, A>,
    {
        GearBox {
            folder,
            t_gear,
            pd: PhantomData,
        }
    }
}

struct Evaluator();

struct Mapper1 {

}

impl expr::Folder<i32> for Evaluator {
    fn value(&mut self, x: i32) -> i32 {
        x
    }

    fn binop(&mut self, op: Bop, a: i32, b: i32) -> i32 {
        match op {
            Bop::Add => a + b,
            Bop::Sub => a - b,
            Bop::Mul => a * b,
            Bop::Div => a / b,
        }
    }
}

fn main() {
    let mut eval = expr::gear(Evaluator(), |x: &i32| *x);

    // 2 * (7 + 1) = 16
    let e = Expr::Binop(
        Bop::Mul,
        Box::new(Expr::Value(2)),
        Box::new(Expr::Binop(
            Bop::Add,
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1)),
        )),
    );

    println!("result: {}", eval.apply(&e));
}
