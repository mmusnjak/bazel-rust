#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let greeter = hello_lib::Greeter::new("Hello");
        assert_eq!(greeter.greeting("world"), "Hello world");
    }

    #[test]
    fn greeting_id_is_a_uuid_v7() {
        let greeter = hello_lib::Greeter::new("Hello");
        let id = greeter.greeting_id();
        assert_eq!(id.get_version_num(), 7);
    }

    #[test]
    fn different_greetings_scope_to_different_ids() {
        let hello = hello_lib::Greeter::new("Hello");
        let goodbye = hello_lib::Greeter::new("Goodbye");
        assert_ne!(hello.greeting_id(), goodbye.greeting_id());
    }

    #[test]
    fn greeting_id_encoded_round_trips_through_crockford_base32() {
        let greeter = hello_lib::Greeter::new("Hello");
        let id = greeter.greeting_id();
        let encoded = hello_lib::Greeter::encode(&id);

        assert_eq!(encoded.len(), 26);
        assert!(encoded.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));

        let decoded = base32::decode(base32::Alphabet::Crockford, &encoded).unwrap();
        assert_eq!(decoded, id.as_bytes());
    }
}
