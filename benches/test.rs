#[test]
fn libsm_sign_test() {
    let test_word = b"hello world";
    let ctx = libsm::sm2::signature::SigCtx::new();
    let (pk, sk) = ctx.new_keypair();

    let a = ctx.sign(test_word, &sk, &pk);
    println!("{}", a.get_r().to_str_radix(16));
}