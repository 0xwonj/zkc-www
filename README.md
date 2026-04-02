# ZKC WWW

ZKC public website built with Leptos in client-side rendering mode and bundled with Trunk.

## Prerequisites

- Rust toolchain
- `wasm32-unknown-unknown` target
- `trunk`

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Development

Run the site locally from the `site/` app directory.

```bash
cd site
trunk serve --open
```

The dev server defaults to `http://127.0.0.1:8080`.

## Production Build

```bash
cd site
trunk build --release
```

The production bundle is written to `site/dist/`.

## Notes

- This is a CSR-only Leptos app. There is no server-side rendering setup in this repository.
- The project page route is `/protocol-ir`.

## Troubleshooting

If Trunk exits with `invalid value '1' for '--no-color'`, your shell is exporting `NO_COLOR=1`.

```bash
unset NO_COLOR
```

Or run Trunk commands with:

```bash
env -u NO_COLOR trunk serve --open
env -u NO_COLOR trunk build --release
```
