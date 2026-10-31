// Copyright (c) 2026 Paul Insider. See LICENSE.
//! A standalone public illustration of exact minor-unit accounting.
//!
//! Adapted from the Money abstraction in BitSwop, with a checked precision
//! boundary and strict decimal-string parsing added for this public sample.
//! This crate neither identifies an asset nor verifies an on-chain payment.

use std::fmt;

/// Largest decimal precision whose power of ten fits in a `u128`.
pub const MAX_DECIMALS: u8 = 38;

/// An unsigned amount and its decimal precision, with no floating-point input.
///
/// Precision is not an asset identifier: callers must establish matching assets
/// and networks before comparing or subtracting two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactAmount {
    minor: u128,
    decimals: u8,
}

/// A rejected precision, decimal syntax, or value outside the integer range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmountError {
    UnsupportedPrecision,
    InvalidSyntax,
    ExcessPrecision,
    Overflow,
}

impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedPrecision => "precision must be between 0 and 38",
            Self::InvalidSyntax => "expected unsigned ASCII decimal digits",
            Self::ExcessPrecision => "fraction contains more digits than the precision",
            Self::Overflow => "amount exceeds the u128 range",
        })
    }
}

impl std::error::Error for AmountError {}

impl ExactAmount {
    /// Construct directly from integer minor units.
    pub fn from_minor(minor: u128, decimals: u8) -> Result<Self, AmountError> {
        if decimals > MAX_DECIMALS {
            return Err(AmountError::UnsupportedPrecision);
        }
        Ok(Self { minor, decimals })
    }

    /// Parse a major-unit decimal string exactly, without rounding.
    ///
    /// Missing fractional digits are padded on the right. Signs, whitespace,
    /// exponents, separators, empty components and non-ASCII digits are rejected.
    /// Leading zeroes are accepted. Excess fractional digits are rejected even
    /// when they are zeroes.
    pub fn parse_decimal(input: &str, decimals: u8) -> Result<Self, AmountError> {
        Self::from_minor(0, decimals)?;
        let (whole, fraction) = match input.split_once('.') {
            Some((whole, fraction)) => {
                if fraction.is_empty() {
                    return Err(AmountError::InvalidSyntax);
                }
                (whole, Some(fraction))
            }
            None => (input, None),
        };
        if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(AmountError::InvalidSyntax);
        }
        let fraction = fraction.unwrap_or("");
        if !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(AmountError::InvalidSyntax);
        }
        if fraction.len() > usize::from(decimals) {
            return Err(AmountError::ExcessPrecision);
        }
        let scale = 10_u128.pow(u32::from(decimals));
        let whole = parse_digits(whole)?;
        let fractional = parse_digits(fraction)?;
        let padding = u32::from(decimals) - fraction.len() as u32;
        let fractional = fractional
            .checked_mul(10_u128.pow(padding))
            .ok_or(AmountError::Overflow)?;
        let minor = whole
            .checked_mul(scale)
            .and_then(|scaled| scaled.checked_add(fractional))
            .ok_or(AmountError::Overflow)?;
        Self::from_minor(minor, decimals)
    }

    pub const fn minor(self) -> u128 {
        self.minor
    }

    pub const fn decimals(self) -> u8 {
        self.decimals
    }

    /// Return `None` for mismatched precision or an unsigned underflow.
    ///
    /// The caller is responsible for matching the asset and network first.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        if self.decimals != other.decimals {
            return None;
        }
        self.minor.checked_sub(other.minor).map(|minor| Self {
            minor,
            decimals: self.decimals,
        })
    }
}

fn parse_digits(input: &str) -> Result<u128, AmountError> {
    input.bytes().try_fold(0_u128, |value, digit| {
        value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(digit - b'0')))
            .ok_or(AmountError::Overflow)
    })
}

impl fmt::Display for ExactAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.decimals == 0 {
            return write!(f, "{}", self.minor);
        }
        let scale = 10_u128.pow(u32::from(self.decimals));
        write!(
            f,
            "{}.{:0width$}",
            self.minor / scale,
            self.minor % scale,
            width = usize::from(self.decimals)
        )
    }
}
