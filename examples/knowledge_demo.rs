//! Knowledge Base Demo
//! 
//! Shows the persistent knowledge base with physics/math constants, theorems, and laws.
//! 
//! Run: cargo run --example knowledge_demo

use s_wind::knowledge::{KnowledgeBase, storage, KnowledgeStatus};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║       S-wind Knowledge Base Demo                                  ║");
    println!("║       Physics + Mathematics + Discoveries                         ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Load or create knowledge base
    let kb_path = "knowledge_base.json";
    let mut kb = storage::load_or_create(kb_path);
    
    println!("\n{}\n", kb.stats());
    
    // Show some physical constants
    println!("=== Sample Physical Constants (NIST CODATA) ===\n");
    for c in kb.physical_constants.iter().take(5) {
        println!("  {} ({}):", c.name, c.symbol);
        println!("    Value: {} {}", c.value, c.unit);
        println!("    TeX: {}\n", c.tex_definition);
    }
    
    // Show some math constants
    println!("=== Sample Mathematical Constants ===\n");
    for c in kb.math_constants.iter().take(5) {
        println!("  {} ({}):", c.name, c.symbol);
        println!("    Value: {:.10}", c.value_approx);
        if let Some(ref exact) = c.exact_form {
            println!("    Exact: {}", exact);
        }
        println!();
    }
    
    // Show some theorems
    println!("=== Sample Theorems ===\n");
    for t in kb.theorems.iter().take(3) {
        println!("  {}:", t.name);
        println!("    ${} $", t.statement_tex);
        println!("    Source: {}\n", t.source);
    }
    
    // Show some physical laws
    println!("=== Sample Physical Laws ===\n");
    for l in kb.physical_laws.iter().take(3) {
        println!("  {} ({:?}):", l.name, l.domain);
        println!("    ${} $", l.equation_tex);
        println!("    {}\n", l.description);
    }
    
    // Add a discovery (marked as PendingReview)
    println!("=== Adding Discovery (PendingReview) ===\n");
    let id = kb.add_discovery(
        r"f(n) = \frac{n(n+1)}{2}".into(),
        "Triangular Number Formula - sum of first n integers".into(),
        r"\begin{proof}
By induction: $\sum_{k=1}^n k = \frac{n(n+1)}{2}$
Base case: $n=1$: $1 = \frac{1 \cdot 2}{2} = 1$ ✓
Inductive step: Assume true for $n$, then for $n+1$:
$\sum_{k=1}^{n+1} k = \frac{n(n+1)}{2} + (n+1) = \frac{(n+1)(n+2)}{2}$ ✓
\qed
\end{proof}".into(),
    );
    println!("  Created: {} (Status: PendingReview)", id);
    
    // Show pending items
    println!("\n=== Pending Review ===\n");
    for (id, eq) in kb.get_pending_review().iter().map(|d| (&d.id, &d.equation_tex)) {
        println!("  [PENDING] {}: ${} $", id, eq);
    }
    
    // Save
    println!("\n=== Saving Knowledge Base ===\n");
    storage::save_to_json(&kb, kb_path).unwrap();
    println!("  Saved to: {}", kb_path);
    
    // Export to TeX
    let tex_path = "knowledge_base.tex";
    storage::export_to_tex_file(&kb, tex_path).unwrap();
    println!("  Exported to: {} (human-readable)", tex_path);
    
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Knowledge base is persistent - will be loaded on next run        ║");
    println!("║  New discoveries are marked 'PendingReview' until verified        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
