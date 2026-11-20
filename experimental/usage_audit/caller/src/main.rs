//! Calls public library entry points. Does not implement a search.
use s_wind::symbolic::AlgebraicBase;
use s_wind::{
    AlgebraicSearchEngine, EPR, Fate, GravitationalField, GravitationalFieldConfig,
    KnowledgeBase, PhysicalConstant, Rational,
};

fn main() {
    fate_library_path();
    gravity_variant();
    algebraic_search_variant();
}

fn fate_library_path() {
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let signal = vec![
        EPR::from_ratio(1, 2),
        EPR::from_i128(1),
        EPR::from_ratio(3, 2),
    ];
    let mut fate = Fate::new();
    let fast = fate.explore(&signal, &kb);
    println!("LIB explore status={:?} formula={:?}", fast.status, fast.formula);
    let verified = fate.explore_with_verification(&signal, &kb);
    println!(
        "LIB verified fit={:?} discovery={} formula={:?}",
        verified.fit_level, verified.is_discovery, verified.formula
    );
    let (looped, _hist, stats) = fate.explore_with_self_loop(&signal, &kb);
    println!(
        "LIB loop formula={:?} closure_reached={} truly_exhausted={} iterations={}",
        looped.formula, stats.closure_reached, stats.truly_exhausted, stats.iterations_run
    );
}

fn gravity_variant() {
    let field = GravitationalField::new(GravitationalFieldConfig::default());
    for (label, r) in [("unit_test_r", 2i128), ("variant_r", 4)] {
        let force = field.gravitational_force(Rational::new(r, 1)).unwrap();
        println!(
            "LIB gravity {label} r={r} F={}/{}",
            force.numerator, force.denominator
        );
    }
}

fn algebraic_search_variant() {
    let pi = std::f64::consts::PI;
    let sqrt2 = std::f64::consts::SQRT_2;
    let cases = [
        ("one_plus_two_pi", 1.0 + 2.0 * pi, AlgebraicBase::Pi, -3, 3),
        ("five_plus_sqrt2", 5.0 + sqrt2, AlgebraicBase::Sqrt(2), -5, 5),
    ];
    for (name, value, base, lo, hi) in cases {
        let target = PhysicalConstant {
            name,
            symbol: name,
            value,
            latex: name,
        };
        let engine = AlgebraicSearchEngine::new()
            .with_bases(vec![base])
            .with_coeff_range(lo..=hi)
            .with_tolerance(0.001);
        let hits = engine.search_linear(&target);
        println!("LIB search_linear {name} hits={}", hits.len());
        for (i, hit) in hits.iter().take(5).enumerate() {
            println!(
                "  #{i} expr={} error_percent={:.12} value={:.16}",
                hit.expression,
                hit.error_percent,
                hit.expression.to_f64()
            );
        }
    }
}
