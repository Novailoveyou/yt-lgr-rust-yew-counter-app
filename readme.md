# Building a Rust App With Yew

## About

This is me following
[Let's Get Rusty's youtube video](https://youtu.be/KmOeFrwz8BM)

Year 2022

## CLI

Make sure Rust & cargo are installed

Install trunk

```zsh
cargo install trunk
```

Add the Rust's target to WA (WebAssembly)

```zsh
rustup target add wasm32-unknown-unknown
```

Init new project

```zsh
cargo new yew-counter-app
```

Run app locally

```zsh
trunk serve
```
