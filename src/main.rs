
// Expression AST type
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

trait ExprCata<I, S> {
    fn transform(&self, inh: I, x: &Expr) -> S;
}

// Catamorphism trait for Expression AST (this will be generated automatically in future)
trait ExprTransformer {
    type Inh;
    type Synth;

    fn fold_value<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, v: &i32) -> Self::Synth;
    fn fold_add<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth;
    fn fold_mult<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth;
}

struct ExprCataImpl<Tr>(Tr);

impl<Tr> ExprCata<Tr::Inh, Tr::Synth> for ExprCataImpl<Tr>
where
    Tr : ExprTransformer
{
    fn transform(&self, inh: Tr::Inh, x: &Expr) -> Tr::Synth {
        match x {
            Expr::Value(ref v) => Tr::fold_value(self, inh, v),
            Expr::Add(ref e1, ref e2) => Tr::fold_add(self, inh, e1, e2),
            Expr::Mult(ref e1, ref e2) => Tr::fold_mult(self, inh, e1, e2),
        }
    }
}

// Evaluator is a `newtype` for unit.
// We use it only to derive `impl ExprTransformer` on it.
struct Evaluator(());

impl ExprTransformer for Evaluator {
    type Inh = ();
    type Synth = i32;

    fn fold_value<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, v: &i32) -> Self::Synth {
        *v
    }

    fn fold_add<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        cata.transform(inh, &**e1) + cata.transform(inh, &**e2)
    }

    fn fold_mult<EC: ExprCata<Self::Inh, Self::Synth>>(cata: &EC, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
        cata.transform(inh, &**e1) * cata.transform(inh, &**e2)
    }
}

//// Mapper transformer
//struct Mapper(());
//
//// Default implementation for `map`;
//// We should (somehow) be able to override it partly in other implementations
//impl ExprTransformer for Mapper {
//    type Inh = ();
//    type Synth = Expr;
//
//    fn fold_value(&self, inh: Self::Inh, v: &i32) -> Self::Synth {
//        Expr::Value(*v)
//    }
//
//    fn fold_add(&self, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Expr::Add(
//            Box::new(transform(self, inh, &**e1)),
//            Box::new(transform(self, inh, &**e2)),
//        )
//    }
//
//    fn fold_mult(&self, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Expr::Mult(
//            Box::new(transform(self, inh, &**e1)),
//            Box::new(transform(self, inh, &**e2)),
//        )
//    }
//}
//
//struct IncMapper(Mapper);
//
//impl ExprTransformer for IncMapper {
//    type Inh = <Mapper as ExprTransformer>::Inh;
//    type Synth = <Mapper as ExprTransformer>::Synth;
//
//    fn fold_value(&self, inh: Self::Inh, v: &i32) -> Self::Synth {
//        Expr::Value(*v + 1)
//    }
//
//    fn fold_add(&self, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
////        Expr::Add(transform(self, inh, &**e1), transform(self, inh, &**e2))
////        self.0
//        Mapper::fold_add(&self, inh, e1, e2)
//    }
//
//    fn fold_mult(&self, inh: Self::Inh, e1: &Box<Expr>, e2: &Box<Expr>) -> Self::Synth {
//        Mapper::fold_mult(&self, inh, e1, e2)
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
//    let v = transform(&Evaluator(()), (), &e);
    let v = ExprCataImpl(Evaluator(())).transform((), &e);
    println!("result={}", v);

    // 3 * (8 + 2) = 30
//    let e_inc = transform(&IncMapper(Mapper(())), (), &e);
//    let v_inc = transform(&Evaluator(()), (), &e_inc);
//    println!("result={}", v_inc);
}