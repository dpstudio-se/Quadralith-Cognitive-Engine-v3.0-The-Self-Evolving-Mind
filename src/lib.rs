//! Quadralith Cognitive Engine.
//!
//! The crate implements a deterministic, inspectable state machine. Names such
//! as "entropy", "metabolic", and "Silent Glyph" belong to the model's symbolic
//! vocabulary; they are software metrics, not claims of biological cognition or
//! new physical laws.

pub mod core;

pub use core::quadralith::{
    CycleReport, EngineConfig, EngineError, EngineSnapshot, LearningSpiral, OriginCore,
    PerformanceMode, QuadralithEngine, SelectionVector, SilentGlyph, TeaxBatchReport, TeaxProfile,
    TransparencyField, TEAX_SIGNATURE,
};
