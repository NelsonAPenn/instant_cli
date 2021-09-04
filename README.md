# instant\_cli

Instantly turn a Rust library into a simple CLI app.

## Usage (not yet implemented)

Consider a function from `lib.rs` that you want to be exposed in a CLI interface:

```rust
pub fn show(n: u32, verbose: bool, format: String)
{
  ...
}
```

To expose it as part of the command line interface, simply add

```rust
#[instant_cli]
pub fn show(
    n: u32,
    #[flag(short='v')] verbose: bool,
    #[option(default="zip")] format: String
)
{
  ...
}
```

...and that's it! No main function!

## P.S.

```rust
#[option(short='h')] help // is reserved
```