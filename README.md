# cfg-match

Provides cfg_match and cfg_match_with_default macros

[Based on: cfg-if](http://alexcrichton.com/cfg-if)

Macros to ergonomically define an item depending on a large number of #[cfg]
parameters. Structured like a match block, the first matching branch is the
item that gets emitted.

## Example

```rust
#[macro_use]
extern crate cfg_match;

fn works() -> bool {
    cfg_match! {
         #[cfg(foo)] => { false },
         #[cfg(test)] => { true }
    }
}

fn works_with_default() -> bool {
    cfg_match_with_default! {
        #[cfg(foo)] => { false },
        #[cfg(bar)] => { false },
        _ => { true }
    }
}

#[test]
fn smoke() {
    assert!(works());
    assert!(works_with_default());
}

```

When using cfg_match and no match is possible you get a compilation error:

```rust
fn does_not_compile() -> bool {
    cfg_match! {
        #[cfg(foo)] => { false },
        #[cfg(bar)] => { false }
    }
}
```

You get the following error:

```text
error: cfg_match: #[cfg(<check desired feature case>)] was not implemented.
  --> tests/xcrate.rs:45:5
   |
45 | /     cfg_match! {
46 | |         #[cfg(foo)] => { false },
...  |
51 | |         }
52 | |     }
   | |_____^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
