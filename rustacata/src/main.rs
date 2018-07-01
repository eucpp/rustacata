#![feature(proc_macro)]

//extern crate rustacata_macro;

//use rustacata_macro::derive_transformer;

trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: T) -> S;
}

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

impl<'a, I, S> Default for ExprFold<'a, I, S> {
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

impl<'a, I, S> Default for ExprFoldMut<'a, I, S> {
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

impl<'a> Default for Evaluator<'a> {
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

struct Map<'a, I>(ExprFold<'a, I, Expr>);

impl<'a, I: Copy> Default for Map<'a, I> {
    fn default() -> Self {
        Map(ExprFold::default()
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

impl<'a, I> Map<'a, I> {
    fn with_map_value<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b i32) -> Expr
    {
        Map(ExprFold { fold_value: Box::new(f), ..self.0 })
    }

    fn with_map_add<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b Box<Expr>, &'b Box<Expr>) -> Expr
    {
        Map(ExprFold { fold_add: Box::new(f), ..self.0 })
    }

    fn with_map_mult<'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + for<'b> Fn(&Transformer<&'b Expr, I, Expr>, I, &'b Box<Expr>, &'b Box<Expr>) -> Expr
    {
        Map(ExprFold { fold_mult: Box::new(f), ..self.0 })
    }

    fn map(&self, inh: I, e: &Expr) -> Expr {
        self.0.transform(inh, e)
    }
}

struct IterMut<'a, I>(ExprFoldMut<'a, I, ()>);

impl<'a, I: Copy> Default for IterMut<'a, I> {
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

    let inc_mapper = Map::default().with_map_value(
        |tr, inh, v| {
            Expr::Value(*v + 1)
        });
    let e_inc = inc_mapper.map((), &e);
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