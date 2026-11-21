# Third-party notices

STUDYVUT includes or is derived from the following works. The crates and npm
packages it depends on carry their own licenses (see `Cargo.lock` and
`package-lock.json`); a generated list of all of them is on the TODO list.

## HARDWARIO TOWER Bridge

The workspace layout, parts of the Rust code (settings, paths, the SQLite
migration ladder, the Tauri setup) and of the frontend (the stylesheet, the
sidebar, `Icon`, `PrefsSwitcher`, `SegmentedControl`, `Section`, `Field`,
`Toggle`, the i18n helpers, the release scripts' approach) and the subject
colour palette are derived from https://github.com/hardwario/tower-bridge.

```
MIT License

Copyright (c) HARDWARIO a.s.

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

## Fonts

- **Inter** and **Roboto Mono**: SIL Open Font License 1.1, see
  `src/assets/fonts/OFL-Inter.txt` and `src/assets/fonts/OFL-RobotoMono.txt`.
- **KaTeX fonts** (bundled by the `katex` package): SIL Open Font License 1.1.

## Icons

Icon paths from **Lucide** (https://lucide.dev), ISC License, Copyright (c)
Lucide Contributors.

## KaTeX

Math typesetting by **KaTeX** (https://katex.org), MIT License, Copyright (c)
2013-2020 Khan Academy and other contributors.
