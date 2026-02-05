//! Rational Arithmetic (i128/i128)
//!
//! Exact rational number representation with automatic GCD reduction.
//! From Paper Section 2.1:
//!
//! ```rust
//! pub struct Rational {
//!     numerator: i128,    // 128-bit signed integer
//!     denominator: i128,  // Always positive, coprime with numerator
//! }
//! ```
//!
//! Properties:
//! - Addition, subtraction, multiplication: always exact
//! - Division: exact (no precision loss)
//! - Comparison: exact equality, not tolerance-based
//! - All intermediate results maintain exact fraction form

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use serde::{Deserialize, Serialize};

/// Exact rational number - i128/i128 with GCD reduction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rational {
    pub numerator: i128,
    pub denominator: i128,
}

impl Rational {
    /// Create a new rational number, automatically reduced to lowest terms
    pub fn new(n: i128, d: i128) -> Self {
        if d == 0 {
            panic!("Division by zero in Rational::new");
        }
        let g = gcd(n.abs(), d.abs());
        let sign = if d < 0 { -1 } else { 1 };
        Rational {
            numerator: sign * n / g,
            denominator: (sign * d).abs() / g,
        }
    }

    /// Create rational from integer
    pub fn from_i128(n: i128) -> Self {
        Rational { numerator: n, denominator: 1 }
    }

    /// Zero
    pub fn zero() -> Self {
        Rational { numerator: 0, denominator: 1 }
    }

    /// One
    pub fn one() -> Self {
        Rational { numerator: 1, denominator: 1 }
    }

    /// Two
    pub fn two() -> Self {
        Rational { numerator: 2, denominator: 1 }
    }

    /// Check if zero
    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    /// Check if one
    pub fn is_one(&self) -> bool {
        self.numerator == 1 && self.denominator == 1
    }

    /// Check if integer
    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Check if positive
    pub fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    /// Check if negative
    pub fn is_negative(&self) -> bool {
        self.numerator < 0
    }

    /// Absolute value
    pub fn abs(&self) -> Self {
        Rational {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }

    /// Reciprocal (1/x)
    pub fn reciprocal(&self) -> Self {
        if self.numerator == 0 {
            panic!("Cannot take reciprocal of zero");
        }
        Rational::new(self.denominator, self.numerator)
    }

    /// Power with integer exponent
    pub fn pow(&self, exp: i32) -> Self {
        if exp == 0 {
            return Rational::one();
        }
        if exp < 0 {
            return self.reciprocal().pow(-exp);
        }
        
        let mut result = Rational::one();
        let mut base = *self;
        let mut e = exp as u32;
        
        while e > 0 {
            if e & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            e >>= 1;
        }
        result
    }

    /// Compare two rationals (returns -1, 0, or 1)
    pub fn cmp_rational(&self, other: &Rational) -> i32 {
        let diff = *self - *other;
        if diff.numerator > 0 {
            1
        } else if diff.numerator < 0 {
            -1
        } else {
            0
        }
    }
    
    // ===== Explicit method versions (for compatibility with Fate/Voyager API) =====
    
    /// Add (method version)
    pub fn add(&self, other: &Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
    
    /// Subtract (method version)
    pub fn sub(&self, other: &Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
    
    /// Multiply (method version)
    pub fn mul(&self, other: &Rational) -> Rational {
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
    
    /// Divide (method version)
    pub fn div(&self, other: &Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

/// Greatest common divisor using Euclidean algorithm
fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        if other.numerator == 0 {
            panic!("Division by zero");
        }
        Rational::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl From<i128> for Rational {
    fn from(n: i128) -> Self {
        Rational::from_i128(n)
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Self {
        Rational::from_i128(n as i128)
    }
}

impl From<i32> for Rational {
    fn from(n: i32) -> Self {
        Rational::from_i128(n as i128)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare a/b vs c/d by comparing a*d vs c*b
        let lhs = self.numerator * other.denominator;
        let rhs = other.numerator * self.denominator;
        lhs.cmp(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 3);
        
        assert_eq!(a + b, Rational::new(5, 6));
        assert_eq!(a - b, Rational::new(1, 6));
        assert_eq!(a * b, Rational::new(1, 6));
        assert_eq!(a / b, Rational::new(3, 2));
    }

    #[test]
    fn test_gcd_reduction() {
        let r = Rational::new(4, 8);
        assert_eq!(r.numerator, 1);
        assert_eq!(r.denominator, 2);
    }

    #[test]
    fn test_exact_sum_s20() {
        // Compute S_20 = 1/1² + 1/2² + ... + 1/20²
        let mut sum = Rational::zero();
        for n in 1i128..=20 {
            sum = sum + Rational::new(1, n * n);
        }
        
        // Paper Section 4.1: S₂₀ = 17299975731542641/10838475198270720
        assert_eq!(sum.numerator, 17299975731542641);
        assert_eq!(sum.denominator, 10838475198270720);
    }

    #[test]
    fn test_power() {
        let half = Rational::new(1, 2);
        assert_eq!(half.pow(2), Rational::new(1, 4));
        assert_eq!(half.pow(-1), Rational::new(2, 1));
        assert_eq!(half.pow(0), Rational::one());
    }
}
