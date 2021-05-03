# Experiment with rust_decimal

The [rust_decimal](https://crates.io/crates/rust_decimal) module provides
a Decimal type suitable for finicial calculations. I wanted to do some
quick tests before trying to use it in the binance api client code.

Seems to work :)

## Run

```
wink@3900x:~/prgs/rust/projects/expr-rust_decimal (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/expr-decimal`
dn: -1.23
mls: MarketLotSize { step_size: 0.000001 }
q: 1.0000014999999999 aq: 1.0000010000000000 rq: 1.0000019999999999 rq_mss: 0.0000009999999999
d: 1.0000000000000000 md: 0.0000000000000000
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
