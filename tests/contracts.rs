use renkei_autonomy_fixture::{labels::label, successor::successor};
#[test] fn next_integer() { assert_eq!(successor(41),42); assert_eq!(successor(-1),0); }
#[test] fn maximum_stays_maximum() { assert_eq!(successor(i64::MAX),i64::MAX); }
#[test] fn labels_preserve_content() { assert_eq!(label("x"),"item:x"); assert_eq!(label(""),"item:"); }
