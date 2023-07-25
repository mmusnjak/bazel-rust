#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let greeter = hello_lib::Greeter::new("Hello");
        assert_eq!(greeter.greeting("world"), "Hello world");
    }
}
