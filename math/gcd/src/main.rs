fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn fib(n: u64) -> u64 {
    let (mut a, mut b) = (1, 1);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }

    a
}

#[cfg(test)]
mod tests {
    use crate::{fib, gcd};

    #[test]
    fn fib_test() {
        assert_eq!(fib(10), 55);
        assert_eq!(fib(60), 1548008755920);
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(fib(32), fib(31)), 1);
    }
}

fn main() {
    println!("{}", gcd(fib(32), fib(31)));
}
