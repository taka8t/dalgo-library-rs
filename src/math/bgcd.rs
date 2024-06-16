// 0 <= a, b <= i64::MAX
fn bgcd(a: u64, b: u64) -> u64 {
    if a == 0 {return b;}
    if b == 0 {return a;}
    let mut a = a as i64;
    let mut b = b as i64;

    let mut tza = a.trailing_zeros();
    let tzb = b.trailing_zeros();
    let tzm = tza.min(tzb);
    b >>= tzb;

    while a != 0 {
        a >>= tza;
        let diff = a - b;
        tza = diff.trailing_zeros();
        b = a.min(b);
        a = diff.abs();
    }
    return (b << tzm) as u64;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bgcd(){
        assert_eq!(2, bgcd(54, 20));
        assert_eq!(21, bgcd(147, 105));
        assert_eq!(2*3*7*13*19, bgcd(2*3*7*13*19*1000000007, 2*3*7*13*19*1000000009));
        assert_eq!(7*13*19*1000000007, bgcd(1303*2*3*7*13*19*1000000007, 29*7*13*19*1000000007));
        assert_eq!(19*1000000007, bgcd(19*1000000007, 0));
        assert_eq!(1, bgcd(63245986, 102334155));
    }
}