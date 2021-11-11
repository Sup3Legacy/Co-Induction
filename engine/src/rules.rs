use crate::colors::Color;

/// All metrics that can be used to form a rule on a beedline
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NumeralMetric {
    NumberOfDifferentColors,
    CardinalOfColor(Color),
    MaxCardinal,
    MinNonZeroCardinal,
    MinColorStreak,
    MaxColorStreak,
}

/// The condition imposed on the metric
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NumeralCondition {
    G(usize),
    GE(usize),
    E(usize),
    LE(usize),
    L(usize),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BoolMetric {
    AdjacentColors(Color, Color),
    BeginsWith(Color),
    EndsWith(Color),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BoolCondition {
    Id(BoolMetric),
    Not(BoolMetric),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Condition {
    /// Composition of a metric and its associated condition
    Numeral(NumeralMetric, NumeralCondition),
    /// Boolean condition
    Boolean(BoolMetric, BoolCondition),
}

/// Compositions of conditions
pub enum Rule {

}