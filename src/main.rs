
// Expression AST type
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

trait Transformer<T, I, S> {
    fn transform(&self, inh: I, x: &T) -> S;
}

trait ExprTransformation {
    type Inh;
    type Synth;

    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth;
    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth;
    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth;
}

struct ExprTransformer<Tm>();

impl<Tm> Transformer<Expr, Tm::Inh, Tm::Synth> for ExprTransformer<Tm>
where
    Tm : ExprTransformation
{
    fn transform(&self, inh: Tm::Inh, x: &Expr) -> Tm::Synth {
        match x {
            Expr::Value(ref v) => Tm::fold_value(self, inh, v),
            Expr::Add(ref e1, ref e2) => Tm::fold_add(self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => Tm::fold_mult(self, inh, e1, e2),
        }
    }
}

struct Evaluator(());

impl ExprTransformation for Evaluator {
    type Inh = ();
    type Synth = i32;

    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth {
        *v
    }

    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
    }

    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
    }
}

// Mapper transformer
struct Mapper(());

// Default implementation for `map`;
// We should (somehow) be able to override it partly in other implementations
impl ExprTransformation for Mapper {
    type Inh = ();
    type Synth = Expr;

    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth {
        Expr::Value(*v)
    }

    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        Expr::Add(
            Box::new(tr.transform(inh, &**e1)),
            Box::new(tr.transform(inh, &**e2)),
        )
    }

    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        Expr::Mult(
            Box::new(tr.transform(inh, &**e1)),
            Box::new(tr.transform(inh, &**e2)),
        )
    }
}

struct IncMapper(Mapper);

impl ExprTransformation for IncMapper {
    type Inh = <Mapper as ExprTransformation>::Inh;
    type Synth = <Mapper as ExprTransformation>::Synth;

    fn fold_value<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, v: &i32) -> Self::Synth {
        Expr::Value(*v + 1)
    }

    fn fold_add<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        Mapper::fold_add(tr, inh, e1, e2)
    }

    fn fold_mult<Tr: Transformer<Expr, Self::Inh, Self::Synth>>(tr: &Tr, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        Mapper::fold_mult(tr, inh, e1, e2)
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
//    let v = transform(&Evaluator(()), (), &e);
    let v = ExprTransformer
        (Evaluator(())).transform((), &e);
    println!("result={}", v);

    // 3 * (8 + 2) = 30
    let e_inc = ExprTransformer(IncMapper(Mapper(()))).transform((), &e);
    let v_inc = ExprTransformer(Evaluator(())).transform((), &e_inc);
    println!("result={}", v_inc);
}