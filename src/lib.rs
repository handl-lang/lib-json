//! `json` — the JSON codec (`Encoder`/`Decoder`/`Serializer`) plus the
//! `json.parse` / `json.stringify` functions.
//!
//! Rule B: the codec lives here, not in `lib-serialize`; that crate stays
//! contract-only and the stdlib/kit row composes the two.

pub mod ops;

pub use ops::{json_op_registry, register, Json, JSON_OPS};

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const JSON_FUNCS: &[&str] = &["stringify","parse"];
pub const JSON_SOURCES: &[(&str, &str, &[&str])] = &[
    ("helpers", include_str!("internal/helpers.hnl"), &["ws_p","num_p","alpha_p","slen","take","drop","esc"]),
    ("stringify", include_str!("stringify.hnl"), &["stringify"]),
    ("parse", include_str!("parse.hnl"), &["parse"]),
];

/// The `json` package index — aggregator `.hnl` re-exporting the arrows.
pub const JSON_HNL: &str = include_str!("index.hnl");
