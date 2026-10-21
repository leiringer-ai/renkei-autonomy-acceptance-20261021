/// Return the next i64, saturating at the maximum value.
pub fn successor(value: i64) -> i64 { value.saturating_add(1) }

/// Return the previous i64, saturating at the minimum value.
pub fn predecessor(value: i64) -> i64 {
    value.saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::predecessor;

    #[test]
    fn predecessor_decrements_positive_negative_and_zero_values() {
        assert_eq!(predecessor(42), 41);
        assert_eq!(predecessor(-42), -43);
        assert_eq!(predecessor(0), -1);
    }

    #[test]
    fn predecessor_saturates_at_minimum() {
        assert_eq!(predecessor(i64::MIN), i64::MIN);
        assert_eq!(predecessor(i64::MIN + 1), i64::MIN);
    }

    #[test]
    fn predecessor_decrements_maximum() {
        assert_eq!(predecessor(i64::MAX), i64::MAX - 1);
    }
}
