//! Industry exhaustive optimization demo: steel beam selection.
//! Standards (constraints): GB 50017 design formulas — bending M/(gamma_x*W) <= f,
//!   web shear V/(hw*tw) <= fv, deflection 5qL^4/(384EI) <= L/250.
//! Standard data: GB/T 11263 hot-rolled HN sections (representative subset below).
//! All feasibility decisions use EPR exact rational arithmetic (no floats).
//! Market prices are declared assumptions (2026 illustrative): Q235 4.20, Q355 4.45 CNY/kg.

use s_wind::EPR;

fn r(n: i128, d: i128) -> EPR { EPR::from_ratio(n, d) }

struct Section {
    name: &'static str,
    h: i128, b: i128,          // mm
    tw100: i128, tf100: i128,  // mm * 100
    area100: i128,             // cm^2 * 100
    kgm100: i128,              // kg/m * 100
    ix: i128,                  // cm^4
    wx100: i128,               // cm^3 * 100
}

// (name, h, b, tw, tf, area, kg/m, Ix, Wx) — decimals stored x100
const SECTIONS: &[Section] = &[
    Section { name: "HN150x75",  h: 150, b: 75,  tw100: 500,  tf100: 700,  area100: 1816,  kgm100: 1430,  ix: 666,   wx100: 8880 },
    Section { name: "HN175x90",  h: 175, b: 90,  tw100: 500,  tf100: 800,  area100: 2304,  kgm100: 1810,  ix: 1220,  wx100: 14000 },
    Section { name: "HN200x100", h: 200, b: 100, tw100: 550,  tf100: 800,  area100: 2757,  kgm100: 2170,  ix: 1880,  wx100: 18800 },
    Section { name: "HN250x125", h: 250, b: 125, tw100: 600,  tf100: 900,  area100: 3787,  kgm100: 2970,  ix: 4080,  wx100: 32600 },
    Section { name: "HN300x150", h: 300, b: 150, tw100: 650,  tf100: 900,  area100: 4678,  kgm100: 3670,  ix: 7210,  wx100: 48100 },
    Section { name: "HN350x175", h: 350, b: 175, tw100: 700,  tf100: 1100, area100: 6366,  kgm100: 5000,  ix: 13700, wx100: 78200 },
    Section { name: "HN400x200", h: 400, b: 200, tw100: 800,  tf100: 1300, area100: 8412,  kgm100: 6600,  ix: 23700, wx100: 119000 },
    Section { name: "HN450x200", h: 450, b: 200, tw100: 900,  tf100: 1400, area100: 9741,  kgm100: 7650,  ix: 33500, wx100: 149000 },
    Section { name: "HN500x200", h: 500, b: 200, tw100: 1000, tf100: 1600, area100: 11420, kgm100: 8960,  ix: 48100, wx100: 192000 },
    Section { name: "HN600x200", h: 600, b: 200, tw100: 1100, tf100: 1700, area100: 13440, kgm100: 10550, ix: 77600, wx100: 259000 },
];

struct Grade { name: &'static str, price100: i128 } // CNY/kg *100
const GRADES: &[Grade] = &[
    Grade { name: "Q235", price100: 420 },
    Grade { name: "Q355", price100: 445 },
];

// GB 50017 Table 4.2.1 (simplified ranges used here): design strengths by flange thickness
fn strengths(grade: &str, tf100: i128) -> (i128, i128) { // (f, fv) MPa
    match (grade, tf100 <= 1600) {
        ("Q235", true) => (215, 125), ("Q235", false) => (205, 120),
        ("Q355", true) => (305, 175), ("Q355", false) => (295, 170),
        _ => unreachable!(),
    }
}

fn main() {
    // Project data: secondary beam, span 6 m, floor design load 8.0 kN/m^2,
    // standard-combination load 5.5 kN/m^2 (deflection), deflection limit L/250.
    let span = r(6000, 1);                       // mm
    let load_d = r(8, 1);                        // kN/m^2 == N/mm per m width -> handled via spacing_m
    let load_k = r(11, 2);                       // 5.5 kN/m^2
    let e_mod = r(206000, 1);                    // MPa
    let gamma_x = r(21, 20);                     // 1.05
    let g = r(981, 100);                         // 9.81
    let spacings_mm: [i128; 6] = [1500, 2000, 2500, 3000, 3500, 4000];

    // Data self-consistency check: kg/m ~ area * 0.785, Wx ~ 2*Ix/h (within 3%)
    for s in SECTIONS {
        let mass = r(s.area100, 100) * r(785, 1000);
        let dm = (mass.to_f64_lossy() - s.kgm100 as f64 / 100.0).abs() / (s.kgm100 as f64 / 100.0);
        let wx = 2.0 * s.ix as f64 / (s.h as f64 / 10.0); // h converted mm -> cm
        let dw = (wx - s.wx100 as f64 / 100.0).abs() / (s.wx100 as f64 / 100.0);
        println!("data-check {}: mass dev {:.2}%, Wx dev {:.2}%", s.name, dm * 100.0, dw * 100.0);
        assert!(dm < 0.02 && dw < 0.03, "section data inconsistent: {}", s.name);
    }

    let mut total = 0u32;
    let mut feasible: Vec<(EPR, String, EPR, EPR, EPR)> = Vec::new(); // cost, desc, sigma/f, defl/lim, tau/fv
    for s in SECTIONS {
        let self_knm = r(s.kgm100, 100) * g.clone() / r(1000, 1); // kN/m
        let wx = r(s.wx100, 100) * r(1000, 1);                    // mm^3
        let ix = r(s.ix, 1) * r(10000, 1);                        // mm^4
        let hw = r(s.h * 100 - 2 * s.tf100, 100);                 // mm
        let tw = r(s.tw100, 100);
        for gr in GRADES {
            let (f, fv) = strengths(gr.name, s.tf100);
            for &sp in &spacings_mm {
                total += 1;
                let sp_m = r(sp, 1000);
                // design line load (N/mm == kN/m): 1.3*self + area load
                let q = load_d.clone() * sp_m.clone() + r(13, 10) * self_knm.clone();
                let qk = load_k.clone() * sp_m.clone() + self_knm.clone();
                let m = q.clone() * span.clone() * span.clone() / r(8, 1);       // N*mm
                let v = q.clone() * span.clone() / r(2, 1);                      // N
                let sigma = m / (gamma_x.clone() * wx.clone());
                let tau = v / (hw.clone() * tw.clone());
                let defl = r(5, 1) * qk * span.clone() * span.clone() * span.clone() * span.clone()
                    / (r(384, 1) * e_mod.clone() * ix.clone());
                let defl_lim = span.clone() / r(250, 1);
                if sigma <= r(f, 1) && tau <= r(fv, 1) && defl <= defl_lim.clone() {
                    // objective: CNY per m^2 of floor = (kg/m * price) / spacing_m
                    let cost = r(s.kgm100, 100) * r(gr.price100, 100) / sp_m.clone();
                    feasible.push((
                        cost,
                        format!("{} {} spacing={}mm", s.name, gr.name, sp),
                        sigma / r(f, 1), defl / defl_lim, tau / r(fv, 1),
                    ));
                }
            }
        }
    }
    feasible.sort_by(|a, b| a.0.cmp(&b.0));
    println!("\nExhaustive search: {} combinations, {} feasible", total, feasible.len());
    println!("Top 5 cheapest feasible (exact EPR decisions; ratios shown decimal):");
    for (i, (c, d, su, du, tu)) in feasible.iter().take(5).enumerate() {
        println!("  #{} {} cost={:.2} CNY/m2  sigma/f={:.3} defl/lim={:.3} tau/fv={:.3}",
            i + 1, d, c.to_f64_lossy(), su.to_f64_lossy(), du.to_f64_lossy(), tu.to_f64_lossy());
    }
    if let Some((c, d, _, _, _)) = feasible.first() {
        println!("\nOPTIMUM (proven by exhaustion over the declared space): {} @ {:.4} CNY/m2, exact cost = {:?}",
            d, c.to_f64_lossy(), c);
    }
}
