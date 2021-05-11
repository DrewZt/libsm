#![feature(test)]
extern crate test;
extern crate num_bigint;

use test::Bencher;
use num_bigint::BigUint;

#[bench]
fn libsm_verify_bench(bench: &mut Bencher) {
    let test_word = b"hello world";
    let ctx = libsm::sm2::signature::SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    let sig = ctx.sign(test_word, &sk, &pk);

    bench.iter(|| {
        let _ = ctx.verify(test_word, &pk, &sig);
    });
}

#[bench]
fn libsm_sign_bench(bench: &mut Bencher) {
    let test_word = b"hello world";
    let ctx = libsm::sm2::signature::SigCtx::new();
    let (pk, sk) = ctx.new_keypair();

    bench.iter(|| {
        let _ = ctx.sign(test_word, &sk, &pk);
    });
}

pub fn test1(test_word:&[u8], sku:&[u8]) -> String {

    let ctx = libsm::sm2::signature::SigCtx::new();
    let curve = libsm::sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(sku);
    let pk = curve.g_mul(&sk);
    let res = ctx.sign(test_word, &sk, &pk);
    let mut r = format!("{:0>64}", res.get_r().to_str_radix(16));
    let s = &format!("{:0>64}",res.get_s().to_str_radix(16));
    let pkn = &curve.to_affine(&pk);
    r.push_str(&s);
    r.push_str(&format!("{:0>64}", pkn.0.to_str(16)));
    r.push_str(&format!("{:0>64}", pkn.1.to_str(16)));
    return r

}

#[test]
fn sign_test(){
    let test_word = b"hello world";
    let sku:&[u8] = &[2, 54, 56, 100, 101, 53, 55, 49, 48, 100, 54, 54, 49, 57, 53, 101, 50, 98, 97, 99,
        100, 57, 57, 52, 98, 49, 52, 48, 56, 100, 52, 101];
    let res = test1(test_word,sku);
    println!("sig: {:?}", res);
}

#[bench]
fn sign_bench(bench: &mut Bencher){
    let test_word = &[2, 54, 56, 100, 101, 53, 55, 49, 48, 100, 54, 54, 49, 57, 53, 101, 50, 98, 97, 99,
        100, 57, 57, 52, 98, 49, 52, 48, 56, 100, 52, 101];
    let sku:&[u8] = &[2, 54, 56, 100, 101, 53, 55, 49, 48, 100, 54, 54, 49, 57, 53, 101, 50, 98, 97, 99,
        100, 57, 57, 52, 98, 49, 52, 48, 56, 100, 52, 101];
    bench.iter(|| {
        let _ = test1(test_word, sku);
    });
}