//! Symbolic Expression System
//!
//! From Paper Section 2.2:
//!
//! ```text
//! pub enum Expr {
//!     Sym(Symbol),           // Named variable (e.g., "n", "x")
//!     Num(Rational),         // Exact rational constant
//!     Const(PhysicsConst),   // Physical constants (π, e, c, ℏ)
//!     Add(Box<Expr>, Box<Expr>),
//!     Sub(Box<Expr>, Box<Expr>),
//!     Mul(Box<Expr>, Box<Expr>),
//!     Div(Box<Expr>, Box<Expr>),
//!     Pow(Box<Expr>, Box<Expr>),
//!     Sqrt(Box<Expr>),
//!     Derivative { expr, var, order },
//!     Integral { expr, var },
//!     // ... more operations
//! }
//! ```
//!
//! Capabilities:
//! - Full symbolic algebra without numerical evaluation
//! - Symbolic differentiation and integration
//! - Pattern matching and simplification
//! - LaTeX rendering

use std::fmt;
use super::Rational;
use serde::{Deserialize, Serialize};

/// Symbol identifier for variables
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn new(name: &str) -> Self {
        Symbol(name.to_string())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Physical/Mathematical constants - kept symbolic, never evaluated to floating point
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhysicsConst {
    // Mathematical constants
    Pi,           // π
    E,            // e (Euler's number)
    Gamma,        // γ (Euler-Mascheroni constant)
    
    // Fundamental physical constants (CODATA 2022)
    C,            // c (speed of light) - exact: 299792458 m/s
    Hbar,         // ℏ (reduced Planck constant)
    H,            // h (Planck constant) - exact: 6.62607015×10⁻³⁴ J·s
    G,            // G (gravitational constant)
    KBoltzmann,   // k_B (Boltzmann constant) - exact: 1.380649×10⁻²³ J/K
    Epsilon0,     // ε₀ (vacuum permittivity)
    Mu0,          // μ₀ (vacuum permeability)
    ElemCharge,   // e (elementary charge) - exact: 1.602176634×10⁻¹⁹ C
    Alpha,        // α (fine-structure constant) ≈ 1/137
    
    // Planck units (derived)
    LPlanck,      // l_P = √(ℏG/c³) (Planck length)
    TPlanck,      // t_P = √(ℏG/c⁵) (Planck time)
    MPlanck,      // m_P = √(ℏc/G) (Planck mass)
    TempPlanck,   // T_P = √(ℏc⁵/Gk_B²) (Planck temperature)
    QPlanck,      // q_P = √(4πε₀ℏc) (Planck charge)
    EPlanck,      // E_P = m_P·c² (Planck energy)
}

impl fmt::Display for PhysicsConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicsConst::Pi => write!(f, "π"),
            PhysicsConst::E => write!(f, "e"),
            PhysicsConst::Gamma => write!(f, "γ"),
            PhysicsConst::C => write!(f, "c"),
            PhysicsConst::Hbar => write!(f, "ℏ"),
            PhysicsConst::H => write!(f, "h"),
            PhysicsConst::G => write!(f, "G"),
            PhysicsConst::KBoltzmann => write!(f, "k_B"),
            PhysicsConst::Epsilon0 => write!(f, "ε₀"),
            PhysicsConst::Mu0 => write!(f, "μ₀"),
            PhysicsConst::ElemCharge => write!(f, "e"),
            PhysicsConst::Alpha => write!(f, "α"),
            PhysicsConst::LPlanck => write!(f, "ℓ_P"),
            PhysicsConst::TPlanck => write!(f, "t_P"),
            PhysicsConst::MPlanck => write!(f, "m_P"),
            PhysicsConst::TempPlanck => write!(f, "T_P"),
            PhysicsConst::QPlanck => write!(f, "q_P"),
            PhysicsConst::EPlanck => write!(f, "E_P"),
        }
    }
}

impl PhysicsConst {
    /// Convert to LaTeX
    pub fn to_latex(&self) -> &'static str {
        match self {
            PhysicsConst::Pi => "\\pi",
            PhysicsConst::E => "e",
            PhysicsConst::Gamma => "\\gamma",
            PhysicsConst::C => "c",
            PhysicsConst::Hbar => "\\hbar",
            PhysicsConst::H => "h",
            PhysicsConst::G => "G",
            PhysicsConst::KBoltzmann => "k_B",
            PhysicsConst::Epsilon0 => "\\varepsilon_0",
            PhysicsConst::Mu0 => "\\mu_0",
            PhysicsConst::ElemCharge => "e",
            PhysicsConst::Alpha => "\\alpha",
            PhysicsConst::LPlanck => "\\ell_P",
            PhysicsConst::TPlanck => "t_P",
            PhysicsConst::MPlanck => "m_P",
            PhysicsConst::TempPlanck => "T_P",
            PhysicsConst::QPlanck => "q_P",
            PhysicsConst::EPlanck => "E_P",
        }
    }
    
    /// Get algebraic definition (for derived constants)
    /// Returns Some(Expr) if this constant has an algebraic definition
    pub fn algebraic_definition(&self) -> Option<Expr> {
        match self {
            // Planck length: ℓ_P = √(ℏG/c³)
            PhysicsConst::LPlanck => Some(Expr::Sqrt(Box::new(
                Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Const(PhysicsConst::G))
                    )),
                    Box::new(Expr::Pow(
                        Box::new(Expr::Const(PhysicsConst::C)),
                        Box::new(Expr::Num(Rational::new(3, 1)))
                    ))
                )
            ))),
            
            // Planck time: t_P = √(ℏG/c⁵)
            PhysicsConst::TPlanck => Some(Expr::Sqrt(Box::new(
                Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Const(PhysicsConst::G))
                    )),
                    Box::new(Expr::Pow(
                        Box::new(Expr::Const(PhysicsConst::C)),
                        Box::new(Expr::Num(Rational::new(5, 1)))
                    ))
                )
            ))),
            
            // Planck mass: m_P = √(ℏc/G)
            PhysicsConst::MPlanck => Some(Expr::Sqrt(Box::new(
                Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Const(PhysicsConst::C))
                    )),
                    Box::new(Expr::Const(PhysicsConst::G))
                )
            ))),
            
            // Planck temperature: T_P = √(ℏc⁵/Gk_B²)
            PhysicsConst::TempPlanck => Some(Expr::Sqrt(Box::new(
                Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Pow(
                            Box::new(Expr::Const(PhysicsConst::C)),
                            Box::new(Expr::Num(Rational::new(5, 1)))
                        ))
                    )),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::G)),
                        Box::new(Expr::Pow(
                            Box::new(Expr::Const(PhysicsConst::KBoltzmann)),
                            Box::new(Expr::Num(Rational::new(2, 1)))
                        ))
                    ))
                )
            ))),
            
            // Planck charge: q_P = √(4πε₀ℏc)
            PhysicsConst::QPlanck => Some(Expr::Sqrt(Box::new(
                Expr::Mul(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Num(Rational::new(4, 1))),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Const(PhysicsConst::Pi)),
                            Box::new(Expr::Const(PhysicsConst::Epsilon0))
                        ))
                    )),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Const(PhysicsConst::C))
                    ))
                )
            ))),
            
            // Planck energy: E_P = m_P·c²
            PhysicsConst::EPlanck => Some(Expr::Mul(
                Box::new(Expr::Const(PhysicsConst::MPlanck)),
                Box::new(Expr::Pow(
                    Box::new(Expr::Const(PhysicsConst::C)),
                    Box::new(Expr::Num(Rational::new(2, 1)))
                ))
            )),
            
            // Fine-structure constant: α = e²/(4πε₀ℏc)
            PhysicsConst::Alpha => Some(Expr::Div(
                Box::new(Expr::Pow(
                    Box::new(Expr::Const(PhysicsConst::ElemCharge)),
                    Box::new(Expr::Num(Rational::new(2, 1)))
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Num(Rational::new(4, 1))),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Const(PhysicsConst::Pi)),
                            Box::new(Expr::Const(PhysicsConst::Epsilon0))
                        ))
                    )),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(PhysicsConst::Hbar)),
                        Box::new(Expr::Const(PhysicsConst::C))
                    ))
                ))
            )),
            
            // h = 2πℏ
            PhysicsConst::H => Some(Expr::Mul(
                Box::new(Expr::Num(Rational::new(2, 1))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Const(PhysicsConst::Pi)),
                    Box::new(Expr::Const(PhysicsConst::Hbar))
                ))
            )),
            
            // Fundamental constants have no algebraic definition
            _ => None,
        }
    }
    
    /// Check if this is a fundamental (non-derived) constant
    pub fn is_fundamental(&self) -> bool {
        matches!(self, 
            PhysicsConst::C | PhysicsConst::Hbar | PhysicsConst::G | 
            PhysicsConst::KBoltzmann | PhysicsConst::Epsilon0 | PhysicsConst::ElemCharge |
            PhysicsConst::Pi | PhysicsConst::E | PhysicsConst::Gamma
        )
    }
}

/// Symbolic Expression - the core algebraic object
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Symbolic variable
    Sym(Symbol),
    /// Exact rational number
    Num(Rational),
    /// Physical/Mathematical constant (symbolic, not numeric)
    Const(PhysicsConst),
    /// Addition
    Add(Box<Expr>, Box<Expr>),
    /// Subtraction
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication
    Mul(Box<Expr>, Box<Expr>),
    /// Division
    Div(Box<Expr>, Box<Expr>),
    /// Power (base, exponent)
    Pow(Box<Expr>, Box<Expr>),
    /// Negation
    Neg(Box<Expr>),
    /// Square root (kept symbolic)
    Sqrt(Box<Expr>),
    /// Natural logarithm (kept symbolic)
    Ln(Box<Expr>),
    /// Exponential (kept symbolic)
    Exp(Box<Expr>),
    /// Sine (kept symbolic)
    Sin(Box<Expr>),
    /// Cosine (kept symbolic)
    Cos(Box<Expr>),
    /// Derivative
    Derivative {
        expr: Box<Expr>,
        var: Symbol,
        order: u32,
    },
    /// Integral
    Integral {
        expr: Box<Expr>,
        var: Symbol,
    },
    /// Summation (variable, start, end, body)
    Sum {
        var: Symbol,
        start: Box<Expr>,
        end: Box<Expr>,
        body: Box<Expr>,
    },
    /// Product (variable, start, end, body)
    Prod {
        var: Symbol,
        start: Box<Expr>,
        end: Box<Expr>,
        body: Box<Expr>,
    },
    /// Limit
    Limit {
        var: Symbol,
        point: Box<Expr>,
        body: Box<Expr>,
    },
}

impl Expr {
    // ==================== Constructors ====================
    
    /// Create symbol
    pub fn sym(name: &str) -> Expr {
        Expr::Sym(Symbol::new(name))
    }

    /// Create integer
    pub fn num(n: i128) -> Expr {
        Expr::Num(Rational::from_i128(n))
    }

    /// Create rational
    pub fn rat(num: i128, den: i128) -> Expr {
        Expr::Num(Rational::new(num, den))
    }

    /// Create from Rational
    pub fn from_rational(r: Rational) -> Expr {
        Expr::Num(r)
    }

    /// Pi constant
    pub fn pi() -> Expr {
        Expr::Const(PhysicsConst::Pi)
    }

    /// Euler's number e
    pub fn e() -> Expr {
        Expr::Const(PhysicsConst::E)
    }

    /// Euler-Mascheroni constant γ
    pub fn gamma() -> Expr {
        Expr::Const(PhysicsConst::Gamma)
    }

    // ==================== Operations ====================
    // Note: These methods are intentionally named to match mathematical DSL,
    // not to implement std::ops traits (symbolic expressions are lazy)

    /// Addition
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Expr) -> Expr {
        Expr::Add(Box::new(self), Box::new(other))
    }

    /// Subtraction
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Expr) -> Expr {
        Expr::Sub(Box::new(self), Box::new(other))
    }

    /// Multiplication
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Expr) -> Expr {
        Expr::Mul(Box::new(self), Box::new(other))
    }

    /// Division
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, other: Expr) -> Expr {
        Expr::Div(Box::new(self), Box::new(other))
    }

    /// Power
    pub fn pow(self, exp: Expr) -> Expr {
        Expr::Pow(Box::new(self), Box::new(exp))
    }

    /// Square root
    pub fn sqrt(self) -> Expr {
        Expr::Sqrt(Box::new(self))
    }

    /// Natural logarithm
    pub fn ln(self) -> Expr {
        Expr::Ln(Box::new(self))
    }

    /// Exponential
    pub fn exp(self) -> Expr {
        Expr::Exp(Box::new(self))
    }

    /// Negation
    #[allow(clippy::should_implement_trait)]
    pub fn neg(self) -> Expr {
        Expr::Neg(Box::new(self))
    }

    // ==================== Symbolic Operations ====================

    /// Substitute variable with expression
    pub fn substitute(&self, var: &str, replacement: &Expr) -> Expr {
        let sym = Symbol::new(var);
        match self {
            Expr::Sym(s) if *s == sym => replacement.clone(),
            Expr::Sym(_) | Expr::Num(_) | Expr::Const(_) => self.clone(),
            Expr::Add(a, b) => Expr::Add(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Sub(a, b) => Expr::Sub(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Mul(a, b) => Expr::Mul(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Div(a, b) => Expr::Div(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Pow(base, exp) => Expr::Pow(
                Box::new(base.substitute(var, replacement)),
                Box::new(exp.substitute(var, replacement)),
            ),
            Expr::Neg(a) => Expr::Neg(Box::new(a.substitute(var, replacement))),
            Expr::Sqrt(a) => Expr::Sqrt(Box::new(a.substitute(var, replacement))),
            Expr::Ln(a) => Expr::Ln(Box::new(a.substitute(var, replacement))),
            Expr::Exp(a) => Expr::Exp(Box::new(a.substitute(var, replacement))),
            Expr::Sin(a) => Expr::Sin(Box::new(a.substitute(var, replacement))),
            Expr::Cos(a) => Expr::Cos(Box::new(a.substitute(var, replacement))),
            _ => self.clone(),
        }
    }

    /// Simplify expression (basic algebraic simplification)
    pub fn simplify(&self) -> Expr {
        match self {
            // x + 0 = x, 0 + x = x
            Expr::Add(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Expr::Num(n), _) if n.is_zero() => b,
                    (_, Expr::Num(n)) if n.is_zero() => a,
                    (Expr::Num(n1), Expr::Num(n2)) => Expr::Num(*n1 + *n2),
                    _ => Expr::Add(Box::new(a), Box::new(b)),
                }
            }
            // x - 0 = x
            Expr::Sub(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (_, Expr::Num(n)) if n.is_zero() => a,
                    (Expr::Num(n1), Expr::Num(n2)) => Expr::Num(*n1 - *n2),
                    _ => Expr::Sub(Box::new(a), Box::new(b)),
                }
            }
            // x * 0 = 0, x * 1 = x, 1 * x = x
            Expr::Mul(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Expr::Num(n), _) | (_, Expr::Num(n)) if n.is_zero() => Expr::num(0),
                    (Expr::Num(n), _) if n.is_one() => b,
                    (_, Expr::Num(n)) if n.is_one() => a,
                    (Expr::Num(n1), Expr::Num(n2)) => Expr::Num(*n1 * *n2),
                    _ => Expr::Mul(Box::new(a), Box::new(b)),
                }
            }
            // x / 1 = x
            Expr::Div(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (_, Expr::Num(n)) if n.is_one() => a,
                    (Expr::Num(n1), Expr::Num(n2)) if !n2.is_zero() => Expr::Num(*n1 / *n2),
                    _ => Expr::Div(Box::new(a), Box::new(b)),
                }
            }
            // x^0 = 1, x^1 = x
            Expr::Pow(base, exp) => {
                let base = base.simplify();
                let exp = exp.simplify();
                match &exp {
                    Expr::Num(n) if n.is_zero() => Expr::num(1),
                    Expr::Num(n) if n.is_one() => base,
                    _ => Expr::Pow(Box::new(base), Box::new(exp)),
                }
            }
            // sqrt(0) = 0, sqrt(1) = 1
            Expr::Sqrt(inner) => {
                let inner = inner.simplify();
                match &inner {
                    Expr::Num(n) if n.is_zero() => Expr::num(0),
                    Expr::Num(n) if n.is_one() => Expr::num(1),
                    _ => Expr::Sqrt(Box::new(inner)),
                }
            }
            _ => self.clone(),
        }
    }

    /// Convert to LaTeX string
    pub fn to_latex(&self) -> String {
        match self {
            Expr::Sym(s) => s.0.clone(),
            Expr::Num(r) => {
                if r.denominator == 1 {
                    format!("{}", r.numerator)
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numerator, r.denominator)
                }
            }
            Expr::Const(c) => c.to_latex().to_string(),
            Expr::Add(a, b) => format!("{} + {}", a.to_latex(), b.to_latex()),
            Expr::Sub(a, b) => format!("{} - {}", a.to_latex(), b.to_latex()),
            Expr::Mul(a, b) => format!("({}) \\cdot ({})", a.to_latex(), b.to_latex()),
            Expr::Div(a, b) => format!("\\frac{{{}}}{{{}}}", a.to_latex(), b.to_latex()),
            Expr::Pow(base, exp) => format!("{}^{{{}}}", base.to_latex(), exp.to_latex()),
            Expr::Neg(a) => format!("-{}", a.to_latex()),
            Expr::Sqrt(a) => format!("\\sqrt{{{}}}", a.to_latex()),
            Expr::Ln(a) => format!("\\ln({})", a.to_latex()),
            Expr::Exp(a) => format!("e^{{{}}}", a.to_latex()),
            Expr::Sin(a) => format!("\\sin({})", a.to_latex()),
            Expr::Cos(a) => format!("\\cos({})", a.to_latex()),
            Expr::Derivative { expr, var, order } => {
                if *order == 1 {
                    format!("\\frac{{d}}{{d{}}}\\left({}\\right)", var, expr.to_latex())
                } else {
                    format!("\\frac{{d^{}}}{{d{}^{}}}\\left({}\\right)", order, var, order, expr.to_latex())
                }
            }
            Expr::Integral { expr, var } => {
                format!("\\int {} \\, d{}", expr.to_latex(), var)
            }
            Expr::Sum { var, start, end, body } => {
                format!("\\sum_{{{}={}}}^{{{}}} {}", var, start.to_latex(), end.to_latex(), body.to_latex())
            }
            Expr::Prod { var, start, end, body } => {
                format!("\\prod_{{{}={}}}^{{{}}} {}", var, start.to_latex(), end.to_latex(), body.to_latex())
            }
            Expr::Limit { var, point, body } => {
                format!("\\lim_{{{} \\to {}}} {}", var, point.to_latex(), body.to_latex())
            }
        }
    }

    /// Check if expression contains a variable
    pub fn contains_var(&self, var: &str) -> bool {
        let sym = Symbol::new(var);
        match self {
            Expr::Sym(s) => *s == sym,
            Expr::Num(_) | Expr::Const(_) => false,
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => {
                a.contains_var(var) || b.contains_var(var)
            }
            Expr::Neg(a) | Expr::Sqrt(a) | Expr::Ln(a) | Expr::Exp(a) | Expr::Sin(a) | Expr::Cos(a) => {
                a.contains_var(var)
            }
            Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => {
                expr.contains_var(var)
            }
            Expr::Sum { body, .. } | Expr::Prod { body, .. } | Expr::Limit { body, .. } => {
                body.contains_var(var)
            }
        }
    }
    
    /// Try to evaluate expression to a Rational (returns None if contains unsubstituted variables or transcendental functions)
    pub fn try_eval(&self) -> Option<Rational> {
        match self {
            Expr::Num(r) => Some(*r),
            Expr::Sym(_) => None, // Unsubstituted variable
            Expr::Const(_) => None, // Can't evaluate transcendental constants exactly
            
            Expr::Add(a, b) => {
                let a_val = a.try_eval()?;
                let b_val = b.try_eval()?;
                Some(a_val + b_val)
            }
            Expr::Sub(a, b) => {
                let a_val = a.try_eval()?;
                let b_val = b.try_eval()?;
                Some(a_val - b_val)
            }
            Expr::Mul(a, b) => {
                let a_val = a.try_eval()?;
                let b_val = b.try_eval()?;
                Some(a_val * b_val)
            }
            Expr::Div(a, b) => {
                let a_val = a.try_eval()?;
                let b_val = b.try_eval()?;
                if b_val.is_zero() {
                    None // Division by zero
                } else {
                    Some(a_val / b_val)
                }
            }
            Expr::Neg(a) => {
                let a_val = a.try_eval()?;
                Some(Rational::zero() - a_val)
            }
            Expr::Pow(base, exp) => {
                let base_val = base.try_eval()?;
                let exp_val = exp.try_eval()?;
                // Only support integer exponents
                if exp_val.denominator == 1 && exp_val.numerator >= 0 {
                    let mut result = Rational::one();
                    for _ in 0..exp_val.numerator {
                        result = result * base_val;
                    }
                    Some(result)
                } else if exp_val.denominator == 1 && exp_val.numerator < 0 && !base_val.is_zero() {
                    let mut result = Rational::one();
                    for _ in 0..(-exp_val.numerator) {
                        result = result * base_val;
                    }
                    Some(Rational::one() / result)
                } else {
                    None // Non-integer exponent
                }
            }
            
            // Transcendental functions cannot be evaluated exactly
            Expr::Sqrt(_) | Expr::Ln(_) | Expr::Exp(_) | Expr::Sin(_) | Expr::Cos(_) => None,
            
            // Complex expressions not supported
            Expr::Derivative { .. } | Expr::Integral { .. } | Expr::Sum { .. } | Expr::Prod { .. } | Expr::Limit { .. } => None,
        }
    }
    
    /// Evaluate expression at a specific value (substitute variable and evaluate)
    pub fn eval_at(&self, var: &str, value: Rational) -> Option<Rational> {
        let substituted = self.substitute(var, &Expr::from_rational(value));
        substituted.simplify().try_eval()
    }
    
    /// Check if this is a pure number (no variables or constants)
    pub fn is_numeric(&self) -> bool {
        self.try_eval().is_some()
    }
    
    // ==================== Symbolic Calculus ====================
    
    /// Symbolic differentiation with respect to a variable
    /// 
    /// Implements standard differentiation rules:
    /// - d/dx(c) = 0                   (constant)
    /// - d/dx(x) = 1                   (variable)
    /// - d/dx(u+v) = du/dx + dv/dx     (sum)
    /// - d/dx(u*v) = u*dv/dx + v*du/dx (product)
    /// - d/dx(u/v) = (v*du - u*dv)/v²  (quotient)
    /// - d/dx(u^n) = n*u^(n-1)*du/dx   (power)
    /// - d/dx(sin(u)) = cos(u)*du/dx   (chain rule)
    /// - d/dx(cos(u)) = -sin(u)*du/dx  (chain rule)
    /// - d/dx(exp(u)) = exp(u)*du/dx   (chain rule)
    /// - d/dx(ln(u)) = (1/u)*du/dx     (chain rule)
    pub fn differentiate(&self, var: &str) -> Expr {
        let sym = Symbol::new(var);
        match self {
            // Constants
            Expr::Num(_) | Expr::Const(_) => Expr::num(0),
            
            // Variable
            Expr::Sym(s) => {
                if *s == sym {
                    Expr::num(1)
                } else {
                    Expr::num(0)
                }
            }
            
            // Sum rule: d(u+v) = du + dv
            Expr::Add(u, v) => {
                u.differentiate(var).add(v.differentiate(var)).simplify()
            }
            
            // Difference rule: d(u-v) = du - dv
            Expr::Sub(u, v) => {
                u.differentiate(var).sub(v.differentiate(var)).simplify()
            }
            
            // Product rule: d(u*v) = u*dv + v*du
            Expr::Mul(u, v) => {
                let du = u.differentiate(var);
                let dv = v.differentiate(var);
                // u*dv + v*du
                (*u.clone()).mul(dv).add((*v.clone()).mul(du)).simplify()
            }
            
            // Quotient rule: d(u/v) = (v*du - u*dv) / v²
            Expr::Div(u, v) => {
                let du = u.differentiate(var);
                let dv = v.differentiate(var);
                // (v*du - u*dv) / v²
                let numerator = (*v.clone()).mul(du).sub((*u.clone()).mul(dv));
                let denominator = (*v.clone()).mul(*v.clone());
                numerator.div(denominator).simplify()
            }
            
            // Power rule: d(u^n) = n*u^(n-1)*du
            Expr::Pow(base, exp) => {
                if !exp.contains_var(var) {
                    // d(u^c) = c*u^(c-1)*du where c is constant in var
                    let du = base.differentiate(var);
                    let new_exp = (*exp.clone()).sub(Expr::num(1));
                    (*exp.clone())
                        .mul(Expr::Pow(base.clone(), Box::new(new_exp)))
                        .mul(du)
                        .simplify()
                } else if !base.contains_var(var) {
                    // d(c^v) = c^v * ln(c) * dv where c is constant in var
                    let dv = exp.differentiate(var);
                    self.clone()
                        .mul(Expr::Ln(base.clone()))
                        .mul(dv)
                        .simplify()
                } else {
                    // General case: d(u^v) = u^v * (v*du/u + ln(u)*dv)
                    // Leave as symbolic derivative for now
                    Expr::Derivative {
                        expr: Box::new(self.clone()),
                        var: sym,
                        order: 1,
                    }
                }
            }
            
            // Negation: d(-u) = -du
            Expr::Neg(u) => {
                Expr::Neg(Box::new(u.differentiate(var))).simplify()
            }
            
            // Chain rule for sqrt: d(sqrt(u)) = du / (2*sqrt(u))
            Expr::Sqrt(u) => {
                let du = u.differentiate(var);
                du.div(Expr::num(2).mul(Expr::Sqrt(u.clone()))).simplify()
            }
            
            // Chain rule for ln: d(ln(u)) = du / u
            Expr::Ln(u) => {
                let du = u.differentiate(var);
                du.div(*u.clone()).simplify()
            }
            
            // Chain rule for exp: d(exp(u)) = exp(u) * du
            Expr::Exp(u) => {
                let du = u.differentiate(var);
                Expr::Exp(u.clone()).mul(du).simplify()
            }
            
            // Chain rule for sin: d(sin(u)) = cos(u) * du
            Expr::Sin(u) => {
                let du = u.differentiate(var);
                Expr::Cos(u.clone()).mul(du).simplify()
            }
            
            // Chain rule for cos: d(cos(u)) = -sin(u) * du
            Expr::Cos(u) => {
                let du = u.differentiate(var);
                Expr::Neg(Box::new(Expr::Sin(u.clone()))).mul(du).simplify()
            }
            
            // Derivative of derivative: compute higher order
            Expr::Derivative { expr, var: v, order } => {
                if v.0 == var {
                    Expr::Derivative {
                        expr: expr.clone(),
                        var: v.clone(),
                        order: order + 1,
                    }
                } else {
                    // Mixed partial derivative
                    Expr::Derivative {
                        expr: Box::new(Expr::Derivative {
                            expr: expr.clone(),
                            var: v.clone(),
                            order: *order,
                        }),
                        var: sym,
                        order: 1,
                    }
                }
            }
            
            // Integral: d/dx ∫ f dx = f (if same variable)
            Expr::Integral { expr, var: v } => {
                if v.0 == var {
                    *expr.clone()
                } else {
                    Expr::Integral {
                        expr: Box::new(expr.differentiate(var)),
                        var: v.clone(),
                    }
                }
            }
            
            // Sum/Prod/Limit: leave as symbolic
            _ => Expr::Derivative {
                expr: Box::new(self.clone()),
                var: sym,
                order: 1,
            },
        }
    }
    
    /// Symbolic integration with respect to a variable
    /// 
    /// Implements basic integration rules:
    /// - ∫ c dx = c*x                      (constant)
    /// - ∫ x dx = x²/2                     (variable)
    /// - ∫ x^n dx = x^(n+1)/(n+1)          (power, n ≠ -1)
    /// - ∫ 1/x dx = ln|x|                  (reciprocal)
    /// - ∫ (u+v) dx = ∫u dx + ∫v dx        (sum)
    /// - ∫ c*f dx = c * ∫f dx              (constant multiple)
    /// - ∫ exp(x) dx = exp(x)              (exponential)
    /// - ∫ sin(x) dx = -cos(x)             (sine)
    /// - ∫ cos(x) dx = sin(x)              (cosine)
    /// 
    /// Note: For complex expressions, returns symbolic integral
    pub fn integrate(&self, var: &str) -> Expr {
        let sym = Symbol::new(var);
        match self {
            // Constant: ∫ c dx = c*x
            Expr::Num(_) | Expr::Const(_) => {
                self.clone().mul(Expr::sym(var))
            }
            
            // Variable: ∫ x dx = x²/2
            Expr::Sym(s) if *s == sym => {
                Expr::Pow(Box::new(Expr::sym(var)), Box::new(Expr::num(2)))
                    .div(Expr::num(2))
            }
            
            // Other variable: ∫ y dx = y*x (treat as constant)
            Expr::Sym(_) => {
                self.clone().mul(Expr::sym(var))
            }
            
            // Sum rule: ∫(u+v) = ∫u + ∫v
            Expr::Add(u, v) => {
                u.integrate(var).add(v.integrate(var)).simplify()
            }
            
            // Difference rule: ∫(u-v) = ∫u - ∫v
            Expr::Sub(u, v) => {
                u.integrate(var).sub(v.integrate(var)).simplify()
            }
            
            // Constant multiple: ∫(c*u) = c*∫u where c doesn't contain var
            Expr::Mul(u, v) => {
                if !u.contains_var(var) {
                    // c * ∫v
                    (*u.clone()).mul(v.integrate(var)).simplify()
                } else if !v.contains_var(var) {
                    // c * ∫u
                    (*v.clone()).mul(u.integrate(var)).simplify()
                } else {
                    // General case: leave as symbolic
                    Expr::Integral {
                        expr: Box::new(self.clone()),
                        var: sym,
                    }
                }
            }
            
            // Power rule: ∫x^n dx = x^(n+1)/(n+1)
            Expr::Pow(base, exp) => {
                // Only handle simple cases: x^n where n is constant
                if let (Expr::Sym(s), Expr::Num(n)) = (base.as_ref(), exp.as_ref()) {
                    if *s == sym {
                        if n.numerator == -1 && n.denominator == 1 {
                            // ∫1/x dx = ln(x)
                            return Expr::Ln(Box::new(Expr::sym(var)));
                        }
                        // ∫x^n dx = x^(n+1)/(n+1)
                        let new_exp = *n + Rational::one();
                        return Expr::Pow(
                            base.clone(),
                            Box::new(Expr::Num(new_exp)),
                        ).div(Expr::Num(new_exp)).simplify();
                    }
                }
                // General case: leave as symbolic
                Expr::Integral {
                    expr: Box::new(self.clone()),
                    var: sym,
                }
            }
            
            // ∫-u = -∫u
            Expr::Neg(u) => {
                Expr::Neg(Box::new(u.integrate(var))).simplify()
            }
            
            // ∫exp(x) dx = exp(x)
            Expr::Exp(u) => {
                if let Expr::Sym(s) = u.as_ref() {
                    if *s == sym {
                        return Expr::Exp(u.clone());
                    }
                }
                // Check if u = c*x
                if let Expr::Mul(c, x) = u.as_ref() {
                    if !c.contains_var(var) {
                        if let Expr::Sym(s) = x.as_ref() {
                            if *s == sym {
                                // ∫exp(c*x) dx = exp(c*x)/c
                                return Expr::Exp(u.clone()).div(*c.clone());
                            }
                        }
                    }
                }
                Expr::Integral {
                    expr: Box::new(self.clone()),
                    var: sym,
                }
            }
            
            // ∫sin(x) dx = -cos(x)
            Expr::Sin(u) => {
                if let Expr::Sym(s) = u.as_ref() {
                    if *s == sym {
                        return Expr::Neg(Box::new(Expr::Cos(u.clone())));
                    }
                }
                Expr::Integral {
                    expr: Box::new(self.clone()),
                    var: sym,
                }
            }
            
            // ∫cos(x) dx = sin(x)
            Expr::Cos(u) => {
                if let Expr::Sym(s) = u.as_ref() {
                    if *s == sym {
                        return Expr::Sin(u.clone());
                    }
                }
                Expr::Integral {
                    expr: Box::new(self.clone()),
                    var: sym,
                }
            }
            
            // ∫1/u where u doesn't contain var: (1/u)*x
            Expr::Div(num, den) => {
                if let Expr::Num(n) = num.as_ref() {
                    if n.is_one() {
                        if let Expr::Sym(s) = den.as_ref() {
                            if *s == sym {
                                // ∫1/x dx = ln(x)
                                return Expr::Ln(Box::new(Expr::sym(var)));
                            }
                        }
                    }
                }
                // General case
                Expr::Integral {
                    expr: Box::new(self.clone()),
                    var: sym,
                }
            }
            
            // For complex expressions, leave as symbolic integral
            _ => Expr::Integral {
                expr: Box::new(self.clone()),
                var: sym,
            },
        }
    }
    
    /// Compute nth derivative
    pub fn nth_derivative(&self, var: &str, n: u32) -> Expr {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.differentiate(var);
        }
        result
    }
    
    /// Definite integral (if possible to compute)
    pub fn definite_integral(&self, var: &str, lower: Rational, upper: Rational) -> Option<Rational> {
        let antiderivative = self.integrate(var);
        let upper_val = antiderivative.eval_at(var, upper)?;
        let lower_val = antiderivative.eval_at(var, lower)?;
        Some(upper_val - lower_val)
    }
    
    /// Get all free variables in the expression
    pub fn free_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    
    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            Expr::Sym(s) => vars.push(s.0.clone()),
            Expr::Num(_) | Expr::Const(_) => {}
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            Expr::Neg(a) | Expr::Sqrt(a) | Expr::Ln(a) | Expr::Exp(a) | Expr::Sin(a) | Expr::Cos(a) => {
                a.collect_vars(vars);
            }
            Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => {
                expr.collect_vars(vars);
            }
            Expr::Sum { body, .. } | Expr::Prod { body, .. } | Expr::Limit { body, .. } => {
                body.collect_vars(vars);
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Num(r) => write!(f, "{}", r),
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Add(a, b) => write!(f, "({} + {})", a, b),
            Expr::Sub(a, b) => write!(f, "({} - {})", a, b),
            Expr::Mul(a, b) => write!(f, "({} × {})", a, b),
            Expr::Div(a, b) => write!(f, "({} / {})", a, b),
            Expr::Pow(base, exp) => write!(f, "{}^{}", base, exp),
            Expr::Neg(a) => write!(f, "-{}", a),
            Expr::Sqrt(a) => write!(f, "√({})", a),
            Expr::Ln(a) => write!(f, "ln({})", a),
            Expr::Exp(a) => write!(f, "exp({})", a),
            Expr::Sin(a) => write!(f, "sin({})", a),
            Expr::Cos(a) => write!(f, "cos({})", a),
            Expr::Derivative { expr, var, order } => {
                write!(f, "d^{}/d{}^{}({})", order, var, order, expr)
            }
            Expr::Integral { expr, var } => write!(f, "∫({})d{}", expr, var),
            Expr::Sum { var, start, end, body } => {
                write!(f, "Σ[{}={}..{}]({})", var, start, end, body)
            }
            Expr::Prod { var, start, end, body } => {
                write!(f, "Π[{}={}..{}]({})", var, start, end, body)
            }
            Expr::Limit { var, point, body } => {
                write!(f, "lim[{}→{}]({})", var, point, body)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        // x + 0 = x
        let expr = Expr::sym("x").add(Expr::num(0));
        assert_eq!(format!("{}", expr.simplify()), "x");

        // x * 1 = x
        let expr = Expr::sym("x").mul(Expr::num(1));
        assert_eq!(format!("{}", expr.simplify()), "x");

        // sqrt(0) = 0
        let expr = Expr::num(0).sqrt();
        assert_eq!(format!("{}", expr.simplify()), "0");
    }

    #[test]
    fn test_latex() {
        let expr = Expr::rat(1, 2);
        assert_eq!(expr.to_latex(), "\\frac{1}{2}");

        let expr = Expr::sym("x").pow(Expr::num(2));
        assert_eq!(expr.to_latex(), "x^{2}");
    }

    #[test]
    fn test_substitute() {
        let expr = Expr::sym("x").add(Expr::num(1));
        let result = expr.substitute("x", &Expr::num(5));
        assert_eq!(format!("{}", result.simplify()), "6");
    }
    
    #[test]
    fn test_differentiate_constant() {
        // d/dx(5) = 0
        let expr = Expr::num(5);
        let derivative = expr.differentiate("x");
        assert_eq!(derivative.try_eval(), Some(Rational::zero()));
    }
    
    #[test]
    fn test_differentiate_variable() {
        // d/dx(x) = 1
        let expr = Expr::sym("x");
        let derivative = expr.differentiate("x");
        assert_eq!(derivative.try_eval(), Some(Rational::one()));
        
        // d/dx(y) = 0
        let expr = Expr::sym("y");
        let derivative = expr.differentiate("x");
        assert_eq!(derivative.try_eval(), Some(Rational::zero()));
    }
    
    #[test]
    fn test_differentiate_power() {
        // d/dx(x²) = 2x
        let expr = Expr::sym("x").pow(Expr::num(2));
        let derivative = expr.differentiate("x");
        
        // Evaluate at x=3: should be 6
        let result = derivative.eval_at("x", Rational::new(3, 1));
        assert_eq!(result, Some(Rational::new(6, 1)));
    }
    
    #[test]
    fn test_differentiate_sum() {
        // d/dx(x² + x) = 2x + 1
        let expr = Expr::sym("x").pow(Expr::num(2)).add(Expr::sym("x"));
        let derivative = expr.differentiate("x");
        
        // Evaluate at x=2: should be 5
        let result = derivative.eval_at("x", Rational::new(2, 1));
        assert_eq!(result, Some(Rational::new(5, 1)));
    }
    
    #[test]
    fn test_differentiate_product() {
        // d/dx(x * x) = x + x = 2x
        let expr = Expr::sym("x").mul(Expr::sym("x"));
        let derivative = expr.differentiate("x");
        
        // Evaluate at x=3: should be 6
        let result = derivative.eval_at("x", Rational::new(3, 1));
        assert_eq!(result, Some(Rational::new(6, 1)));
    }
    
    #[test]
    fn test_integrate_constant() {
        // ∫5 dx = 5x
        let expr = Expr::num(5);
        let integral = expr.integrate("x");
        
        // Evaluate at x=3: should be 15
        let result = integral.eval_at("x", Rational::new(3, 1));
        assert_eq!(result, Some(Rational::new(15, 1)));
    }
    
    #[test]
    fn test_integrate_power() {
        // ∫x dx = x²/2
        let expr = Expr::sym("x");
        let integral = expr.integrate("x");
        
        // Evaluate at x=4: should be 8
        let result = integral.eval_at("x", Rational::new(4, 1));
        assert_eq!(result, Some(Rational::new(8, 1)));
    }
    
    #[test]
    fn test_definite_integral() {
        // ∫₁² x dx = [x²/2]₁² = 4/2 - 1/2 = 3/2
        let expr = Expr::sym("x");
        let result = expr.definite_integral("x", Rational::new(1, 1), Rational::new(2, 1));
        assert_eq!(result, Some(Rational::new(3, 2)));
    }
}
