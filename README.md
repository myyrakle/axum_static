# axum_static

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.8.6-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/axum_static/blob/master/LICENSE)

static file serving for axum

## Version

You must use axum_static that matches your axum version.

- axum 0.6 => axum_static ~1.6.*
- axum 0.7 => axum_static ~1.7.*
- axum 0.8 => axum_static ~1.8.*

## Usage

First install crate.
```bash
cargo add axum_static
```

Then, create a static route and nest it in the existing route like so
```rust
let app = Router::new()
        .nest("/", axum_static::static_router("public"))
```

## Features

- `handle_error`: Adds graceful IO error responses via `tower_http`'s `handle_error` hook.
- `mime_guess`: Swaps the manual extension map for `mime_guess` so content-types stay current automatically.
- `status_code`: Builds on `handle_error` to include human-readable status text in error responses.
- `tracing`: Emits structured `warn!` logs for unknown MIME types and `error!` logs for IO failures.

## State

If your app has state, [you'll need to add](https://docs.rs/axum/latest/axum/routing/struct.Router.html#nesting-routers-with-state) `with_state`, because static_router does not use state (`()`):

```
let app = Router::new()
        .route("/", get(index))
        ......
        .nest("/static", axum_static::static_router("static").with_state())
        ......
        .with_state(YourAppState { ... })
```

The argument of the `static_router` function is the path to read static files based on the project root path.

Then you can read the file like this. It can also be a sub directory.
![](docs/1.png)
![](docs/2.png)

This is the end.