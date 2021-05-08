# parametrizer
A simple, safe crate for parsing properly-formatted math strings which represent parametric functions into Rust functions that compute them. Ported from an earlier version for Javascript.

### Example Usage

Include the crate in your Cargo.toml file:

```toml
parametrizer = "1.1.1"
```

Then simply create a Parametrizer struct and call the evaluate method:

```rust
use parametrizer::Parametrizer;

let parametric_function = Parametrizer::new("1+2*t*t").unwrap();

assert_eq!(19.0, parametric_function.evaluate(3.0));
```

The underlying terms are public to allow for the manual composition of terms in code to avoid the string parsing overhead. See the documentation for more examples of supported syntax.

### Performance

Obviously, performance of parametrized functions will never match that of Rust code, especially considering performance optimizations performed by the compiler. The overhead from the
parametrizer crate mostly comes from its recursive nature. For simple functions like polynomials, this can cause a fairly noticeable difference (~800 picoseconds for Rust vs ~18 nanoseconds for
parametrizer). For more complex functions, the performance gap lessens due to the fact that the function overhead is less noticeable. (For example, a function involving sine is ~5.5 nanoseconds for Rust vs ~23 nanoseconds for parametrizer.) 

(Numbers obtained from `cargo bench` on one machine. You can clone the repo and run the included benchmarks to see how performance stacks up on your machine.)

### Contribution
Please feel free to suggest additional features, bug fixes, or ways to make the code more idiomatic. Thanks!
