mod dummy_book;

use crate::dummy_book::DummyBook;

use moenarchbook::MDBook;

#[test]
fn mdbook_can_correctly_test_a_passing_book() {
    let temp = DummyBook::new().with_passing_test(true).build().unwrap();
    let mut md = MDBook::load(temp.path()).unwrap();

    let result = md.test(vec![]);
    assert!(
        result.is_ok(),
        "Tests failed with {}",
        result.err().unwrap()
    );
}

#[test]
fn mdbook_detects_book_with_failing_tests() {
    let temp = DummyBook::new().with_passing_test(false).build().unwrap();
    let mut md = MDBook::load(temp.path()).unwrap();

    assert!(md.test(vec![]).is_err());
}
