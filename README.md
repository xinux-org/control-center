<p align="center">
    <h3 align="center">GNOME Settings Application written on Rust/GTK.</h3>
</p>

<p align="center">
    <img src=".github/assets/demo.png" alt="GN*ME Settings Demo" width=800>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/xinux-org/settings?style=flat&logo=rust&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Top Used Language">
    <a href="https://github.com/xinux-org/e-imzo/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/instances/test.yml?style=flat&logo=github&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Test CI"></a>
</p>

## About

Rewritten version of GNOME Control Center for Xinux OS.

## Development

This application has Linux-only dependencies.

```bash
# download dependencies
nix develop 

# Initiate meson environment for the first time. This will generate ./src/config.rs
meson setup build

# build the project
nix build . --show--trace

./result/bin/control-center

# Optional. Generate translation words from /po/POTFILES.in if needed.
cd ./po
xgettext --directory=.. --files-from=POTFILES.in --from-code=UTF-8 -kgettext -o translations.pot
```
