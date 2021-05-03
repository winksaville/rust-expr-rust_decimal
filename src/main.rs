use rust_decimal::prelude::*;

use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MarketLotSize {
    step_size: Decimal,
}

fn main() {
    let dn = dec!(-1.23);
    println!("dn: {}", dn);

    const DEC_DATA: &str = r#"{ "step_size": "0.000001" }"#;
    let mls: MarketLotSize = serde_json::from_str(DEC_DATA).unwrap();
    println!("mls: {:?}", mls);

    //let q = dec!(1.0000009);
    let q= dec!(1.0000014999999999);
    let rq = q + (mls.step_size / dec!(2));
    let rq_mss = rq % mls.step_size;
    let aq = rq - rq_mss;
    println!("q: {} aq: {} rq: {} rq_mss: {}", q, aq, rq, rq_mss);

    let d = aq - mls.step_size;
    let md = d % mls.step_size;
    println!("d: {} md: {}", d, md);
}
