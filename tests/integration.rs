use test_macros::{after, before};

fn before_fn() -> &'static str {
    println!("Executed before");

    "Before value!"
}

fn after_fn() {
    println!("Executed after");
}

#[test]
#[before(call = "before_fn")]
#[after(call = "after_fn")]
fn some_test() {
    assert_eq!("Before value!", _before_values);
    assert!(true);
}

#[before(call = "before_fn")]
#[after(call = "after_fn")]
#[tokio::test]
async fn some_async_test() {
    assert_eq!("Before value!", _before_values);
    assert!(true);
}