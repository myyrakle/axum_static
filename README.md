# axum_static

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.2.3-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/axum_static/blob/master/LICENSE)

static file serving for axum

## Usage

```bash
cargo add axum_static
```

```rust
let app = Router::new()
        .nest("/", axum_static::static_router("public"))
```