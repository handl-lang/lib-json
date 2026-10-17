//! `json` — the JSON codec (`Encoder`/`Decoder`/`Serializer`) plus the
//! `json.parse` / `json.stringify` functions.
//!
//! Rule B: the codec lives here, not in `lib-serialize`; that crate stays
//! contract-only and the stdlib/kit row composes the two.

pub mod ops;

pub use ops::{json_op_registry, register, Json, JSON_OPS};
