#![feature(proc_macro)]

//extern crate rustacata_macro;

//use rustacata_macro::derive_transformer;

trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: T) -> S;
}

trait Foldable<I, S> : Sized {
    type Tr: for<'a> Transformer<&'a Self, I, S>;

    fn transformer() -> Self::Tr;
}

trait Foldable1<T, TS, I, S> : Sized {
    type Tr: for<'a> Transformer<&'a Self, I, S>;

    fn transformer(tr: & for<'a> Transformer<&'a T, I, TS>) -> Self::Tr;
}

trait Mappable<I> : Sized {
    type Tr: for<'a> Transformer<&'a Self, I, Self>;

    fn transformer() -> Self::Tr;
}

struct OptionFold<'a, T, TS, I, S> {
    fold_t : Box<'a + for<'b> Transformer<&'b T, I, TS>>,
    fold_some : Box<'a + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I, &'b T) -> S>,
    fold_none : Box<'a + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, TS>, I) -> S>,
}

impl<'a, T, TS, I, S> OptionFold<'a, T, TS, I, S> {

    fn with_fold_some<'c: 'a, F>(self, f: F) -> Self
    where
        F: 'c + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, S>, I, &'b T) -> S
    {
        OptionFold { fold_some: Box::new(f), ..self }
    }

    fn with_fold_none<'c: 'a, F>(self, f: F) -> Self
    where
        F: 'c + for<'b> Fn(&Transformer<&'b Option<T>, I, S>, &Transformer<&'b T, I, S>, I) -> S
{
    OptionFold { fold_none: Box::new(f), ..self }
}
}

impl<'a, 'b, T, I, S> Transformer<&'b Option<T>, I, S> for OptionFold<'a, T, I, S> {
    fn transform(&self, inh: I, x: &'b Option<T>) -> S {
        match x {
            Some(ref t) => (self.fold_some)(&*self, inh, t),
            None => (self.fold_none)(&*self, inh),
        }
    }
}

impl<T, TS, I, S> Foldable1<T, TS, I, S> for Option<T> {
    type Tr = OptionFold<'static, T, TS, I, S>;

    fn transformer() -> Self::Tr {
        OptionFold {
            fold_some: Box::new(|tr, inh, v| unimplemented!()),
            fold_none: Box::new(|tr, inh, v| unimplemented!()),
        }
    }
}

//Option<T>

// Expression AST type
//#[derive_transformer]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

struct ExprFold<'a, I, S> {
    fold_value : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S>,
    fold_add : Box<'a + for <'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
    fold_mult : Box<'a + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S>,
}

impl<'a, I, S> ExprFold<'a, I, S> {

    fn with_fold_value<'c: 'a, F>(self, f: F) -> Self
    where
        F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b i32) -> S
    {
        ExprFold { fold_value: Box::new(f), ..self }
    }

    fn with_fold_add<'c: 'a, F>(self, f: F) -> Self
    where
        F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
    {
        ExprFold { fold_add: Box::new(f), ..self }
    }

    fn with_fold_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, S>, I, &'b Box<Expr>, &'b Box<Expr>) -> S
    {
        ExprFold { fold_mult: Box::new(f), ..self }
    }
}

impl<I, S> Default for ExprFold<'static, I, S> {
    fn default() -> Self {
        ExprFold {
            fold_value: Box::new(|tr, inh, v| unimplemented!()),
            fold_add: Box::new(|tr, inh, e1, e2| unimplemented!()),
            fold_mult: Box::new(|tr, inh, e1, e2| unimplemented!()),
        }
    }
}

impl<'a, 'b, I, S> Transformer<&'b Expr, I, S> for ExprFold<'a, I, S> {
    fn transform(&self, inh: I, x: &'b Expr) -> S {
        match x {
            Expr::Value(ref v) => (self.fold_value)(&*self, inh, v),
            Expr::Add(ref e1, ref e2) => (self.fold_add)(&*self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => (self.fold_mult)(&*self, inh, e1, e2),
        }
    }
}

struct ExprFoldMut<'a, I, S> {
    fold_value : Box<'a + for<'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut i32) -> S>,
    fold_add : Box<'a + for <'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> S>,
    fold_mult : Box<'a + for<'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> S>,
}

impl<'a, I, S> ExprFoldMut<'a, I, S> {

    fn with_fold_value<'c: 'a, F>(self, f: F) -> Self
    where
        F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut i32) -> S
    {
        ExprFoldMut { fold_value: Box::new(f), ..self }
    }

    fn with_fold_add<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> S
    {
        ExprFoldMut { fold_add: Box::new(f), ..self }
    }

    fn with_fold_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, S>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> S
    {
        ExprFoldMut { fold_mult: Box::new(f), ..self }
    }
}

impl<I, S> Default for ExprFoldMut<'static, I, S> {
    fn default() -> Self {
        ExprFoldMut {
            fold_value: Box::new(|tr, inh, v| unimplemented!()),
            fold_add: Box::new(|tr, inh, e1, e2| unimplemented!()),
            fold_mult: Box::new(|tr, inh, e1, e2| unimplemented!()),
        }
    }
}

impl<'a, 'b, I, S> Transformer<&'b mut Expr, I, S> for ExprFoldMut<'a, I, S> {
    fn transform(&self, inh: I, x: &'b mut Expr) -> S {
        match x {
            Expr::Value(ref mut v) => (self.fold_value)(&*self, inh, v),
            Expr::Add(ref mut e1, ref mut e2) => (self.fold_add)(&*self, inh, e1, e2),
            Expr::Mult(ref mut e1, ref mut e2) => (self.fold_mult)(&*self, inh, e1, e2),
        }
    }
}

struct Evaluator<'a>(ExprFold<'a, (), i32>);

impl<'a> Evaluator<'a> {
    fn eval<'b>(&self, e: &'b Expr) -> i32 {
        self.0.transform((), e)
    }
}

impl Default for Evaluator<'static> {
    fn default() -> Self {
        Evaluator(ExprFold::default()
            .with_fold_value(|tr, inh, v| {
                *v
            })
            .with_fold_add(|tr, inh, e1, e2| {
                tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
            })
            .with_fold_mult(|tr, inh, e1, e2| {
                tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
            }))
    }
}

struct ExprMap<'a, I>(ExprFold<'a, I, Expr>);

impl<I: Copy> Default for ExprMap<'static, I> {
    fn default() -> Self {
        ExprMap(ExprFold::default()
            .with_fold_value(|tr, inh, v| {
                Expr::Value(*v)
            })
            .with_fold_add(|tr, inh, e1, e2| {
                Expr::Add(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            })
            .with_fold_mult(|tr, inh, e1, e2| {
                Expr::Mult(
                    Box::new(tr.transform(inh, &**e1)),
                    Box::new(tr.transform(inh, &**e2)),
                )
            })
        )
    }
}

impl<'a, I> ExprMap<'a, I> {
    fn with_map_value<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b i32) -> Expr
    {
        ExprMap(ExprFold { fold_value: Box::new(f), ..self.0 })
    }

    fn with_map_add<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b Box<Expr>, &'b Box<Expr>) -> Expr
    {
        ExprMap(ExprFold { fold_add: Box::new(f), ..self.0 })
    }

    fn with_map_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b Box<Expr>, &'b Box<Expr>) -> Expr
    {
        ExprMap(ExprFold { fold_mult: Box::new(f), ..self.0 })
    }
}

impl<'a, 'b, I> Transformer<&'b Expr, I, Expr> for ExprMap<'a, I> {
    fn transform(&self, inh: I, x: &'b Expr) -> Expr {
        self.0.transform(inh, x)
    }
}

impl<I: Copy> Mappable<I> for Expr {
    type Tr = ExprMap<'static, I>;

    fn transformer() -> Self::Tr {
        ExprMap::default()
    }
}

struct IterMut<'a, I>(ExprFoldMut<'a, I, ()>);

impl<I: Copy> Default for IterMut<'static, I> {
    fn default() -> Self {
        IterMut(ExprFoldMut::default()
            .with_fold_value(|tr, inh, v| {
                ()
            })
            .with_fold_add(|tr, inh, e1, e2| {
                tr.transform(inh, &mut **e1);
                tr.transform(inh, &mut **e2);
            })
            .with_fold_mult(|tr, inh, e1, e2| {
                tr.transform(inh, &mut **e1);
                tr.transform(inh, &mut **e2);
            })
        )
    }
}

impl<'a, I> IterMut<'a, I> {
    fn with_iter_value<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, ()>, I, &'b mut i32) -> ()
    {
        IterMut(ExprFoldMut { fold_value: Box::new(f), ..self.0 })
    }

    fn with_iter_add<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, ()>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> ()
    {
        IterMut(ExprFoldMut { fold_add: Box::new(f), ..self.0 })
    }

    fn with_iter_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b mut Expr, I, ()>, I, &'b mut Box<Expr>, &'b mut Box<Expr>) -> ()
    {
        IterMut(ExprFoldMut { fold_mult: Box::new(f), ..self.0 })
    }

    fn iter(&self, inh: I, e: &mut Expr) -> () {
        self.0.transform(inh, e)
    }
}

fn main() {
//    // 2 * (7 + 1) = 16
    let mut e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let evaluator = Evaluator::default();

    let v = evaluator.eval(&e);
    println!("result={}", v);

//    let inc_mapper = ExprMap::default()
    let inc_mapper = <Expr as Mappable<()>>::transformer()
        .with_map_value(
            |tr, inh, v| {
                Expr::Value(*v + 1)
            });
    let e_inc = inc_mapper.transform((), &e);
    let v_inc = evaluator.eval(&e_inc);
    println!("result={}", v_inc);

    let inc_iter = IterMut::default().with_iter_value(
        |tr, inh, v| {
            *v = *v + 1;
        });
    inc_iter.iter((), &mut e);
    let v_inc = evaluator.eval(&e);
    println!("result={}", v_inc);
}