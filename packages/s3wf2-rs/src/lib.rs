pub mod document;
pub mod error;
pub mod parser;
pub mod emitter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
