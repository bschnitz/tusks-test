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

## Calling Callbacks with AutomaticTtype Conversion

For each public function in the module tree, a mirror function is now generated.
The mirror function has the same parameters, which however may have a different type:

- parameters of type `bool` will have the same type in the mirror function
- parameters of type `Option<_type_>` will have the type `Option<String>` in the
  mirror function.
- parameters of any other type `_type_` will have the type `String` in the
  mirror function.

The mirror function will then convert all non-boolean types using `from_string`
into the types accepted by the original function and pass them to the original
function. Of course the `Option<*>`-types are first unwrapped and then
converted, if not `None` or otherwise passed as `None`.

This means, that the types used in the orginal functions must implement the
`FromStr`-Trait!

**Example:**
```rust
use tusks::tusks;

#[tusks]
mod tasks {
    pub fn count(times: u8) {
        for i in 1..=times {
            println!("{}", i);
        }
    }
}

fn main() {
    tasks::__tusks_internal_module::mirror_module::count("5".into());
    // will print:
    // 1
    // 2
    // 3
    // 4
    // 5
}
```
