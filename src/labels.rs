/// Format the label as item:<value>.
pub fn label(value: &str) -> String { format!("item:{value}") }

/// Format two labels in order, preserving their contents exactly.
pub fn label_pair(left: &str, right: &str) -> String {
    format!("item:{left}|item:{right}")
}

#[cfg(test)]
mod tests {
    use super::label_pair;

    #[test]
    fn preserves_distinct_inputs_in_order() {
        assert_eq!(label_pair("left", "right"), "item:left|item:right");
        assert_eq!(label_pair("right", "left"), "item:right|item:left");
    }

    #[test]
    fn supports_both_empty_inputs() {
        assert_eq!(label_pair("", ""), "item:|item:");
    }

    #[test]
    fn supports_each_empty_side() {
        assert_eq!(label_pair("", "right"), "item:|item:right");
        assert_eq!(label_pair("left", ""), "item:left|item:");
    }

    #[test]
    fn preserves_unicode_on_both_sides() {
        assert_eq!(label_pair("雪🌸e\u{301}", "café🦀"), "item:雪🌸e\u{301}|item:café🦀");
    }

    #[test]
    fn preserves_delimiters_whitespace_newlines_and_nuls() {
        assert_eq!(
            label_pair(" |:\t\n\0 ", "\0\n\r\t:| "),
            "item: |:\t\n\0 |item:\0\n\r\t:| "
        );
    }
}
