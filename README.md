# axum_static

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.2.2-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/axum_static/blob/master/LICENSE)

static file serving router for axum server

## requirements

1. axum 0.6.\*

## guide

Usage is very simple.

First install crate.

```
cargo add axum_static
```

Then, create a static route and nest it in the existing route like so

```
let app = Router::new()
        .route("/", get(index))
        ......
        .nest("/static", axum_static::static_router("static"))
```

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

Then you can read the file like this.
It can also be a sub directory.
![](docs/1.png)
![](docs/2.png)

This is the end.

How easy is it?
