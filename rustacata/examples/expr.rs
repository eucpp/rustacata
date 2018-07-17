#![feature(proc_macro)]

extern crate rustacata;
extern crate rustacata_macro;

use rustacata::{Transformer, Foldable};
use rustacata_macro::cata;

#[cata]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

fn main() {
//    // 2 * (7 + 1) = 16
    let e = Expr::Mult(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Add(
            Box::new(Expr::Value(7)),
            Box::new(Expr::Value(1))
        ))
    );

    let evaluator = <&Expr as Foldable<i32>>::transformer()
        .with_fold_value(|tr, v| {
            *v
        })
        .with_fold_add(|tr, e1, e2| {
            tr.transform(e1) + tr.transform(e2)
        })
        .with_fold_mult(|tr, e1, e2| {
            tr.transform(e1) * tr.transform(e2)
        });

    println!("result={}", evaluator.transform(&e));
}