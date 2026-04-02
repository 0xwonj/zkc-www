# ZK Compiler Research

Static Leptos CSR site for the Protocol IR document archive.

## Development

```bash
cd site
./scripts/trunk.sh serve
```

## Build

```bash
cd site
./scripts/trunk.sh build --release
```

The production output is written to `site/dist/`.

## Hosting Notes

This app uses browser routing for `/` and `/protocol-ir`, with `/archive` retained
as an alias for the Protocol IR dossier. Static hosts should rewrite unknown paths
to `index.html` so direct navigation resolves inside the client router.

## Environment Note

In this workspace, `NO_COLOR=1` is exported globally and `trunk 0.21` misparses
that flag. The wrapper script unsets `NO_COLOR` before invoking Trunk so local
development and release builds remain reproducible.
