
pub struct UserCollection {
    users: [&'static str; 3],
}

impl UserCollection {
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Carl"],
        }
    }
    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self
        }
    }
}

pub struct UserIterator<'a> {
    index: usize,
    user_collection: &'a UserCollection,
}

impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    /// A `next` method is the only `Iterator` trait method which is mandatory to be
    /// implemented. It makes accessible a huge range of standard methods,
    /// e.g. `fold`, `map`, `for_each`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            return user;
        }
        None
    }
}