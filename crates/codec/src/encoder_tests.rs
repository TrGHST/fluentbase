use crate::encoder::{ALIGN_32, ALIGN_DEFAULT};
use crate::if_align_default_then;

#[test]
fn test_if_align_default_then() {
    if_align_default_then!(ALIGN_DEFAULT, { assert!(true) }, { assert!(false) });
    if_align_default_then!(ALIGN_32, { assert!(false) }, { assert!(true) });
}
