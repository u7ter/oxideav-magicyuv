//! Pure-Rust MagicYUV v7 lossless decoder + encoder.
//!
//! **Codec-only crate.** AVI is a container, not a codec, and its
//! demux + mux (including OpenDML 2.0 multi-RIFF) live in
//! `oxideav-avi`. This crate handles only the MAGY frame wire format.

#![forbid(unsafe_code)]

#[doc(hidden)]
pub mod bitreader;
pub mod decoder;
pub mod encoder;
pub mod error;
pub mod header;
#[doc(hidden)]
pub mod huffman;
#[doc(hidden)]
pub mod predict;
#[cfg(feature = "registry")]
pub mod registry;
#[cfg(test)]
mod roundtrip_tests;
pub mod slice_bounds;
pub mod tables;
#[cfg(feature = "trace")]
pub(crate) mod trace;

pub use crate::decoder::{decode_frame, decode_into, DecodedFrame, DecodedPlane, Samples};
#[cfg(feature = "registry")]
pub use crate::encoder::output_params;
pub use crate::encoder::{encode_frame, EncodeOptions, PlaneInput, PredictorStrategy, SliceMode};
pub use crate::error::{Error, Result};

#[cfg(feature = "registry")]
pub use crate::registry::{register, register_codecs, CODEC_ID_STR};

#[cfg(feature = "registry")]
oxideav_core::register!("oxideav-magicyuv", register);
