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

Clap will do type converstion. However the type must implement FromStr or
Option<FromStr>

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

## TODO and Wanted Features

- [ ] Argumente mit Multiplicity
- [ ] Doku in tusks
- [ ] Publish on crates.io
- [ ] Doc texte für tusks und argumente
- [ ] Listen (--buy apple pear bread)
- [ ] Enum (nur bestimmte werte erlauben)
- [ ] Structs mit den Argumenten
- [ ] remove mirror module code
- [ ] other cleanup
- [ ] reexport clap from `tusks_lib` and then `tusks` to have it only once as
      dependency
- [ ] Alternative attribut syntax für Argumente
- [ ] Tests
