// Expression AST type

#[derive(Copy, Clone, Debug)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr<T> {
    Value(T),
    Binop(Bop, Box<Expr<T>>, Box<Expr<T>>),
}

pub trait Gear<A, B> {
    fn apply(&mut self, x: A) -> B;
}

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

    struct Fused<A, F, GA>
        where
            F: Folder<A>,
            GA: for<'a> ::Gear<&'a A, A>,
    {
        folder: F,
        a_gear: GA,
        pd: PhantomData<A>,
    }

    impl<A, F, GA> Folder<A> for Fused<A, F, GA>
        where
            F: Folder<A>,
            GA: for<'a> ::Gear<&'a A, A>,
    {
        fn value(&mut self, a: A) -> A {
            self.a_gear.apply(
                &self.folder.value(a)
            )
        }

        fn binop(&mut self, op: ::Bop, a: A, b: A) -> A {
            self.a_gear.apply(
                &self.folder.binop(op, a, b)
            )
        }
    }

    pub fn fuse<A, F, GA>(folder: F, a_gear : GA) -> impl Folder<A>
        where
            F: Folder<A>,
            GA: for<'a> ::Gear<&'a A, A>,
    {
        Fused {
            folder,
            a_gear,
            pd: PhantomData,
        }
    }
}

struct Evaluator();

struct Mapper1();
struct Mapper2();

impl<T> expr::Folder<Expr<T>> for Mapper1
    where
        T: std::fmt::Debug
    {
        fn value(&mut self, a: ::Expr<T>) -> ::Expr<T> {
            println!("Mapper1::value({:?})", a);
            a
        }

        fn binop(&mut self, op: ::Bop, a: ::Expr<T>, b: ::Expr<T>) -> ::Expr<T> {
            println!("Mapper1::binop({:?}, {:?}, {:?})", op, a, b);
            ::Expr::Binop(op, Box::new(a), Box::new(b))
        }
    }

impl<T> expr::Folder<Expr<T>> for Mapper2
    where
        T: std::fmt::Debug
    {
        fn value(&mut self, a: ::Expr<T>) -> ::Expr<T> {
            println!("Mapper2::value({:?})", a);
            a
        }

        fn binop(&mut self, op: ::Bop, a: ::Expr<T>, b: ::Expr<T>) -> ::Expr<T> {
            println!("Mapper2::binop({:?}, {:?}, {:?})", op, a, b);
            ::Expr::Binop(op, Box::new(a), Box::new(b))
        }
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

    let mut map1 = expr::gear(Mapper1(), |x: &i32| { Expr::Value(*x) });
    let mut map2 = expr::gear(Mapper2(), |x: &i32| { Expr::Value(*x) });

    println!("Map1");
    map1.apply(&e);

    println!("Map2");
    map2.apply(&e);

    let mut fused = expr::gear(expr::fuse(Mapper1(), map2), |x: &i32| { Expr::Value(*x) });

    println!("Fused");
    fused.apply((&e));
}
