# test-macros


Helper macros for righting clean tests


## Example

```rust
use test_macros;

fn setup_db() {
    // Create test data, load env etc.
}

fn cleanup_db() {
    // Delete data from tables, close connections etc.
}

#[test]
#[before(call = "setup_db")]
#[after("cleanup_db")]
fn some_test() {
    // … your stuff goes here …
}
```

## _before_ proc macro


Call a function **BEFORE** a test has run. Useful for e.g. DB setup


### Example

```rust
fn setup_db() -> &'static str {
    // Create test data, load env etc.

    "Return value, stored in variable '_before_values'"
}

#[test]
#[before(call = "setup_db")]
fn some_test() {
    // … your stuff goes here …
}
```

### Variable '_before\_values'


As shown in the _setup_db_ fn example, the return value of the _before_ fn can be used inside
the test by using the variable name __before_values_


```rust
fn setup_db() -> &'static str {
    // Create test data, load env etc.

    "Return value!"
}

#[test]
#[before(call = "setup_db")]
fn some_test() {
    assert_eq!(_before_values, "Return value!") // Asserts positively
}
```


## _after_ proc macro


Call a function **AFTER** a test has run. Useful for e.g. DB cleanup

### Example

```rust
fn cleanup_db() {
    // Delete data from tables, close connections etc.
}

#[test]
#[after(call = "cleanup_db")]
fn some_test() {
    // … your stuff goes here …
}
```


## Use with other macros


In the integration test it came up, the order of macros sometimes matters:


```rust
#[before(call = "before_fn")]
#[after(call = "after_fn")]
#[tokio::test]
async fn some_async_test() {
    // works well
}
```


```rust
#[tokio::test]
#[before(call = "before_fn")]
#[after(call = "after_fn")]
async fn some_async_test() {
    // marks macro 'after' as unreachable
}
```