# scoped-env

A helper struct (mostly for tests) that scopes an environment variable lifetime to that of a rust scope. Useful for setting environment variables that should be local to the test function.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scoped-env = "2.1.0"
```
