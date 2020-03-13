# Const Layout

Compile time checked Layout bounds.


### Example

the input and output type must habe the same size

```rust
use const_layout::EqSize;

fn must_have_same_size<I: EqSize<O>, O>(input: I) -> O {
    ...
}

```
