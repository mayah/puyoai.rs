#![feature(test)]

extern crate puyoai_core;
extern crate test;

use test::Bencher;
use puyoai_core::field::BitField;

#[bench]
fn simulate_19rensa(b: &mut Bencher) {
    let bf = BitField::from_str(concat!(
        ".G.BRG",
        "GBRRYR",
        "RRYYBY",
        "RGYRBR",
        "YGYRBY",
        "YGBGYR",
        "GRBGYR",
        "BRBYBY",
        "RYYBYY",
        "BRBYBR",
        "BGBYRR",
        "YGBGBG",
        "RBGBGG"));

    b.iter(|| {
        test::black_box(bf.clone().simulate())
    })
}

#[bench]
fn simulate_fast_19rensa(b: &mut Bencher) {
    let bf = BitField::from_str(concat!(
        ".G.BRG",
        "GBRRYR",
        "RRYYBY",
        "RGYRBR",
        "YGYRBY",
        "YGBGYR",
        "GRBGYR",
        "BRBYBY",
        "RYYBYY",
        "BRBYBR",
        "BGBYRR",
        "YGBGBG",
        "RBGBGG"));

    b.iter(|| {
        test::black_box(bf.clone().simulate_fast())
    })
}
