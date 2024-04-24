# axum_static

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.7.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/axum_static/blob/master/LICENSE)

static file serving for axum

## Version

You must use axum_static that matches your axum version.

- axum 0.7 => axum_static 1.7

## Usage

```bash
cargo add axum_static
```

```rust
let app = Router::new()
        .nest("/", axum_static::static_router("public"))
```