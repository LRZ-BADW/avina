#[macro_use]
extern crate bencher;

use std::{env, str::FromStr};

use avina::{Api, Token};
use bencher::Bencher;
use futures::executor::block_on;

fn bench_hello_user(b: &mut Bencher) {
    let token =
        Token::from_str(env::var("OS_TOKEN").unwrap().as_str()).unwrap();
    let api =
        Api::new("http://localhost:8000/api".to_string(), token, None, None)
            .unwrap();

    b.iter(|| {
        block_on(api.hello.user()).unwrap();
    });
}

benchmark_group!(benches, bench_hello_user);
benchmark_main!(benches);
