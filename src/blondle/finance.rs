//! Finance Module for Blondle
//!
//! Covers:
//! - Options pricing (Black-Scholes, Greeks)
//! - Portfolio theory (Markowitz, CAPM)
//! - Risk management (VaR, Expected Shortfall)
//! - Fixed income (bond pricing, duration)
//! - Derivatives and hedging

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all finance knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Finance] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 12;
const LAW_COUNT: usize = 15;
const THEOREM_COUNT: usize = 12;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Market parameters (typical values)
        PhysicalConstant {
            id: "risk_free_rate_typical".into(),
            name: "Typical risk-free rate".into(),
            symbol: "r_f".into(),
            value: "0.03".into(),
            unit: "per year".into(),
            uncertainty: Some("0.02".into()),
            source: "Treasury rates".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"r_f \approx 1\%-5\% \text{ (depends on economic conditions)}".into(),
        },
        PhysicalConstant {
            id: "market_risk_premium".into(),
            name: "Historical equity risk premium".into(),
            symbol: r"\text{ERP}".into(),
            value: "0.05".into(),
            unit: "per year".into(),
            uncertainty: Some("0.02".into()),
            source: "Historical data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"ERP \approx 4\%-7\% \text{ (historical average)}".into(),
        },
        PhysicalConstant {
            id: "sp500_volatility".into(),
            name: "S&P 500 historical volatility".into(),
            symbol: r"\sigma_{SPX}".into(),
            value: "0.16".into(),
            unit: "per year".into(),
            uncertainty: Some("0.05".into()),
            source: "Historical data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\sigma \approx 15\%-20\% \text{ (long-term)}".into(),
        },
        PhysicalConstant {
            id: "vix_average".into(),
            name: "VIX long-term average".into(),
            symbol: "VIX_{avg}".into(),
            value: "19".into(),
            unit: "%".into(),
            uncertainty: Some("5".into()),
            source: "CBOE".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"VIX_{avg} \approx 19 \text{ (1990-2020)}".into(),
        },
        
        // Trading parameters
        PhysicalConstant {
            id: "trading_days".into(),
            name: "Trading days per year".into(),
            symbol: "N_{trading}".into(),
            value: "252".into(),
            unit: "days".into(),
            uncertainty: None,
            source: "Market convention".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"N_{trading} = 252 \text{ days/year}".into(),
        },
        PhysicalConstant {
            id: "day_count_30_360".into(),
            name: "30/360 day count convention".into(),
            symbol: "DC_{30/360}".into(),
            value: "360".into(),
            unit: "days".into(),
            uncertainty: None,
            source: "ISDA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{30/360: Each month = 30 days, year = 360}".into(),
        },
        PhysicalConstant {
            id: "day_count_act_365".into(),
            name: "ACT/365 day count convention".into(),
            symbol: "DC_{ACT/365}".into(),
            value: "365".into(),
            unit: "days".into(),
            uncertainty: None,
            source: "ISDA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{ACT/365: Actual days, year = 365}".into(),
        },
        
        // Risk limits
        PhysicalConstant {
            id: "var_confidence".into(),
            name: "Standard VaR confidence level".into(),
            symbol: r"\alpha_{VaR}".into(),
            value: "0.99".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Basel III".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{VaR typically at 99\% confidence}".into(),
        },
        PhysicalConstant {
            id: "var_z_99".into(),
            name: "Z-score for 99% VaR".into(),
            symbol: "z_{0.99}".into(),
            value: "2.326".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Standard normal".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"z_{0.99} = 2.326".into(),
        },
        PhysicalConstant {
            id: "var_z_95".into(),
            name: "Z-score for 95% VaR".into(),
            symbol: "z_{0.95}".into(),
            value: "1.645".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Standard normal".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"z_{0.95} = 1.645".into(),
        },
        
        // Black-Scholes parameters
        PhysicalConstant {
            id: "bs_normal_cdf".into(),
            name: "Standard normal CDF".into(),
            symbol: "N(x)".into(),
            value: "CDF".into(),
            unit: "function".into(),
            uncertainty: None,
            source: "Statistics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"N(x) = \frac{1}{\sqrt{2\pi}}\int_{-\infty}^x e^{-t^2/2}dt".into(),
        },
        PhysicalConstant {
            id: "sqrt_252".into(),
            name: "Annualization factor for daily volatility".into(),
            symbol: r"\sqrt{252}".into(),
            value: "15.87".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Mathematics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\sigma_{annual} = \sigma_{daily} \times \sqrt{252}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Black-Scholes
        PhysicalLaw {
            id: "black_scholes_call".into(),
            name: "Black-Scholes Call Price".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"C = SN(d_1) - Ke^{-rT}N(d_2)".into(),
            description: "European call option price".into(),
            variables: vec![
                Variable { symbol: "C".into(), name: "Call price".into(), unit: Some("currency".into()) },
                Variable { symbol: "S".into(), name: "Spot price".into(), unit: Some("currency".into()) },
                Variable { symbol: "K".into(), name: "Strike price".into(), unit: Some("currency".into()) },
                Variable { symbol: "T".into(), name: "Time to expiry".into(), unit: Some("years".into()) },
            ],
            source: "Black-Scholes 1973".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "black_scholes_put".into(),
            name: "Black-Scholes Put Price".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"P = Ke^{-rT}N(-d_2) - SN(-d_1)".into(),
            description: "European put option price".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Put price".into(), unit: Some("currency".into()) },
            ],
            source: "Black-Scholes 1973".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "black_scholes_d1d2".into(),
            name: "Black-Scholes d1 and d2".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"d_1 = \frac{\ln(S/K) + (r + \sigma^2/2)T}{\sigma\sqrt{T}}, \quad d_2 = d_1 - \sigma\sqrt{T}".into(),
            description: "Black-Scholes parameters".into(),
            variables: vec![
                Variable { symbol: "σ".into(), name: "Volatility".into(), unit: Some("per √year".into()) },
                Variable { symbol: "r".into(), name: "Risk-free rate".into(), unit: Some("per year".into()) },
            ],
            source: "Black-Scholes 1973".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Greeks
        PhysicalLaw {
            id: "delta_call".into(),
            name: "Delta (Call Option)".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"\Delta_{call} = N(d_1)".into(),
            description: "Rate of change of option price with spot".into(),
            variables: vec![
                Variable { symbol: "Δ".into(), name: "Delta".into(), unit: None },
            ],
            source: "Options theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "gamma".into(),
            name: "Gamma".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"\Gamma = \frac{N'(d_1)}{S\sigma\sqrt{T}}".into(),
            description: "Rate of change of delta with spot".into(),
            variables: vec![
                Variable { symbol: "Γ".into(), name: "Gamma".into(), unit: Some("currency⁻¹".into()) },
            ],
            source: "Options theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "vega".into(),
            name: "Vega".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"\nu = S\sqrt{T}N'(d_1)".into(),
            description: "Rate of change of option price with volatility".into(),
            variables: vec![
                Variable { symbol: "ν".into(), name: "Vega".into(), unit: Some("currency".into()) },
            ],
            source: "Options theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "theta_call".into(),
            name: "Theta (Call Option)".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"\Theta = -\frac{S\sigma N'(d_1)}{2\sqrt{T}} - rKe^{-rT}N(d_2)".into(),
            description: "Rate of change of option price with time".into(),
            variables: vec![
                Variable { symbol: "Θ".into(), name: "Theta".into(), unit: Some("currency/time".into()) },
            ],
            source: "Options theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "rho_call".into(),
            name: "Rho (Call Option)".into(),
            domain: PhysicsDomain::Other("Mathematical Finance".into()),
            equation_tex: r"\rho = KTe^{-rT}N(d_2)".into(),
            description: "Rate of change of option price with interest rate".into(),
            variables: vec![
                Variable { symbol: "ρ".into(), name: "Rho".into(), unit: Some("currency".into()) },
            ],
            source: "Options theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Portfolio theory
        PhysicalLaw {
            id: "capm".into(),
            name: "Capital Asset Pricing Model".into(),
            domain: PhysicsDomain::Other("Portfolio Theory".into()),
            equation_tex: r"E[R_i] = r_f + \beta_i(E[R_m] - r_f)".into(),
            description: "Expected return in equilibrium".into(),
            variables: vec![
                Variable { symbol: "E[R_i]".into(), name: "Expected return".into(), unit: Some("per year".into()) },
                Variable { symbol: "β_i".into(), name: "Market beta".into(), unit: None },
                Variable { symbol: "E[R_m]".into(), name: "Expected market return".into(), unit: Some("per year".into()) },
            ],
            source: "Sharpe 1964, Lintner 1965".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "portfolio_variance".into(),
            name: "Portfolio Variance".into(),
            domain: PhysicsDomain::Other("Portfolio Theory".into()),
            equation_tex: r"\sigma_p^2 = \sum_i\sum_j w_i w_j \sigma_i \sigma_j \rho_{ij}".into(),
            description: "Variance of portfolio returns".into(),
            variables: vec![
                Variable { symbol: "w_i".into(), name: "Asset weight".into(), unit: None },
                Variable { symbol: "ρ_ij".into(), name: "Correlation".into(), unit: None },
            ],
            source: "Markowitz 1952".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "sharpe_ratio".into(),
            name: "Sharpe Ratio".into(),
            domain: PhysicsDomain::Other("Portfolio Theory".into()),
            equation_tex: r"SR = \frac{E[R_p] - r_f}{\sigma_p}".into(),
            description: "Risk-adjusted return".into(),
            variables: vec![
                Variable { symbol: "SR".into(), name: "Sharpe ratio".into(), unit: None },
            ],
            source: "Sharpe 1966".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Fixed income
        PhysicalLaw {
            id: "bond_price".into(),
            name: "Bond Price".into(),
            domain: PhysicsDomain::Other("Fixed Income".into()),
            equation_tex: r"P = \sum_{t=1}^{n}\frac{C}{(1+y)^t} + \frac{F}{(1+y)^n}".into(),
            description: "Present value of bond cash flows".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Bond price".into(), unit: Some("currency".into()) },
                Variable { symbol: "C".into(), name: "Coupon payment".into(), unit: Some("currency".into()) },
                Variable { symbol: "y".into(), name: "Yield to maturity".into(), unit: Some("per period".into()) },
                Variable { symbol: "F".into(), name: "Face value".into(), unit: Some("currency".into()) },
            ],
            source: "Fixed income theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "macaulay_duration".into(),
            name: "Macaulay Duration".into(),
            domain: PhysicsDomain::Other("Fixed Income".into()),
            equation_tex: r"D = \frac{\sum_{t=1}^{n}t \cdot PV(CF_t)}{P}".into(),
            description: "Weighted average time to cash flows".into(),
            variables: vec![
                Variable { symbol: "D".into(), name: "Duration".into(), unit: Some("years".into()) },
            ],
            source: "Macaulay 1938".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "modified_duration".into(),
            name: "Modified Duration".into(),
            domain: PhysicsDomain::Other("Fixed Income".into()),
            equation_tex: r"D_{mod} = \frac{D}{1+y}".into(),
            description: "Price sensitivity to yield change".into(),
            variables: vec![
                Variable { symbol: "D_mod".into(), name: "Modified duration".into(), unit: Some("years".into()) },
            ],
            source: "Fixed income theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Risk management
        PhysicalLaw {
            id: "var_parametric".into(),
            name: "Parametric VaR".into(),
            domain: PhysicsDomain::Other("Risk Management".into()),
            equation_tex: r"VaR_\alpha = \mu + z_\alpha \sigma".into(),
            description: "Value at Risk under normal distribution".into(),
            variables: vec![
                Variable { symbol: "VaR".into(), name: "Value at Risk".into(), unit: Some("currency".into()) },
                Variable { symbol: "z_α".into(), name: "Normal quantile".into(), unit: None },
            ],
            source: "Risk management".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "put_call_parity".into(),
            name: "Put-Call Parity".into(),
            category: TheoremCategory::Other("Mathematical Finance".into()),
            statement_tex: r"C - P = S - Ke^{-rT}".into(),
            proof_tex: Some("Arbitrage argument with replicating portfolios".into()),
            references: vec![],
            source: "Stoll 1969".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "black_scholes_pde".into(),
            name: "Black-Scholes PDE".into(),
            category: TheoremCategory::Other("Mathematical Finance".into()),
            statement_tex: r"\frac{\partial V}{\partial t} + \frac{1}{2}\sigma^2 S^2\frac{\partial^2 V}{\partial S^2} + rS\frac{\partial V}{\partial S} - rV = 0".into(),
            proof_tex: Some("Derived from Ito's lemma and no-arbitrage condition".into()),
            references: vec![],
            source: "Black-Scholes 1973".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "risk_neutral_pricing".into(),
            name: "Risk-Neutral Pricing".into(),
            category: TheoremCategory::Other("Mathematical Finance".into()),
            statement_tex: r"V_0 = e^{-rT}E^Q[V_T]".into(),
            proof_tex: Some("Under risk-neutral measure, all assets drift at risk-free rate".into()),
            references: vec![],
            source: "Harrison-Kreps 1979".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "ito_lemma".into(),
            name: "Ito's Lemma".into(),
            category: TheoremCategory::Other("Stochastic Calculus".into()),
            statement_tex: r"df = \frac{\partial f}{\partial t}dt + \frac{\partial f}{\partial S}dS + \frac{1}{2}\frac{\partial^2 f}{\partial S^2}(dS)^2".into(),
            proof_tex: Some("Chain rule for stochastic processes".into()),
            references: vec![],
            source: "Ito 1951".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "efficient_frontier".into(),
            name: "Efficient Frontier".into(),
            category: TheoremCategory::Other("Portfolio Theory".into()),
            statement_tex: r"\text{Pareto-optimal portfolios: max return for given risk}".into(),
            proof_tex: Some("Quadratic optimization with linear constraints".into()),
            references: vec![],
            source: "Markowitz 1952".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "two_fund_separation".into(),
            name: "Two-Fund Separation Theorem".into(),
            category: TheoremCategory::Other("Portfolio Theory".into()),
            statement_tex: r"\text{All efficient portfolios = combinations of any two efficient portfolios}".into(),
            proof_tex: Some("With risk-free asset: tangency portfolio + risk-free".into()),
            references: vec![],
            source: "Tobin 1958".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "modigliani_miller".into(),
            name: "Modigliani-Miller Theorem".into(),
            category: TheoremCategory::Other("Corporate Finance".into()),
            statement_tex: r"V_L = V_U \text{ (in perfect markets, capital structure irrelevant)}".into(),
            proof_tex: Some("Arbitrage argument between levered and unlevered firms".into()),
            references: vec![],
            source: "Modigliani-Miller 1958".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "var_subadditivity".into(),
            name: "VaR Non-Subadditivity".into(),
            category: TheoremCategory::Other("Risk Management".into()),
            statement_tex: r"VaR(A+B) \not\leq VaR(A) + VaR(B) \text{ in general}".into(),
            proof_tex: Some("Diversification may increase VaR in some cases".into()),
            references: vec![],
            source: "Artzner et al. 1999".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "expected_shortfall".into(),
            name: "Expected Shortfall (CVaR)".into(),
            category: TheoremCategory::Other("Risk Management".into()),
            statement_tex: r"ES_\alpha = E[X | X > VaR_\alpha]".into(),
            proof_tex: Some("Coherent risk measure; average loss beyond VaR".into()),
            references: vec![],
            source: "Acerbi-Tasche 2002".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "duration_approximation".into(),
            name: "Duration Price Approximation".into(),
            category: TheoremCategory::Other("Fixed Income".into()),
            statement_tex: r"\frac{\Delta P}{P} \approx -D_{mod} \cdot \Delta y".into(),
            proof_tex: Some("First-order Taylor expansion of bond price".into()),
            references: vec![],
            source: "Fixed income theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "convexity_adjustment".into(),
            name: "Convexity Adjustment".into(),
            category: TheoremCategory::Other("Fixed Income".into()),
            statement_tex: r"\frac{\Delta P}{P} \approx -D_{mod} \cdot \Delta y + \frac{1}{2}C(\Delta y)^2".into(),
            proof_tex: Some("Second-order Taylor expansion including convexity".into()),
            references: vec![],
            source: "Fixed income theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "gbm".into(),
            name: "Geometric Brownian Motion".into(),
            category: TheoremCategory::Other("Stochastic Processes".into()),
            statement_tex: r"dS = \mu S dt + \sigma S dW".into(),
            proof_tex: Some("Solution: S(t) = S(0)exp((μ-σ²/2)t + σW(t))".into()),
            references: vec![],
            source: "Mathematical finance".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
