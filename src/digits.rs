pub trait Digits<T> {
    type I: Iterator<Item = T>;

    fn digit_count(self) -> T;
    fn nth_digit(self, d: T) -> T;
    fn digits(self) -> Self::I;
}

impl Digits<u64> for u64 {
    type I = impl Iterator<Item = Self>;

    fn digit_count(self) -> Self { ((self as f64).log10() as Self) + 1 }

    fn nth_digit(self, d: Self) -> Self { self % (10u64.pow(d as u32 + 1)) / (10u64.pow(d as u32)) }

    fn digits(self) -> Self::I { (0..self.digit_count()).map(move |d| self.nth_digit(d)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_digits() {
        assert_eq!(0.digit_count(), 1);
        assert_eq!(1.digit_count(), 1);
        assert_eq!(9.digit_count(), 1);
        assert_eq!(1_000_000_000.digit_count(), 10);
    }

    #[test]
    fn test_digits() {
        assert_eq!(0.digits().collect::<Vec<_>>(), [0]);
        assert_eq!(9.digits().collect::<Vec<_>>(), [9]);
        assert_eq!(1_234_567_890.digits().collect::<Vec<_>>(), [0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
