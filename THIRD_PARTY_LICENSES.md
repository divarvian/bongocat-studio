# Third-Party Licenses & Attributions

**BongoCat Studio** is licensed under the MIT License (Copyright (c) 2026 Arvian).
This project embeds and links third-party open-source components and artwork listed below.

## 1. Embedded Classic Bongo Cat SVG Artwork

The five classic Bongo Cat frame SVGs in `assets/svg/` (`bongo-both-up.svg`,
`bongo-left-down.svg`, `bongo-right-down.svg`, `bongo-both-down.svg`, and
`bongo-sleeping.svg`) are included from **wayland-bongocat**:

- **Project**: [wayland-bongocat](https://github.com/saatvik333/wayland-bongocat)
- **License**: MIT License
- **Copyright**: Copyright (c) 2025 Saatvik Sharma

```text
MIT License

Copyright (c) 2025 Saatvik Sharma

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 2. Direct Rust Crates

| Crate | License | Upstream |
| :--- | :--- | :--- |
| `iced` | MIT | https://github.com/iced-rs/iced |
| `axum` | MIT | https://github.com/tokio-rs/axum |
| `tokio` / `tokio-stream` | MIT | https://github.com/tokio-rs/tokio |
| `evdev` | Apache-2.0 OR MIT | https://github.com/emberian/evdev |
| `ksni` | Unlicense | https://github.com/iovxw/ksni |
| `serde` / `serde_json` | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `toml` | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| `clap` | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| `dirs` | MIT OR Apache-2.0 | https://github.com/dirs-dev/dirs-rs |
| `thiserror` | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| `tracing` / `tracing-subscriber` | MIT | https://github.com/tokio-rs/tracing |
| `futures-core` | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |

## 3. Mozilla Public License 2.0 (MPL-2.0) Notice

The compiled binary includes the following unmodified transitive Rust crates
licensed under the Mozilla Public License, v. 2.0 (MPL-2.0), via `iced` and `dirs`:

| Crate | Version | License | Upstream Source |
| :--- | :--- | :--- | :--- |
| `resvg` | 0.42.0 | MPL-2.0 | https://github.com/linebender/resvg |
| `usvg` | 0.42.0 | MPL-2.0 | https://github.com/linebender/resvg |
| `option-ext` | 0.2.0 | MPL-2.0 | https://github.com/soc/option-ext |

No files belonging to these MPL-2.0 crates were modified. The full text of
MPL-2.0 is available at https://mozilla.org/MPL/2.0/.

## 4. Cultural Credit & Trademark Disclaimer

BongoCat Studio is a free, fan-made open-source project inspired by the
Bongo Cat internet meme (original cat illustration by StrayRogue, bongo drum
edit by DitzyFlama, 2018). This project is not affiliated with, sponsored by,
or endorsed by any commercial Bongo Cat game, studio, or brand.
