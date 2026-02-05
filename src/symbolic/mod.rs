//! Symbolic Computation Module
//!
//! S-wind 符号计算系统:
//!
//! - Rational (i128/i128): 快速精确，适合大部分计算
//! - EPR (ExtendedInt/ExtendedInt): 无限精度，适合阶乘、高精度常数
//! - Expr: 符号表达式，支持微分、积分、简化
//! - Algebraic: 代数优先的表达式系统，支持无理数
//!
//! 核心原则: 代数永远优先于数字
//! - 所有计算尽可能保持符号形式
//! - 只在最终验证时才转换为数值近似
//! - 零舍入误差 (除非显式请求近似)

mod rational;
mod expr;
mod epr;
mod algebraic;

pub use rational::Rational;
pub use expr::{Expr, Symbol, PhysicsConst};
pub use epr::{EPR, ExtendedInt};
pub use algebraic::{
    AlgebraicBase, AlgebraicTerm, AlgebraicExpr, 
    AlgebraicBuilder, AlgebraicDiscovery
};