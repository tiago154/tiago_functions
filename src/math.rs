pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn square_root(n: f64) -> f64 {
    n.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(29), true);
        assert_eq!(is_prime(30), false);
    }

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(16.0), 4.0);
        assert_eq!(square_root(25.0), 5.0);
        assert_eq!(square_root(9.0), 3.0);
        assert_eq!(square_root(2.0), 2.0_f64.sqrt());
    }
}
