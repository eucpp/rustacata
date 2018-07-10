extern crate rustacata_example_eval;

use rustacata_example_eval::Expr;
use rustacata_example_eval::Foldable;
use rustacata_example_eval::Transformer;

fn main() {
//    // 2 * (7 + 1) = 16
    let e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let eval = <Expr as Foldable<(), i32>>::transformer()
        .with_fold_value(|tr, inh, v| {
            *v
        })
        .with_fold_add(|tr, inh, e1, e2| {
            tr.transform(inh, &**e1) + tr.transform(inh, &**e2)
        })
        .with_fold_mult(|tr, inh, e1, e2| {
            tr.transform(inh, &**e1) * tr.transform(inh, &**e2)
        });


    let v = eval.transform((), &e);
    println!("result={}", v);
}