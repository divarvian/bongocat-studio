# BongoCat Studio

[![License: MIT](https://img.shields.io/badge/License-MIT-38bdf8.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024%20Safe-f97316.svg)](Cargo.toml)
[![Linux](https://img.shields.io/badge/Linux-Wayland%20%7C%20X11-4ade80.svg)](#supported-linux-systems)

BongoCat Studio is a standalone Rust desktop application and built-in OBS Studio Browser Source server for Linux.

It ships as a single native binary with no Node.js runtime, no separate C helper, no Electron or WebKitGTK dependency, and no desktop shell extension.

## Features

- **Single Native Binary**: Written in 100% safe Rust (`iced`, `evdev`, `axum`, `tokio`, and `ksni`) with `#![forbid(unsafe_code)]` and `epoll`-based input reading that uses near-zero CPU when idle.
- **Live OBS Config Sync**: Add one fixed URL (`http://127.0.0.1:17890/`) to OBS Studio. Any change to presets, modes, pointer devices, themes, eyes, KPS, bounce, or privacy in the desktop app updates OBS immediately without editing the URL.
- **Three Visual Modes**:
  - **Slanted (~10 deg)**: Unified slanted laptop layout with the trackpad centered below the spacebar, or a tucked mousepad beside the keyboard with the cat centered over the keys.
  - **Flat Desk**: Top-down laptop trackpad or mouse layout with a 63-key QWERTY keyboard.
  - **Classic**: Five embedded classic Bongo Cat SVG frames with bounce and KPS display.
- **Four-Stage Paw Animation**: Active typing or pointer movement, short relaxed rest on the palm-rest or mouse, smooth dual-paw raise showing pink paw beans after idle delay (configurable from 1 to 15 seconds), and calm sleep mode.
- **Ergonomic Input & Clickpad Support**: Ten-finger touch-typing hand split, alternating spacebar thumbs, two-finger clickpad right-click and scroll, three-finger middle-click, bottom-right clickpad click zone, and delta-accumulated virtual cursor eye tracking.
- **Privacy & Background Operation**: Loopback-only HTTP server (`127.0.0.1`) with Host header validation, zero keycode logging, Full / Gaming / Zones privacy modes, one-click Pause Privacy, close-to-background daemon mode, SNI system tray, single-instance relaunch, and login autostart (`--minimized`).

## Supported Linux Systems

The compiled binary links only against standard `libc`, `libm`, and `libgcc_s`, and loads Wayland or X11 dynamically at runtime.

- **Distributions**: Ubuntu (22.04, 24.04, 26.04), Debian (12, 13), Linux Mint, Pop!_OS, Fedora, Arch Linux, openSUSE, and NixOS.
- **Desktops & Window Managers**: GNOME, KDE Plasma, COSMIC, XFCE, Cinnamon, MATE, Hyprland, Sway, and Niri (both Wayland and X11).
- **OBS Studio**: Compatible with native system packages (`.deb`, `.rpm`, Arch) as well as OBS Studio Flatpak over `127.0.0.1:17890`.

## Installation

### From Packages ([GitHub Releases](https://github.com/divarvian/bongocat-studio/releases) or Local Build)

**Ubuntu, Debian, Linux Mint, Pop!_OS (`.deb`)**:

```bash
sudo apt install ./bongocat-studio_0.1.0_amd64.deb
```

**Fedora, openSUSE, RHEL-compatible (`.rpm`)**:

```bash
sudo dnf install ./bongocat-studio-0.1.0-1.x86_64.rpm
```

**Arch Linux (`PKGBUILD`)**:

```bash
cd packaging/arch && makepkg -si
```

**Portable Tarball (`.tar.gz`)**:

```bash
tar -xzf bongocat-studio-0.1.0-linux-x86_64.tar.gz
cd bongocat-studio-0.1.0-linux-x86_64
./install.sh
```

### Grant Input Group Permission

Reading global keyboard, mouse, and touchpad events through Linux `evdev` requires membership in the `input` group:

```bash
sudo usermod -aG input "$USER"
```

Log out and back in once after adding your user to the `input` group.

## Building From Source

### Prerequisites

Install the Rust toolchain via [rustup](https://rustup.rs) and basic C build tools:

```bash
# Ubuntu, Debian, Linux Mint, Pop!_OS
sudo apt update && sudo apt install -y build-essential pkg-config

# Fedora
sudo dnf install -y gcc pkgconf-pkg-config rpm-build

# Arch Linux
sudo pacman -S --needed base-devel pkgconf
```

### Build, Test & Local Install

```bash
make check    # cargo fmt --check, clippy -D warnings, cargo test --workspace
make release  # build target/release/bongocat-studio
make install  # install binary, .desktop entry, icon, and licenses to ~/.local
```

### Building Distribution Packages (`.rpm`, `.deb`, `.tar.gz`)

```bash
make rpm       # build dist/bongocat-studio-<version>-1.x86_64.rpm
make deb       # build dist/bongocat-studio_<version>_amd64.deb
make tarball   # build dist/bongocat-studio-<version>-linux-x86_64.tar.gz
make packages  # build all packages and generate dist/SHA256SUMS
```

For public distribution across older and newer distros, release binaries should be built on an older `glibc` baseline such as Ubuntu 22.04 (`glibc 2.35`). The included workflow at `.github/workflows/release.yml` automates building `.deb`, `.rpm`, `.tar.gz`, and `SHA256SUMS` on `ubuntu-22.04` whenever a `v*` tag is pushed.

## OBS Studio Setup

1. Launch **BongoCat Studio** from your application menu or run `bongocat-studio`.
2. In OBS Studio, add a **Browser** source with:
   - **URL**: `http://127.0.0.1:17890/`
   - **Recommended size**: `640 x 450` (Slanted), `640 x 430` (Flat), or `420 x 260` (Classic)
3. Change any preset, mode, theme, or timing option inside BongoCat Studio. OBS updates live automatically.
4. Optional URL query overrides are also available for multi-scene setups (for example, `http://127.0.0.1:17890/?mode=classic&kps=1`).

## License & Credits

- **BongoCat Studio** is licensed under the [MIT License](LICENSE), Copyright (c) 2026 Arvian.
- **Classic Bongo Cat SVG frames** (`assets/svg/*.svg`) are from [wayland-bongocat](https://github.com/saatvik333/wayland-bongocat), licensed under the MIT License, Copyright (c) 2025 Saatvik Sharma.
- **Third-party Rust dependencies**, including MPL-2.0 notices for unmodified transitive crates (`resvg`, `usvg`, and `option-ext`), are documented in [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) and inside the application under **Privacy & System -> About & Licenses**.

## Disclaimer

BongoCat Studio is a free, fan-made open-source project inspired by the Bongo Cat internet meme (original cat illustration by StrayRogue, bongo drum edit by DitzyFlama, 2018). This project is not affiliated with, sponsored by, or endorsed by any commercial Bongo Cat game, studio, or brand.
