---
title: "Extending Python with Rust: a hands-on introduction to PyO3"
author: "Ivan Carvalho"
format:
  beamer:
    pdf-engine: tectonic
---

## Agenda

- Introduce  the tools
- Build an extension using PyO3
- Understand what problems Python extensions in Rust solve

## About me

- Maintainer of `rustworkx` (peaked at the Top 1% of PyPI packages)
- User of PyO3 since 2021
- Casually contributed features I needed to PyO3 upstream

## Why this presentation

Rust has taken over the Python ecosystem!

- Popular libraries like `pydantic` and `cryptography` use Rust
- Python tooling also uses Rust like:
    - Astral's `uv`
    - Microsoft's Python Environment Tools
    - Facebook's `pyrefly`

But why? Hopefully you'll understand by the end.

note: this presentation was rendered with Tectonic (LaTeX in Rust)

## Tools we are going to be using

We don't assume you'll be familiar with all of them. Rust users will know Rust tools,
Python users will known Python tools. 

- PyO3
- maturin
- cargo
- pip
- crates.io
- PyPI