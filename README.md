# How To Run, What To Expect

Run `cargo build` to see the parsed structure of the `#[tusks]` module. Try to
break the syntax to check out error handling at compile time.

## Getting the Tasks Tree

```rust
#[tusks]
mod tasks {
...
}

fn main() {
    let tree = tasks::__tusks_internal_module::get_tusks_tree();
    println!("{:#?}", tree);
}
```

This will print the complete tree including links to other modules.
