//! `json` — the JSON codec: pure `.hnl` `parse`/`stringify` (stack-machine
//! tokenizer + worklist serializer) over `str`/`char`/`math`/`obj`/`typeof`.
//! This crate is a data carrier — the impls live in `src/*.hnl`.

/// The handl source declaring the `json` interface.
pub const JSON_HNL: &str = include_str!("index.hnl");

/// Exported `.hnl` arrows.
pub const JSON_FUNCS: &[&str] = &["stringify", "parse"];

/// `.hnl` package source set.
pub const JSON_SOURCES: &[(&str, &str, &[&str])] = &[
    ("helpers", include_str!("internal/helpers.hnl"), &[]),
    ("stringify", include_str!("stringify.hnl"), &["stringify"]),
    ("parse", include_str!("parse.hnl"), &["parse"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const JSON_FLAT_HNL: &str = include_str!("index_flat.hnl");
