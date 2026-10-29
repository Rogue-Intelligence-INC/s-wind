//! Statistics Module for Blondle
//!
//! Covers:
//! - Probability distributions
//! - Statistical tests
//! - Estimation theory
//! - Information-theoretic statistics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all statistics knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Statistics] Loaded {} constants, {} laws, {} theorems",
             kb.physical_constants.len() - before.0,
             kb.physical_laws.len() - before.1,
             kb.theorems.len() - before.2);
}

pub fn count() -> (usize, usize, usize) {
    // Measured, not declared: count what this module actually inserts
    // into a standard (fundamentals-initialized) knowledge base.
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(&mut kb);
    add_laws(&mut kb);
    add_theorems(&mut kb);
    (
        kb.physical_constants.len() - before.0,
        kb.physical_laws.len() - before.1,
        kb.theorems.len() - before.2,
    )
}

const CONSTANT_COUNT: usize = 8;
const LAW_COUNT: usize = 12;
const THEOREM_COUNT: usize = 20;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Critical values
        PhysicalConstant {
            id: "z_95".into(),
            name: "95% confidence z-score".into(),
            symbol: "z_{0.975}".into(),
            value: "1.96".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Normal distribution".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"z_{0.975} = 1.96 \text{ (two-tailed 95% CI)}".into(),
        },
        PhysicalConstant {
            id: "z_99".into(),
            name: "99% confidence z-score".into(),
            symbol: "z_{0.995}".into(),
            value: "2.576".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Normal distribution".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"z_{0.995} = 2.576 \text{ (two-tailed 99% CI)}".into(),
        },
        PhysicalConstant {
            id: "z_997".into(),
            name: "99.7% confidence z-score (3σ)".into(),
            symbol: "z_{3\\sigma}".into(),
            value: "3.0".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Normal distribution".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"3\sigma \text{ rule: } 99.73\% \text{ within } \pm 3\sigma".into(),
        },
        
        // Distribution parameters
        PhysicalConstant {
            id: "skewness_normal".into(),
            name: "Skewness of normal distribution".into(),
            symbol: r"\gamma_1".into(),
            value: "0".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Normal distribution".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\gamma_1 = 0 \text{ (symmetric)}".into(),
        },
        PhysicalConstant {
            id: "kurtosis_normal".into(),
            name: "Excess kurtosis of normal distribution".into(),
            symbol: r"\gamma_2".into(),
            value: "0".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Normal distribution".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\gamma_2 = 0 \text{ (mesokurtic)}".into(),
        },
        
        // Sample size rules
        PhysicalConstant {
            id: "clt_threshold".into(),
            name: "CLT rule of thumb".into(),
            symbol: "n_{CLT}".into(),
            value: "30".into(),
            unit: "samples".into(),
            uncertainty: None,
            source: "Statistics practice".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"n \geq 30 \text{ for CLT to apply approximately}".into(),
        },
        PhysicalConstant {
            id: "cohen_d_medium".into(),
            name: "Cohen's d (medium effect)".into(),
            symbol: "d".into(),
            value: "0.5".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Cohen 1988".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d = 0.2 \text{ (small)}, 0.5 \text{ (medium)}, 0.8 \text{ (large)}".into(),
        },
        PhysicalConstant {
            id: "bonferroni_alpha".into(),
            name: "Bonferroni correction".into(),
            symbol: r"\alpha_{adj}".into(),
            value: "α/m".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Multiple testing".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\alpha_{adj} = \frac{\alpha}{m} \text{ for } m \text{ tests}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Distributions
        PhysicalLaw {
            id: "normal_pdf".into(),
            name: "Normal Distribution PDF".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"f(x) = \frac{1}{\sigma\sqrt{2\pi}} e^{-\frac{(x-\mu)^2}{2\sigma^2}}".into(),
            description: "Gaussian probability density function".into(),
            variables: vec![
                Variable { symbol: "μ".into(), name: "Mean".into(), unit: None },
                Variable { symbol: "σ".into(), name: "Standard deviation".into(), unit: None },
            ],
            source: "Gauss".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "binomial_pmf".into(),
            name: "Binomial Distribution PMF".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"P(X=k) = \binom{n}{k} p^k (1-p)^{n-k}".into(),
            description: "Probability of k successes in n trials".into(),
            variables: vec![
                Variable { symbol: "n".into(), name: "Number of trials".into(), unit: None },
                Variable { symbol: "p".into(), name: "Success probability".into(), unit: None },
            ],
            source: "Bernoulli".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "poisson_pmf".into(),
            name: "Poisson Distribution PMF".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"P(X=k) = \frac{\lambda^k e^{-\lambda}}{k!}".into(),
            description: "Probability of k events with rate λ".into(),
            variables: vec![
                Variable { symbol: "λ".into(), name: "Rate parameter".into(), unit: None },
            ],
            source: "Poisson 1837".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "exponential_pdf".into(),
            name: "Exponential Distribution PDF".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"f(x) = \lambda e^{-\lambda x}, \quad x \geq 0".into(),
            description: "Time between Poisson events".into(),
            variables: vec![
                Variable { symbol: "λ".into(), name: "Rate parameter".into(), unit: None },
            ],
            source: "Probability theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "chi_square_pdf".into(),
            name: "Chi-Square Distribution".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"f(x) = \frac{x^{k/2-1}e^{-x/2}}{2^{k/2}\Gamma(k/2)}".into(),
            description: "Sum of squared standard normals".into(),
            variables: vec![
                Variable { symbol: "k".into(), name: "Degrees of freedom".into(), unit: None },
            ],
            source: "Probability theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "student_t_pdf".into(),
            name: "Student's t-Distribution".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"f(t) = \frac{\Gamma((\\nu+1)/2)}{\sqrt{\\nu\pi}\,\Gamma(\\nu/2)}\left(1+\frac{t^2}{\\nu}\right)^{-(\\nu+1)/2}".into(),
            description: "Ratio of normal to chi-square".into(),
            variables: vec![
                Variable { symbol: "ν".into(), name: "Degrees of freedom".into(), unit: None },
            ],
            source: "Student (Gosset) 1908".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Estimation
        PhysicalLaw {
            id: "sample_variance".into(),
            name: "Unbiased Sample Variance".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"s^2 = \frac{1}{n-1}\sum_{i=1}^{n}(x_i - \bar{x})^2".into(),
            description: "Unbiased estimator of population variance".into(),
            variables: vec![
                Variable { symbol: "s²".into(), name: "Sample variance".into(), unit: None },
                Variable { symbol: "x̄".into(), name: "Sample mean".into(), unit: None },
            ],
            source: "Statistics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "standard_error".into(),
            name: "Standard Error of Mean".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"SE = \frac{\sigma}{\sqrt{n}}".into(),
            description: "Standard deviation of sample mean".into(),
            variables: vec![
                Variable { symbol: "SE".into(), name: "Standard error".into(), unit: None },
                Variable { symbol: "n".into(), name: "Sample size".into(), unit: None },
            ],
            source: "Statistics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "confidence_interval".into(),
            name: "Confidence Interval for Mean".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"\bar{x} \pm z_{\alpha/2} \frac{\sigma}{\sqrt{n}}".into(),
            description: "Interval estimate for population mean".into(),
            variables: vec![
                Variable { symbol: "z".into(), name: "Critical value".into(), unit: None },
            ],
            source: "Statistics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Tests
        PhysicalLaw {
            id: "z_test_statistic".into(),
            name: "Z-Test Statistic".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"z = \frac{\bar{x} - \mu_0}{\sigma/\sqrt{n}}".into(),
            description: "Test statistic for known variance".into(),
            variables: vec![
                Variable { symbol: "μ₀".into(), name: "Hypothesized mean".into(), unit: None },
            ],
            source: "Statistics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "t_test_statistic".into(),
            name: "T-Test Statistic".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"t = \frac{\bar{x} - \mu_0}{s/\sqrt{n}}".into(),
            description: "Test statistic for unknown variance".into(),
            variables: vec![
                Variable { symbol: "s".into(), name: "Sample standard deviation".into(), unit: None },
            ],
            source: "Student 1908".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "chi_square_test".into(),
            name: "Chi-Square Test Statistic".into(),
            domain: PhysicsDomain::Other("Statistics".into()),
            equation_tex: r"\chi^2 = \sum_i \frac{(O_i - E_i)^2}{E_i}".into(),
            description: "Goodness of fit test".into(),
            variables: vec![
                Variable { symbol: "O_i".into(), name: "Observed frequency".into(), unit: None },
                Variable { symbol: "E_i".into(), name: "Expected frequency".into(), unit: None },
            ],
            source: "Pearson 1900".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        // Limit theorems
        MathTheorem {
            id: "clt".into(),
            name: "Central Limit Theorem".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"\frac{\bar{X}_n - \mu}{\sigma/\sqrt{n}} \xrightarrow{d} N(0,1)".into(),
            proof_tex: Some("Normalized sum converges to standard normal".into()),
            references: vec![],
            source: "Laplace, Lindeberg, Lévy".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "wlln".into(),
            name: "Weak Law of Large Numbers".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"\bar{X}_n \xrightarrow{P} \mu".into(),
            proof_tex: Some("Sample mean converges in probability to population mean".into()),
            references: vec![],
            source: "Bernoulli 1713".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "slln".into(),
            name: "Strong Law of Large Numbers".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"\bar{X}_n \xrightarrow{a.s.} \mu".into(),
            proof_tex: Some("Sample mean converges almost surely".into()),
            references: vec!["wlln".into()],
            source: "Kolmogorov 1930".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Bayesian
        MathTheorem {
            id: "bayes_theorem".into(),
            name: "Bayes' Theorem".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"P(A|B) = \frac{P(B|A)P(A)}{P(B)}".into(),
            proof_tex: Some("From conditional probability definition".into()),
            references: vec![],
            source: "Bayes 1763".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "law_total_probability".into(),
            name: "Law of Total Probability".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"P(B) = \sum_i P(B|A_i)P(A_i)".into(),
            proof_tex: Some("For partition {A_i} of sample space".into()),
            references: vec![],
            source: "Probability theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Inequalities
        MathTheorem {
            id: "markov_inequality".into(),
            name: "Markov's Inequality".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"P(X \geq a) \leq \frac{E[X]}{a}".into(),
            proof_tex: Some("For non-negative X and a > 0".into()),
            references: vec![],
            source: "Markov".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "chebyshev_inequality".into(),
            name: "Chebyshev's Inequality".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"P(|X-\mu| \geq k\sigma) \leq \frac{1}{k^2}".into(),
            proof_tex: Some("Bound using variance".into()),
            references: vec!["markov_inequality".into()],
            source: "Chebyshev 1867".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "hoeffding_inequality".into(),
            name: "Hoeffding's Inequality".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"P(|\bar{X}_n - \mu| \geq t) \leq 2e^{-2nt^2/(b-a)^2}".into(),
            proof_tex: Some("For bounded random variables in [a,b]".into()),
            references: vec![],
            source: "Hoeffding 1963".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "jensen_inequality".into(),
            name: "Jensen's Inequality".into(),
            category: TheoremCategory::Probability,
            statement_tex: r"\varphi(E[X]) \leq E[\varphi(X)] \text{ for convex } \varphi".into(),
            proof_tex: Some("With equality for linear functions".into()),
            references: vec![],
            source: "Jensen 1906".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Estimation theory
        MathTheorem {
            id: "cramer_rao".into(),
            name: "Cramér-Rao Lower Bound".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"Var(\hat{\theta}) \geq \frac{1}{I(\theta)}".into(),
            proof_tex: Some("Fisher information bounds estimator variance".into()),
            references: vec![],
            source: "Cramér 1946, Rao 1945".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "rao_blackwell".into(),
            name: "Rao-Blackwell Theorem".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"E[\hat{\theta}|T] \text{ has lower MSE than } \hat{\theta}".into(),
            proof_tex: Some("Conditioning on sufficient statistic improves estimator".into()),
            references: vec![],
            source: "Rao 1945, Blackwell 1947".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "lehmann_scheffe".into(),
            name: "Lehmann-Scheffé Theorem".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\text{Unbiased + function of complete sufficient stat} \Rightarrow \text{UMVUE}".into(),
            proof_tex: Some("Uniqueness of minimum variance unbiased estimator".into()),
            references: vec!["rao_blackwell".into()],
            source: "Lehmann, Scheffé".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mle_consistency".into(),
            name: "MLE Consistency".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\hat{\theta}_{MLE} \xrightarrow{P} \theta_0 \text{ (under regularity)}".into(),
            proof_tex: Some("Maximum likelihood estimator is consistent".into()),
            references: vec![],
            source: "Fisher".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mle_asymptotic_normality".into(),
            name: "MLE Asymptotic Normality".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\sqrt{n}(\hat{\theta}_{MLE} - \theta_0) \xrightarrow{d} N(0, I(\theta_0)^{-1})".into(),
            proof_tex: Some("MLE achieves Cramér-Rao bound asymptotically".into()),
            references: vec!["cramer_rao".into()],
            source: "Fisher".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Hypothesis testing
        MathTheorem {
            id: "neyman_pearson".into(),
            name: "Neyman-Pearson Lemma".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\text{Likelihood ratio test is most powerful for simple hypotheses}".into(),
            proof_tex: Some("Optimal test rejects when L(x) > k".into()),
            references: vec![],
            source: "Neyman, Pearson 1933".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "wilks_theorem".into(),
            name: "Wilks' Theorem".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"-2\ln\Lambda \xrightarrow{d} \chi^2_k".into(),
            proof_tex: Some("Likelihood ratio test statistic is asymptotically chi-squared".into()),
            references: vec![],
            source: "Wilks 1938".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Regression
        MathTheorem {
            id: "gauss_markov".into(),
            name: "Gauss-Markov Theorem".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\text{OLS is BLUE under Gauss-Markov assumptions}".into(),
            proof_tex: Some("Best Linear Unbiased Estimator when errors uncorrelated, homoscedastic".into()),
            references: vec![],
            source: "Gauss, Markov".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "cochran_theorem".into(),
            name: "Cochran's Theorem".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"\sum_i Q_i = Q \text{ with } Q_i \sim \chi^2_{d_i} \text{ independent if } \sum d_i = n".into(),
            proof_tex: Some("Partitioning chi-squared into independent components".into()),
            references: vec![],
            source: "Cochran 1934".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "aic".into(),
            name: "Akaike Information Criterion".into(),
            category: TheoremCategory::Other("Statistics".into()),
            statement_tex: r"AIC = 2k - 2\ln(\hat{L})".into(),
            proof_tex: Some("Model selection balancing fit and complexity".into()),
            references: vec![],
            source: "Akaike 1974".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
