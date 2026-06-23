# Address Source

## Purpose

This directory holds the pure root, weak-path, and Markdown leaf address reducer
for document and artifact tools.

## Table of Contents

- [file.rs](file.rs): file-root classification and catalog ancestor lookup.
- [model.rs](model.rs): address kinds, problems, and next-action model.
- [path.rs](path.rs): path helpers for suffix, parent, and workspace-relative forms.
- [render.rs](render.rs): semantic refusal and valid-example rendering.
- [resolve.rs](resolve.rs): root and optional path reducer entry point.
