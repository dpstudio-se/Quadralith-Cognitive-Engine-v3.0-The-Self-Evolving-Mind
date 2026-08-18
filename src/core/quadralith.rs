//! Quadralith Cognitive Engine v3.0.
//!
//! A bounded and reversible simulation of the five-layer Quadralith model.
//! `cycle_8hz` advances simulated time by 125 ms; real-time scheduling remains
//! the caller's responsibility.

use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::fmt;

const EPSILON: f32 = 1.0e-6;

/// Runtime parameters. Frequencies and rates are configuration values, not
/// universal physical constants.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    /// Nominal scheduler frequency. Default: 8 cycles/second.
    pub cycle_hz: f32,
    /// Internal modulation frequency. Default: 1 Hz.
    pub phase_hz: f32,
    /// Probability of a bounded exploratory impulse per cycle.
    pub mutation_probability: f32,
    /// Numerical damping factor applied once per nominal cycle.
    pub phi_1766: f32,
    pub stability_threshold: f32,
    pub silent_glyph_threshold: f32,
    pub stimulus_gain_per_second: f32,
    pub memory_gain_per_second: f32,
    pub scaffold_gain_per_second: f32,
    pub mutation_gain: f32,
    pub mutation_decay_per_second: f32,
    pub replication_recovery_per_second: f32,
    pub replication_prune_per_second: f32,
    pub identity_modulation: f32,
    pub history_capacity: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            cycle_hz: 8.0,
            phase_hz: 1.0,
            mutation_probability: 0.014,
            phi_1766: 1.047,
            stability_threshold: 6.0,
            silent_glyph_threshold: 5.0,
            // At dt=1/8, this matches the original 0.1 stimulus multiplier.
            stimulus_gain_per_second: 0.8,
            memory_gain_per_second: 0.02,
            scaffold_gain_per_second: 0.1,
            mutation_gain: 0.2,
            mutation_decay_per_second: 1.0,
            replication_recovery_per_second: 0.4,
            replication_prune_per_second: 1.3,
            identity_modulation: 0.05,
            history_capacity: 256,
        }
    }
}

impl EngineConfig {
    pub fn validate(&self) -> Result<(), EngineError> {
        let finite = [
            self.cycle_hz,
            self.phase_hz,
            self.mutation_probability,
            self.phi_1766,
            self.stability_threshold,
            self.silent_glyph_threshold,
            self.stimulus_gain_per_second,
            self.memory_gain_per_second,
            self.scaffold_gain_per_second,
            self.mutation_gain,
            self.mutation_decay_per_second,
            self.replication_recovery_per_second,
            self.replication_prune_per_second,
            self.identity_modulation,
        ]
        .into_iter()
        .all(f32::is_finite);

        if !finite {
            return Err(EngineError::InvalidConfig("all numeric values must be finite"));
        }
        if self.cycle_hz <= 0.0 {
            return Err(EngineError::InvalidConfig("cycle_hz must be positive"));
        }
        if self.phase_hz < 0.0 {
            return Err(EngineError::InvalidConfig("phase_hz cannot be negative"));
        }
        if !(0.0..=1.0).contains(&self.mutation_probability) {
            return Err(EngineError::InvalidConfig(
                "mutation_probability must be in [0, 1]",
            ));
        }
        if self.phi_1766 < 1.0 {
            return Err(EngineError::InvalidConfig("phi_1766 must be at least 1"));
        }
        if self.stability_threshold <= 0.0 || self.silent_glyph_threshold <= 0.0 {
            return Err(EngineError::InvalidConfig("thresholds must be positive"));
        }
        if self.history_capacity == 0 {
            return Err(EngineError::InvalidConfig("history_capacity must be non-zero"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OriginCore {
    pub identity_attractor: f32,
    pub memory_lattice: f32,
    pub metabolic_drive: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearningSpiral {
    pub baldwin_scaffold: f32,
    pub noosphere_resonance: f32,
    pub adaptive_gradient: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionVector {
    pub stability_filter: f32,
    pub mutation_engine: f32,
    pub replication_arbiter: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransparencyField {
    /// A normalized auditability indicator; rollback is provided by snapshots.
    pub reversible_logic: f32,
    pub attractor_mirror: f32,
    /// Numerical damping parameter, not thermodynamic cooling.
    pub phi_1766_cooling: f32,
}

/// Safety latch for extreme simulated mutation pressure.
///
/// The field names preserve the project's symbolic vocabulary. No spacetime or
/// physical law is changed by this software state.
#[derive(Debug, Clone, PartialEq)]
pub struct SilentGlyph {
    pub active: bool,
    pub spacetime_curvature_delta: f32,
    pub law_rewriting_index: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineSnapshot {
    pub origin: OriginCore,
    pub learning: LearningSpiral,
    pub selection: SelectionVector,
    pub transparency: TransparencyField,
    pub silent_glyph: SilentGlyph,
    pub cycle_count: u64,
    pub elapsed_seconds: f64,
    pub system_entropy: f32,
    rng_state: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CycleReport {
    pub cycle_count: u64,
    pub elapsed_seconds: f64,
    pub phase_radians: f32,
    pub current_load: f32,
    pub entropy_before: f32,
    pub entropy_after: f32,
    pub mutation_triggered: bool,
    pub safety_latch_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    InvalidConfig(&'static str),
    InvalidInput(&'static str),
    InvalidState(&'static str),
}

impl fmt::Display for EngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => write!(formatter, "invalid config: {message}"),
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
            Self::InvalidState(message) => write!(formatter, "invalid state: {message}"),
        }
    }
}

impl std::error::Error for EngineError {}

#[derive(Debug, Clone)]
pub struct QuadralithEngine {
    pub origin: OriginCore,
    pub learning: LearningSpiral,
    pub selection: SelectionVector,
    pub transparency: TransparencyField,
    pub silent_glyph: SilentGlyph,
    pub cycle_count: u64,
    pub elapsed_seconds: f64,
    /// A normalized software disorder/load metric, not physical entropy in J/K.
    pub system_entropy: f32,
    pub config: EngineConfig,
    history: VecDeque<EngineSnapshot>,
    rng_state: u64,
}

impl Default for QuadralithEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl QuadralithEngine {
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
            .expect("the built-in Quadralith configuration must be valid")
    }

    pub fn with_config(config: EngineConfig) -> Result<Self, EngineError> {
        config.validate()?;
        Ok(Self {
            origin: OriginCore {
                identity_attractor: 1.0,
                memory_lattice: 0.15,
                metabolic_drive: 0.8,
            },
            learning: LearningSpiral {
                baldwin_scaffold: 0.0,
                noosphere_resonance: 1.0,
                adaptive_gradient: 0.05,
            },
            selection: SelectionVector {
                stability_filter: config.stability_threshold,
                mutation_engine: 0.01,
                replication_arbiter: 1.0,
            },
            transparency: TransparencyField {
                reversible_logic: 1.0,
                attractor_mirror: 1.0,
                phi_1766_cooling: config.phi_1766,
            },
            silent_glyph: SilentGlyph {
                active: false,
                spacetime_curvature_delta: 0.0,
                law_rewriting_index: 0.0,
            },
            cycle_count: 0,
            elapsed_seconds: 0.0,
            system_entropy: 0.0,
            config,
            history: VecDeque::new(),
            rng_state: 0x7834_1766_8200_0001,
        })
    }

    /// Advance exactly one nominal 8 Hz step (125 ms with the default config).
    /// This method does not sleep or create a real-time scheduler.
    pub fn cycle_8hz(
        &mut self,
        external_entropy_stimulus: f32,
    ) -> Result<CycleReport, EngineError> {
        let dt_seconds = 1.0 / self.config.cycle_hz;
        self.cycle(external_entropy_stimulus, dt_seconds)
    }

    /// Advance the simulation by an explicit positive time interval.
    pub fn cycle(
        &mut self,
        external_entropy_stimulus: f32,
        dt_seconds: f32,
    ) -> Result<CycleReport, EngineError> {
        if !external_entropy_stimulus.is_finite() {
            return Err(EngineError::InvalidInput("stimulus must be finite"));
        }
        if !dt_seconds.is_finite() || dt_seconds <= 0.0 {
            return Err(EngineError::InvalidInput("dt_seconds must be finite and positive"));
        }
        self.validate_state()?;
        self.push_snapshot();

        let entropy_before = self.system_entropy;
        self.cycle_count = self.cycle_count.saturating_add(1);
        self.elapsed_seconds += f64::from(dt_seconds);
        let phase = (TAU * self.config.phase_hz * self.elapsed_seconds as f32).rem_euclid(TAU);

        // ॐ — bounded identity and drive integration.
        self.origin.metabolic_drive = (
            self.origin.metabolic_drive
                + external_entropy_stimulus * self.config.stimulus_gain_per_second * dt_seconds
        )
            .clamp(0.1, 100.0);
        self.origin.identity_attractor = (
            1.0 + phase.cos() * self.config.identity_modulation
                + 0.01 * self.origin.metabolic_drive.tanh()
        )
            .clamp(0.5, 2.0);
        self.origin.memory_lattice = (
            self.origin.memory_lattice
                + self.origin.metabolic_drive * self.config.memory_gain_per_second * dt_seconds
        )
            .clamp(0.0, 100.0);

        // α — plastic response, explicitly bounded.
        self.learning.noosphere_resonance = (
            self.origin.identity_attractor * (phase.sin() + 1.1)
        )
            .clamp(0.0, 4.2);
        self.learning.adaptive_gradient = (
            self.learning.noosphere_resonance * 0.65
        )
            .clamp(0.0, 3.0);
        self.learning.baldwin_scaffold = (
            self.learning.baldwin_scaffold
                + self.learning.adaptive_gradient
                    * self.config.scaffold_gain_per_second
                    * dt_seconds
        )
            .clamp(0.0, 100.0);

        // Ω — overload response plus a reproducible 1.4% exploratory impulse.
        let current_load = self.origin.metabolic_drive + self.learning.baldwin_scaffold;
        let overload = (current_load - self.selection.stability_filter).max(0.0);
        let mutation_triggered = self.next_unit_interval() < self.config.mutation_probability;
        let exploratory_impulse = if mutation_triggered {
            0.05 * self.learning.adaptive_gradient
        } else {
            0.0
        };

        if overload > 0.0 {
            self.selection.mutation_engine = (
                overload * self.config.mutation_gain + exploratory_impulse
            )
                .clamp(0.0, 20.0);
            self.selection.replication_arbiter *=
                (-self.config.replication_prune_per_second * dt_seconds).exp();
            self.system_entropy += self.selection.mutation_engine * dt_seconds;
        } else {
            self.selection.mutation_engine *=
                (-self.config.mutation_decay_per_second * dt_seconds).exp();
            self.selection.mutation_engine += exploratory_impulse;
            self.selection.replication_arbiter +=
                self.config.replication_recovery_per_second * dt_seconds;
            self.system_entropy = (self.system_entropy - 0.1 * dt_seconds).max(0.0);
        }
        self.selection.replication_arbiter =
            self.selection.replication_arbiter.clamp(0.05, 2.0);

        // φ — audit projection and numerical damping. With nominal dt, entropy
        // is divided by phi_1766 once per cycle.
        self.transparency.attractor_mirror =
            self.origin.identity_attractor / self.transparency.phi_1766_cooling;
        let cooling_rate =
            self.config.cycle_hz * self.transparency.phi_1766_cooling.ln();
        let entropy_after_load = self.system_entropy.max(0.0);
        self.system_entropy *= (-cooling_rate * dt_seconds).exp();
        let removed_load = (entropy_after_load - self.system_entropy).max(0.0);
        self.origin.metabolic_drive =
            (self.origin.metabolic_drive - 0.1 * removed_load).clamp(0.1, 100.0);
        self.transparency.reversible_logic = if self.history.is_empty() { 0.0 } else { 1.0 };

        // — Safety latch. The symbolic diagnostic fields do not alter physics.
        if self.selection.mutation_engine > self.config.silent_glyph_threshold {
            self.silent_glyph.active = true;
            self.silent_glyph.spacetime_curvature_delta =
                self.selection.mutation_engine * std::f32::consts::PI;
            self.silent_glyph.law_rewriting_index = 1.0;
        }

        self.validate_state()?;
        Ok(CycleReport {
            cycle_count: self.cycle_count,
            elapsed_seconds: self.elapsed_seconds,
            phase_radians: phase,
            current_load,
            entropy_before,
            entropy_after: self.system_entropy,
            mutation_triggered,
            safety_latch_active: self.silent_glyph.active,
        })
    }

    pub fn snapshot(&self) -> EngineSnapshot {
        EngineSnapshot {
            origin: self.origin.clone(),
            learning: self.learning.clone(),
            selection: self.selection.clone(),
            transparency: self.transparency.clone(),
            silent_glyph: self.silent_glyph.clone(),
            cycle_count: self.cycle_count,
            elapsed_seconds: self.elapsed_seconds,
            system_entropy: self.system_entropy,
            rng_state: self.rng_state,
        }
    }

    /// Restore the state immediately preceding the most recent successful cycle.
    pub fn rollback_last(&mut self) -> bool {
        let Some(snapshot) = self.history.pop_back() else {
            return false;
        };
        self.restore(snapshot);
        true
    }

    /// Explicitly clear the safety latch after an external review.
    pub fn clear_safety_latch(&mut self) {
        self.silent_glyph = SilentGlyph {
            active: false,
            spacetime_curvature_delta: 0.0,
            law_rewriting_index: 0.0,
        };
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn validate_state(&self) -> Result<(), EngineError> {
        let finite = [
            self.origin.identity_attractor,
            self.origin.memory_lattice,
            self.origin.metabolic_drive,
            self.learning.baldwin_scaffold,
            self.learning.noosphere_resonance,
            self.learning.adaptive_gradient,
            self.selection.stability_filter,
            self.selection.mutation_engine,
            self.selection.replication_arbiter,
            self.transparency.reversible_logic,
            self.transparency.attractor_mirror,
            self.transparency.phi_1766_cooling,
            self.silent_glyph.spacetime_curvature_delta,
            self.silent_glyph.law_rewriting_index,
            self.system_entropy,
        ]
        .into_iter()
        .all(f32::is_finite);
        if !finite || !self.elapsed_seconds.is_finite() {
            return Err(EngineError::InvalidState("state contains a non-finite value"));
        }
        if self.origin.identity_attractor < 0.5 - EPSILON
            || self.origin.identity_attractor > 2.0 + EPSILON
        {
            return Err(EngineError::InvalidState("identity attractor is outside bounds"));
        }
        if self.system_entropy < -EPSILON {
            return Err(EngineError::InvalidState("system entropy cannot be negative"));
        }
        Ok(())
    }

    fn push_snapshot(&mut self) {
        if self.history.len() == self.config.history_capacity {
            self.history.pop_front();
        }
        self.history.push_back(self.snapshot());
    }

    fn restore(&mut self, snapshot: EngineSnapshot) {
        self.origin = snapshot.origin;
        self.learning = snapshot.learning;
        self.selection = snapshot.selection;
        self.transparency = snapshot.transparency;
        self.silent_glyph = snapshot.silent_glyph;
        self.cycle_count = snapshot.cycle_count;
        self.elapsed_seconds = snapshot.elapsed_seconds;
        self.system_entropy = snapshot.system_entropy;
        self.rng_state = snapshot.rng_state;
    }

    fn next_unit_interval(&mut self) -> f32 {
        // Xorshift64*: deterministic and adequate for simulation exploration.
        // It is intentionally not a cryptographic random-number generator.
        let mut value = self.rng_state;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.rng_state = value;
        let mixed = value.wrapping_mul(0x2545_F491_4F6C_DD1D);
        let mantissa = (mixed >> 40) as u32;
        mantissa as f32 / ((1u32 << 24) as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        assert!(EngineConfig::default().validate().is_ok());
        assert!(QuadralithEngine::new().validate_state().is_ok());
    }

    #[test]
    fn eight_steps_form_one_hertz_phase_cycle() {
        let mut engine = QuadralithEngine::new();
        let mut report = engine.cycle_8hz(0.0).unwrap();
        assert!((report.phase_radians - TAU / 8.0).abs() < 1.0e-5);
        for _ in 1..8 {
            report = engine.cycle_8hz(0.0).unwrap();
        }
        assert!(report.phase_radians < 1.0e-4 || (TAU - report.phase_radians) < 1.0e-4);
        assert!((report.elapsed_seconds - 1.0).abs() < 1.0e-9);
    }

    #[test]
    fn invalid_input_is_rejected_without_advancing() {
        let mut engine = QuadralithEngine::new();
        let before = engine.snapshot();
        assert!(engine.cycle_8hz(f32::NAN).is_err());
        assert_eq!(engine.snapshot(), before);
    }

    #[test]
    fn rollback_restores_previous_state() {
        let mut engine = QuadralithEngine::new();
        let before = engine.snapshot();
        engine.cycle_8hz(2.0).unwrap();
        assert_ne!(engine.snapshot(), before);
        assert!(engine.rollback_last());
        assert_eq!(engine.snapshot(), before);
    }

    #[test]
    fn history_is_bounded() {
        let mut config = EngineConfig::default();
        config.history_capacity = 3;
        let mut engine = QuadralithEngine::with_config(config).unwrap();
        for _ in 0..10 {
            engine.cycle_8hz(0.0).unwrap();
        }
        assert_eq!(engine.history_len(), 3);
    }

    #[test]
    fn state_remains_bounded_under_sustained_load() {
        let mut engine = QuadralithEngine::new();
        for _ in 0..20_000 {
            engine.cycle_8hz(1.0).unwrap();
        }
        assert!(engine.validate_state().is_ok());
        assert!((0.5..=2.0).contains(&engine.origin.identity_attractor));
        assert!((0.0..=100.0).contains(&engine.origin.memory_lattice));
        assert!((0.0..=100.0).contains(&engine.learning.baldwin_scaffold));
        assert!((0.05..=2.0).contains(&engine.selection.replication_arbiter));
    }

    #[test]
    fn extreme_load_sets_and_explicit_review_clears_safety_latch() {
        let mut engine = QuadralithEngine::new();
        let report = engine.cycle_8hz(1_000.0).unwrap();
        assert!(report.safety_latch_active);
        engine.clear_safety_latch();
        assert!(!engine.silent_glyph.active);
    }

    #[test]
    fn nominal_cooling_divides_accumulated_load_by_phi() {
        let mut engine = QuadralithEngine::new();
        engine.system_entropy = 10.0;
        engine.selection.stability_filter = 1_000.0;
        let before_cycle_decay = (10.0_f32 - 0.1 * 0.125).max(0.0);
        engine.cycle_8hz(0.0).unwrap();
        let expected = before_cycle_decay / engine.config.phi_1766;
        assert!((engine.system_entropy - expected).abs() < 1.0e-5);
    }
}

