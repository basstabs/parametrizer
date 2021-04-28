# parametrizer
A simple, safe crate for parsing properly-formatted math strings which represent parametric functions into Rust functions that compute them. Ported from an earlier version for Javascript.

### Example Usage

Include the crate in your Cargo.toml file:

```toml
parametrizer = "1.1.0"
```

Then simply create a Parametrizer struct and call the evaluate method:

```rust
use parametrizer::Parametrizer;

let parametric_function = Parametrizer::new("1+2*t*t").unwrap();

assert_eq!(19.0, parametric_function.evaluate(3.0));
```

The underlying terms are public to allow for the manual composition of terms in code to avoid the string parsing overhead. See the documentation for all of the currently supported syntax. 

### Contribution
Please feel free to suggest additional features, bug fixes, or ways to make the code more idiomatic. Thanks!
