// Unit Tests

use crate::spread_sheets;
use rust_test_lab::Rectangle;

// #[test]
fn exploration() {
    let result = rust_test_lab::add(2, 2);
    assert_eq!(result, 4);
}

// #[test]
fn another() {
    panic!("Make this test fail");
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
#[should_panic(expected = "Correct Path")]
fn wrong_path() {
    spread_sheets::create_or_get_workbook("fzefezz").unwrap();
}

#[test]
fn greeting_contains_name() {
    let result = rust_test_lab::greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}

#[test]
fn it_works() -> Result<(), String> {
    let result = rust_test_lab::add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
