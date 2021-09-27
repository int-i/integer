pub mod members;

#[get("/index")]
pub fn index() -> &'static str {
    "Hello, world!"
}
