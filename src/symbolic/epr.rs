//! EPR - Extended Precision Rational
//!
//! S-wind Design Philosophy:
//! - Exact computation, never approximate
//! - No floating point
//! - Infinite precision
//! - No external dependencies
//!
//! Uses Vec<u64> to store arbitrary large integers
//! EPR = ExtendedInt / ExtendedInt, automatic GCD reduction

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Extended precision integer using base 2^64
/// 
/// Storage: limbs[0] is the least significant, limbs[n-1] is the most significant
/// Example: 2^64 + 5 = [5, 1]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExtendedInt {
    /// Number's limbs (base 2^64)
    limbs: Vec<u64>,
    /// Sign: true = negative
    negative: bool,
}

impl ExtendedInt {
    /// Create from i128
    pub fn from_i128(n: i128) -> Self {
        if n == 0 {
            return Self::zero();
        }
        
        let negative = n < 0;
        let abs_n = n.unsigned_abs();
        
        let low = abs_n as u64;
        let high = (abs_n >> 64) as u64;
        
        let limbs = if high == 0 {
            vec![low]
        } else {
            vec![low, high]
        };
        
        ExtendedInt { limbs, negative }
    }
    
    /// Zero
    pub fn zero() -> Self {
        ExtendedInt { limbs: vec![0], negative: false }
    }
    
    /// One
    pub fn one() -> Self {
        ExtendedInt { limbs: vec![1], negative: false }
    }
    
    /// Check if zero
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }
    
    /// Check if one
    pub fn is_one(&self) -> bool {
        !self.negative && self.limbs.len() == 1 && self.limbs[0] == 1
    }
    
    /// Check if negative
    pub fn is_negative(&self) -> bool {
        self.negative && !self.is_zero()
    }
    
    /// Absolute value
    pub fn abs(&self) -> Self {
        ExtendedInt {
            limbs: self.limbs.clone(),
            negative: false,
        }
    }
    
    /// Bit count
    pub fn bits(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        let top_limb = *self.limbs.last().unwrap();
        (self.limbs.len() - 1) * 64 + (64 - top_limb.leading_zeros() as usize)
    }
    
    /// Normalize (remove leading zeros)
    fn normalize(&mut self) {
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            self.limbs.pop();
        }
        if self.is_zero() {
            self.negative = false;
        }
    }
    
    /// Compare absolute values
    fn cmp_abs(&self, other: &Self) -> Ordering {
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().cmp(&other.limbs.len());
        }
        for i in (0..self.limbs.len()).rev() {
            if self.limbs[i] != other.limbs[i] {
                return self.limbs[i].cmp(&other.limbs[i]);
            }
        }
        Ordering::Equal
    }
    
    /// Addition (ignore sign, add absolute values only)
    fn add_abs(&self, other: &Self) -> Self {
        let max_len = self.limbs.len().max(other.limbs.len());
        let mut result = vec![0u64; max_len + 1];
        let mut carry = 0u128;
        
        for i in 0..max_len {
            let a = *self.limbs.get(i).unwrap_or(&0) as u128;
            let b = *other.limbs.get(i).unwrap_or(&0) as u128;
            let sum = a + b + carry;
            result[i] = sum as u64;
            carry = sum >> 64;
        }
        
        if carry > 0 {
            result[max_len] = carry as u64;
        }
        
        let mut r = ExtendedInt { limbs: result, negative: false };
        r.normalize();
        r
    }
    
    /// Subtraction (assumes self >= other in absolute value)
    fn sub_abs(&self, other: &Self) -> Self {
        let mut result = vec![0u64; self.limbs.len()];
        let mut borrow = 0i128;
        
        for i in 0..self.limbs.len() {
            let a = self.limbs[i] as i128;
            let b = *other.limbs.get(i).unwrap_or(&0) as i128;
            let diff = a - b - borrow;
            
            if diff < 0 {
                result[i] = (diff + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                result[i] = diff as u64;
                borrow = 0;
            }
        }
        
        let mut r = ExtendedInt { limbs: result, negative: false };
        r.normalize();
        r
    }
    
    /// Multiplication
    fn mul_abs(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        
        let mut result = vec![0u64; self.limbs.len() + other.limbs.len()];
        
        for i in 0..self.limbs.len() {
            let mut carry = 0u128;
            for j in 0..other.limbs.len() {
                let prod = (self.limbs[i] as u128) * (other.limbs[j] as u128)
                    + (result[i + j] as u128)
                    + carry;
                result[i + j] = prod as u64;
                carry = prod >> 64;
            }
            if carry > 0 {
                result[i + other.limbs.len()] += carry as u64;
            }
        }
        
        let mut r = ExtendedInt { limbs: result, negative: false };
        r.normalize();
        r
    }
    
    /// Division and remainder (returns (quotient, remainder))
    fn divmod(&self, other: &Self) -> (Self, Self) {
        if other.is_zero() {
            panic!("Division by zero in ExtendedInt");
        }
        
        if self.cmp_abs(other) == Ordering::Less {
            return (Self::zero(), self.clone());
        }
        
        // Simplified: for small numbers use i128 directly
        if self.limbs.len() <= 2 && other.limbs.len() <= 2 {
            let a = self.to_i128_unchecked();
            let b = other.to_i128_unchecked();
            return (Self::from_i128(a / b), Self::from_i128(a % b));
        }
        
        // 对于大数字，使用Long division
        self.long_division(other)
    }
    
    /// Convert to i128 (unchecked)
    fn to_i128_unchecked(&self) -> i128 {
        let mut result = 0i128;
        for (i, &limb) in self.limbs.iter().enumerate() {
            result |= (limb as i128) << (i * 64);
        }
        if self.negative {
            result = -result;
        }
        result
    }
    
    /// Try to convert to i128 (if within range)
    pub fn try_to_i128(&self) -> Option<i128> {
        // i128 max is about 127 bits
        if self.bits() > 127 {
            return None;
        }
        let mut result = 0i128;
        for (i, &limb) in self.limbs.iter().enumerate() {
            if i >= 2 {
                return None; // exceeds 128 bits
            }
            result |= (limb as i128) << (i * 64);
        }
        if self.negative {
            result = -result;
        }
        Some(result)
    }
    
    /// Long division
    fn long_division(&self, divisor: &Self) -> (Self, Self) {
        let mut quotient = Self::zero();
        let mut remainder = Self::zero();
        
        let total_bits = self.bits();
        
        for bit_pos in (0..total_bits).rev() {
            // remainder = remainder * 2
            remainder = remainder.add_abs(&remainder);
            
            // Get current bit
            let limb_idx = bit_pos / 64;
            let bit_idx = bit_pos % 64;
            if limb_idx < self.limbs.len() && (self.limbs[limb_idx] >> bit_idx) & 1 == 1 {
                remainder = remainder.add_abs(&Self::one());
            }
            
            // If remainder >= divisor, subtract and set quotient bit
            if remainder.cmp_abs(divisor) != Ordering::Less {
                remainder = remainder.sub_abs(divisor);
                
                // Set quotient bit
                let q_limb_idx = bit_pos / 64;
                let q_bit_idx = bit_pos % 64;
                while quotient.limbs.len() <= q_limb_idx {
                    quotient.limbs.push(0);
                }
                quotient.limbs[q_limb_idx] |= 1u64 << q_bit_idx;
            }
        }
        
        quotient.normalize();
        remainder.normalize();
        (quotient, remainder)
    }
    
    /// GCD (Euclidean algorithm)
    pub fn gcd(a: &Self, b: &Self) -> Self {
        let mut a = a.abs();
        let mut b = b.abs();
        
        while !b.is_zero() {
            let (_, r) = a.divmod(&b);
            a = b;
            b = r;
        }
        a
    }
}

impl fmt::Display for ExtendedInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        
        if self.negative {
            write!(f, "-")?;
        }
        
        // Convert to decimal string
        let mut digits = Vec::new();
        let mut n = self.abs();
        let ten = Self::from_i128(10);
        
        while !n.is_zero() {
            let (q, r) = n.divmod(&ten);
            digits.push((r.limbs[0] as u8) + b'0');
            n = q;
        }
        
        digits.reverse();
        let s: String = digits.iter().map(|&b| b as char).collect();
        write!(f, "{}", s)
    }
}

impl Add for ExtendedInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.negative == other.negative {
            let mut result = self.add_abs(&other);
            result.negative = self.negative;
            result
        } else {
            match self.cmp_abs(&other) {
                Ordering::Greater | Ordering::Equal => {
                    let mut result = self.sub_abs(&other);
                    result.negative = self.negative;
                    result
                }
                Ordering::Less => {
                    let mut result = other.sub_abs(&self);
                    result.negative = other.negative;
                    result
                }
            }
        }
    }
}

impl Sub for ExtendedInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + ExtendedInt {
            limbs: other.limbs,
            negative: !other.negative,
        }
    }
}

impl Mul for ExtendedInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = self.mul_abs(&other);
        result.negative = self.negative != other.negative;
        if result.is_zero() {
            result.negative = false;
        }
        result
    }
}

impl Div for ExtendedInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let (q, _) = self.divmod(&other);
        let mut result = q;
        result.negative = self.negative != other.negative;
        if result.is_zero() {
            result.negative = false;
        }
        result
    }
}

impl Neg for ExtendedInt {
    type Output = Self;
    fn neg(self) -> Self {
        ExtendedInt {
            limbs: self.limbs,
            negative: !self.negative,
        }
    }
}

impl PartialOrd for ExtendedInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExtendedInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.negative != other.negative {
            return if self.negative { Ordering::Less } else { Ordering::Greater };
        }
        
        let abs_cmp = self.cmp_abs(other);
        if self.negative {
            abs_cmp.reverse()
        } else {
            abs_cmp
        }
    }
}

// ============================================================================
// EPR - Extended Precision Rational
// ============================================================================

/// EPR - Extended Precision Rational
/// 
/// Unlike Rational (i128/i128), EPR has no overflow limits.
/// Can represent:
/// - Exact value of 1000!
/// - Arbitrary precision rational approximation of pi
/// - High precision computation of Riemann zeros
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EPR {
    pub numerator: ExtendedInt,
    pub denominator: ExtendedInt,
}

impl EPR {
    /// Create new EPR with automatic reduction
    pub fn new(n: ExtendedInt, d: ExtendedInt) -> Self {
        if d.is_zero() {
            panic!("Division by zero in EPR::new");
        }
        
        let g = ExtendedInt::gcd(&n, &d);
        let mut num = n / g.clone();
        let mut den = d / g;
        
        // Ensure denominator is positive
        if den.is_negative() {
            num = -num;
            den = -den;
        }
        
        EPR {
            numerator: num,
            denominator: den,
        }
    }
    
    /// Create from i128
    pub fn from_i128(n: i128) -> Self {
        EPR {
            numerator: ExtendedInt::from_i128(n),
            denominator: ExtendedInt::one(),
        }
    }
    
    /// Create from i128 ratio
    pub fn from_ratio(n: i128, d: i128) -> Self {
        Self::new(ExtendedInt::from_i128(n), ExtendedInt::from_i128(d))
    }
    
    /// Zero
    pub fn zero() -> Self {
        EPR {
            numerator: ExtendedInt::zero(),
            denominator: ExtendedInt::one(),
        }
    }
    
    /// One
    pub fn one() -> Self {
        EPR {
            numerator: ExtendedInt::one(),
            denominator: ExtendedInt::one(),
        }
    }
    
    /// Check if zero
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
    
    /// Check if one
    pub fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator.is_one()
    }
    
    /// Check if integer
    pub fn is_integer(&self) -> bool {
        self.denominator.is_one()
    }
    
    /// Try to convert to i128 (if integer and within range)
    pub fn try_to_i128(&self) -> Option<i128> {
        if !self.is_integer() {
            return None;
        }
        self.numerator.try_to_i128()
    }
    
    /// Check if positive
    pub fn is_positive(&self) -> bool {
        !self.numerator.is_negative() && !self.numerator.is_zero()
    }
    
    /// Check if negative
    pub fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }
    
    /// Absolute value
    pub fn abs(&self) -> Self {
        EPR {
            numerator: self.numerator.abs(),
            denominator: self.denominator.clone(),
        }
    }
    
    /// Reciprocal
    pub fn reciprocal(&self) -> Self {
        if self.numerator.is_zero() {
            panic!("Cannot take reciprocal of zero");
        }
        EPR::new(self.denominator.clone(), self.numerator.clone())
    }
    
    /// 整数幂
    pub fn pow(&self, exp: u32) -> Self {
        if exp == 0 {
            return EPR::one();
        }
        
        let mut result = EPR::one();
        let mut base = self.clone();
        let mut e = exp;
        
        while e > 0 {
            if e & 1 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            e >>= 1;
        }
        result
    }
    
    /// 阶乘 - 精确值，无溢出！
    pub fn factorial(n: u64) -> Self {
        let mut result = EPR::one();
        for i in 2..=n {
            result = result * EPR::from_i128(i as i128);
        }
        result
    }
    
    /// 比特数
    pub fn bits(&self) -> usize {
        self.numerator.bits().max(self.denominator.bits())
    }
    
    /// 转换为 f64 (仅用于显示，有损)
    pub fn to_f64_lossy(&self) -> f64 {
        // 这只用于显示，不用于计算
        let n = self.numerator.to_i128_unchecked() as f64;
        let d = self.denominator.to_i128_unchecked() as f64;
        n / d
    }
}

impl fmt::Display for EPR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator.is_one() {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl Add for EPR {
    type Output = EPR;
    fn add(self, other: EPR) -> EPR {
        EPR::new(
            self.numerator.clone() * other.denominator.clone() 
                + other.numerator.clone() * self.denominator.clone(),
            self.denominator * other.denominator,
        )
    }
}

impl Sub for EPR {
    type Output = EPR;
    fn sub(self, other: EPR) -> EPR {
        EPR::new(
            self.numerator.clone() * other.denominator.clone() 
                - other.numerator.clone() * self.denominator.clone(),
            self.denominator * other.denominator,
        )
    }
}

impl Mul for EPR {
    type Output = EPR;
    fn mul(self, other: EPR) -> EPR {
        EPR::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl Div for EPR {
    type Output = EPR;
    fn div(self, other: EPR) -> EPR {
        if other.numerator.is_zero() {
            panic!("Division by zero");
        }
        EPR::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl Neg for EPR {
    type Output = EPR;
    fn neg(self) -> EPR {
        EPR {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl PartialOrd for EPR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EPR {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.numerator.clone() * other.denominator.clone();
        let rhs = other.numerator.clone() * self.denominator.clone();
        lhs.cmp(&rhs)
    }
}

impl From<i128> for EPR {
    fn from(n: i128) -> Self {
        EPR::from_i128(n)
    }
}

impl From<i64> for EPR {
    fn from(n: i64) -> Self {
        EPR::from_i128(n as i128)
    }
}

impl From<i32> for EPR {
    fn from(n: i32) -> Self {
        EPR::from_i128(n as i128)
    }
}

// 从 Rational 转换
impl From<crate::symbolic::Rational> for EPR {
    fn from(r: crate::symbolic::Rational) -> Self {
        EPR::from_ratio(r.numerator, r.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extended_int_basic() {
        let a = ExtendedInt::from_i128(12345678901234567890);
        let b = ExtendedInt::from_i128(98765432109876543210);
        
        println!("a = {}", a);
        println!("b = {}", b);
        
        let sum = a.clone() + b.clone();
        println!("a + b = {}", sum);
        
        let prod = a.clone() * b.clone();
        println!("a * b = {}", prod);
        println!("a * b bits = {}", prod.bits());
    }
    
    #[test]
    fn test_epr_basic() {
        let a = EPR::from_ratio(1, 2);
        let b = EPR::from_ratio(1, 3);
        
        assert_eq!(a.clone() + b.clone(), EPR::from_ratio(5, 6));
        assert_eq!(a.clone() - b.clone(), EPR::from_ratio(1, 6));
        assert_eq!(a.clone() * b.clone(), EPR::from_ratio(1, 6));
        assert_eq!(a / b, EPR::from_ratio(3, 2));
    }
    
    #[test]
    fn test_factorial_no_overflow() {
        // 34! 接近 i128 边界
        let f34 = EPR::factorial(34);
        println!("34! = {}", f34);
        println!("34! bits = {}", f34.bits());
        assert!(f34.bits() > 120); // 34! 约 126 bits
        
        // 35! 超过 i128!
        let f35 = EPR::factorial(35);
        println!("35! = {}", f35);
        println!("35! bits = {}", f35.bits());
        assert!(f35.bits() > 128); // 真正超过 i128
        
        // 50! - 远超 i128
        let f50 = EPR::factorial(50);
        println!("50! bits = {}", f50.bits());
        assert!(f50.bits() > 200);
    }
    
    #[test]
    fn test_exact_sum() {
        // S_50 = 1/1² + 1/2² + ... + 1/50²
        let mut sum = EPR::zero();
        for n in 1i128..=50 {
            sum = sum + EPR::from_ratio(1, n * n);
        }
        
        println!("S_50 = {}", sum);
        assert!(!sum.is_zero());
    }
}
