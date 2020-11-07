# Crate somedoc

A very simple document model and markup generator.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/somedoc.svg)](https://crates.io/crates/somedoc)
[![docs.rs](https://docs.rs/somedoc/badge.svg)](https://docs.rs/somedoc)
![Build](https://github.com/johnstonskj/rust-somedoc/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-somedoc/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-somedoc.svg)](https://github.com/johnstonskj/rust-somedoc/stargazers)

-----------

## Model

TBD.

## Writers

TBD

## Example

TBD

-----------

## Changes

**Version 0.1.4**

* Moved the rules for formatting an `Anchor` into each writer. 

**Version 0.1.3**

* Added `Formatted` alongside `CodeBlock`.
* Removed some additional blank lines from both Markdown and XWiki writers.

**Version 0.1.2**

* Fixed nested list bug in XWiki

**Version 0.1.1**

* Added [cargo-husky](https://github.com/rhysd/cargo-husky) for git hooks.
* Fixed bug in tables for XWiki.

**Version 0.1.0**

* Initial commit. Basic model working, initial markdown and xwiki writers.

## TODO