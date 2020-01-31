#![crate_name = "scoped_env"]

use std::{
    env::{self, VarError},
    ffi::{OsStr, OsString},
};

/// A rust lifetime scope for a set environment variable. When an instance goes out of scope
/// it will automatically cleanup the environment
pub struct ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    name: T,
}

impl<T> ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    /// Sets the environment variable {name} to {value}. The returned instance should be assigned
    /// to a `_name` binding so that it lasts as long as the current block.
    ///
    /// ```rust
    /// use scoped_env::ScopedEnv;
    /// let c = ScopedEnv::set("HELLO", "WORLD");
    /// assert_eq!(c.get().unwrap().as_str(), "WORLD");
    /// ```
    pub fn set(name: T, value: T) -> Self {
        env::set_var(&name, value);
        Self { name }
    }

    /// Used to get the stored value of the assosiated name
    pub fn get_os(&self) -> Option<OsString> {
        env::var_os(&self.name)
    }

    // Used to get the stored value of the assosiated name and convert it to a rust `String`
    pub fn get(&self) -> Result<String, VarError> {
        env::var(&self.name)
    }
}

impl<T> Drop for ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    fn drop(&mut self) {
        env::remove_var(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_set() {
        let _c = ScopedEnv::set("FOOBAR", "hello");
        assert_eq!(env::var_os("FOOBAR").unwrap().to_str().unwrap(), "hello");
    }

    #[test]
    fn does_unset_at_end_of_block() {
        {
            let _c = ScopedEnv::set("FOOBAR1", "hello");
            assert_eq!(env::var_os("FOOBAR1").unwrap().to_str().unwrap(), "hello");
        }

        assert_eq!(env::var_os("FOOBAR1"), None);
    }
}
