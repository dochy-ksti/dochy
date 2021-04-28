use std::time::{Instant, UNIX_EPOCH};
use dochy::error::DpResult;

#[test]
fn temporal_test() -> DpResult<()>{
    let mut v : Vec<u128> = vec![];
    for i in 0..10 {
        let now = std::time::SystemTime::now();
        let d = now.duration_since(UNIX_EPOCH)?;
        v.push(d.as_nanos());
    }
    println!("{:?}", v);
    println!("{}", u64::MAX);
    Ok(())
}