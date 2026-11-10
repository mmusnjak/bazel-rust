mod uuidv7;

pub use uuid::Uuid;

pub struct Greeter {
    greeting: String,
}

impl Greeter {
    pub fn new(greeting: &str) -> Greeter {
        Greeter {
            greeting: greeting.to_string(),
        }
    }

    pub fn greet(&self, thing: &str) {
        println!("{} {} [{}]", self.greeting, thing, self.greeting_id_encoded());
    }

    pub fn greeting(&self, thing: &str) -> String {
        format!("{} {}", self.greeting, thing)
    }

    /// A UUIDv7 scoped to this greeter's greeting, after the masked-timestamp
    /// scheme in https://brooker.co.za/blog/2025/10/22/uuidv7.html. The
    /// greeting is the scope `id` the article's hash formula takes.
    pub fn greeting_id(&self) -> Uuid {
        uuidv7::scoped_v7(&self.greeting)
    }

    /// `greeting_id`, as a 26-character Crockford Base32 string: no hyphens,
    /// case-insensitive, and safe for URLs, headers, and logs.
    pub fn greeting_id_encoded(&self) -> String {
        Self::encode(&self.greeting_id())
    }

    /// Encodes any UUID as lowercase Crockford Base32, the same way
    /// `greeting_id_encoded` does. Crockford Base32 treats case as
    /// insignificant, so a decoder reads this the same as the uppercase form.
    pub fn encode(id: &Uuid) -> String {
        base32::encode(base32::Alphabet::Crockford, id.as_bytes()).to_lowercase()
    }
}
