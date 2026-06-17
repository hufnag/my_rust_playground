use cxx::UniquePtr;

#[cxx::bridge(namespace = "cpp_lib")]
mod ffi {
    unsafe extern "C++" {
        include!("user_age_table_bridge.h");

        type UserAgeTable;

        fn new_user_age_table() -> UniquePtr<UserAgeTable>;
        fn add_user_age(table: Pin<&mut UserAgeTable>, key: &str, age: u8) -> Result<()>;
        fn get_user_age(table: &UserAgeTable, key: &str, age: &mut u8) -> bool;
    }
}

pub struct UserAgeTable {
    inner: UniquePtr<ffi::UserAgeTable>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UserAgeTableError {
    KeyAlreadyInUse,
}

impl UserAgeTable {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_user_age_table(),
        }
    }

    pub fn add(&mut self, key: &str, age: u8) -> Result<(), UserAgeTableError> {
        ffi::add_user_age(self.inner.pin_mut(), key, age)
            .map_err(|_| UserAgeTableError::KeyAlreadyInUse)
    }

    pub fn get(&self, key: &str) -> Option<u8> {
        let mut age = 0;

        if ffi::get_user_age(&self.inner, key, &mut age) {
            Some(age)
        } else {
            None
        }
    }
}

impl Default for UserAgeTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_gets_a_user_age() {
        let mut table = UserAgeTable::new();

        assert_eq!(table.get("Martin"), None);

        table.add("Martin", 20).unwrap();
        table.add("Jens", 36).unwrap();

        assert_eq!(table.get("Martin"), Some(20));
        assert_eq!(table.get("Jens"), Some(36));
        assert_eq!(table.get("Horst"), None);
    }

    #[test]
    fn duplicate_keys_are_returned_as_rust_errors() {
        let mut table = UserAgeTable::new();

        table.add("Martin", 20).unwrap();

        assert_eq!(
            table.add("Martin", 21),
            Err(UserAgeTableError::KeyAlreadyInUse)
        );
        assert_eq!(table.get("Martin"), Some(20));
    }
}
