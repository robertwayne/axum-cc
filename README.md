# axum-cc

`axum-cc` implements a `tower` service layer for setting `Cache-Control` headers
on responses based on MIME types. _This is a work-in-progress and is not
recommended for production use._

## Getting Started

This is not released on crates.io yet, so you'll need to add it as a git dependency.

```toml
[dependencies]
axum-cc = { git = "https://github.com/robertwayne/axum-cc" }
```

## Contributing

Contributions are always welcome! If you have an idea for a feature or find a
bug, let me know. PR's are appreciated, but if it's not a small change, please
open an issue first so we're all on the same page!

## License

`axum-cc` is dual-licensed under either

- **[MIT License](/LICENSE-MIT)**
- **[Apache License, Version 2.0](/LICENSE-APACHE)**

at your option.
