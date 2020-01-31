use std::env;
use std::ffi::{OsStr, OsString};

/// A rust lifetime scope for a set environment
/// variable. When an instance goes out of scope it will
/// automatically cleanup the environment
pub struct ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    name: T,
    old_value: Option<OsString>,
}

impl<T> ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    /// Sets the environment variable {name} to {value}. The
    /// returned instance should be assigned to a `_name`
    /// binding so that it lasts as long as the current
    /// block.
    ///
    /// ```rust
    /// use scoped_env::ScopedEnv;
    /// let c = ScopedEnv::set("HELLO", "WORLD");
    /// assert_eq!(std::env::var(c).unwrap().as_str(), "WORLD");
    /// ```
    pub fn set(name: T, value: T) -> Self {
        let old_value = env::var_os(name.as_ref());
        env::set_var(name.as_ref(), value);
        Self { name, old_value }
    }
}

impl<T> AsRef<OsStr> for ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    fn as_ref(&self) -> &OsStr {
        self.name.as_ref()
    }
}

impl<T> Drop for ScopedEnv<T>
where
    T: AsRef<OsStr>,
{
    fn drop(&mut self) {
        match self.old_value {
            Some(ref old_value) => {
                env::set_var(self.as_ref(), old_value)
            }
            None => env::remove_var(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_set() {
        let c = ScopedEnv::set("FOOBAR", "hello");
        assert_eq!(env::var(c).unwrap(), "hello");
    }

    #[test]
    fn does_unset_at_end_of_block() {
        env::remove_var("FOOBAR1");
        {
            let c = ScopedEnv::set("FOOBAR1", "hello");
            assert_eq!(env::var(c).unwrap(), "hello");
        }

        assert_eq!(env::var_os("FOOBAR1"), None);
    }

    #[test]
    fn does_reset_at_end_of_block() {
        env::set_var("FOOBAR1", "OLD_VALUE");
        {
            let c = ScopedEnv::set("FOOBAR1", "hello");
            assert_eq!(env::var(c).unwrap(), "hello");
        }

        assert_eq!(env::var("FOOBAR1").unwrap(), "OLD_VALUE");
    }
}
