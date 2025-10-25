// src/tests/git/rev_parse.rs

use crate::tests::mock::MockBackend;
use crate::git::rev_parse::get_current_branch;

#[test]
fn test_get_current_branch_main() {
    let backend = MockBackend {
        branch_name: String::from("main"),
    };

    let result = get_current_branch(&backend);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "main");
}

#[test]
fn test_get_current_branch_feature() {
    let backend = MockBackend {
        branch_name: String::from("feature/foo"),
    };

    assert_eq!(get_current_branch(&backend).unwrap(), "feature/foo");
}
