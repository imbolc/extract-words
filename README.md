[![License](https://img.shields.io/crates/l/extract-words.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/extract-words.svg)](https://crates.io/crates/extract-words)
[![Docs.rs](https://docs.rs/extract-words/badge.svg)](https://docs.rs/extract-words)

<!-- cargo-sync-readme start -->

# extract-words

Extracts words from a phrase, omitting any punctuation, without allocation

```rust
assert_eq!(
    extract_words("Hola, mundo!").collect::<Vec<_>>(),
    ["Hola", "mundo"]
);
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
