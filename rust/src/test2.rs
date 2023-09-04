use ext_php_rs::php_function;

#[php_function]
fn greet(filename: String) -> String {
    format!("Hello, {}!", filename)
}