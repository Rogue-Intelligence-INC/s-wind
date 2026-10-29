//! Computer Science Module for Blondle
//!
//! Covers:
//! - Algorithm complexity (Big-O)
//! - Data structure properties
//! - Computational theory fundamentals
//! - Information theory
//! - Cryptography basics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all computer science knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::ComputerScience] Loaded {} constants, {} laws, {} theorems",
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

const CONSTANT_COUNT: usize = 10;
const LAW_COUNT: usize = 10;
const THEOREM_COUNT: usize = 25;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Information theory
        PhysicalConstant {
            id: "nat".into(),
            name: "Natural unit of information".into(),
            symbol: "nat".into(),
            value: "1.4427".into(),
            unit: "bits".into(),
            uncertainty: Some("exact".into()),
            source: "Information theory".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ nat} = \frac{1}{\ln 2} \approx 1.4427 \text{ bits}".into(),
        },
        PhysicalConstant {
            id: "hartley".into(),
            name: "Hartley (decimal digit)".into(),
            symbol: "Hart".into(),
            value: "3.322".into(),
            unit: "bits".into(),
            uncertainty: Some("exact".into()),
            source: "Information theory".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ Hart} = \log_2 10 \approx 3.322 \text{ bits}".into(),
        },
        
        // Algorithmic constants
        PhysicalConstant {
            id: "golden_ratio_algo".into(),
            name: "Golden ratio (Fibonacci analysis)".into(),
            symbol: r"\varphi".into(),
            value: "1.618033988749895".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Fibonacci sequence".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\varphi = \frac{1+\sqrt{5}}{2} \approx 1.618 \text{ (Fib ratio)}".into(),
        },
        PhysicalConstant {
            id: "ln2".into(),
            name: "Natural log of 2".into(),
            symbol: r"\ln 2".into(),
            value: "0.693147180559945".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Mathematics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\ln 2 = 0.6931... \text{ (binary entropy factor)}".into(),
        },
        PhysicalConstant {
            id: "log2_e".into(),
            name: "Log base 2 of e".into(),
            symbol: r"\log_2 e".into(),
            value: "1.4426950408889634".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("exact".into()),
            source: "Mathematics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\log_2 e = \frac{1}{\ln 2} \approx 1.4427".into(),
        },
        
        // Complexity bounds
        PhysicalConstant {
            id: "sorting_lower_bound".into(),
            name: "Comparison sorting lower bound".into(),
            symbol: "T_{sort}".into(),
            value: "n log n".into(),
            unit: "comparisons".into(),
            uncertainty: None,
            source: "Decision tree argument".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Omega(n \log n) \text{ comparisons for sorting } n \text{ elements}".into(),
        },
        PhysicalConstant {
            id: "matrix_mult_bound".into(),
            name: "Matrix multiplication exponent (best known)".into(),
            symbol: r"\omega".into(),
            value: "2.3728596".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.0001".into()),
            source: "Alman-Williams 2021".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\omega < 2.3729 \text{ (matrix mult exponent)}".into(),
        },
        
        // Hash constants
        PhysicalConstant {
            id: "md5_bits".into(),
            name: "MD5 digest size".into(),
            symbol: "MD5".into(),
            value: "128".into(),
            unit: "bits".into(),
            uncertainty: None,
            source: "RFC 1321".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{MD5: } 128 \text{ bits (broken)}".into(),
        },
        PhysicalConstant {
            id: "sha256_bits".into(),
            name: "SHA-256 digest size".into(),
            symbol: "SHA256".into(),
            value: "256".into(),
            unit: "bits".into(),
            uncertainty: None,
            source: "FIPS 180-4".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{SHA-256: } 256 \text{ bits}".into(),
        },
        PhysicalConstant {
            id: "aes_key_size".into(),
            name: "AES key sizes".into(),
            symbol: "AES".into(),
            value: "128, 192, 256".into(),
            unit: "bits".into(),
            uncertainty: None,
            source: "FIPS 197".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\text{AES: } 128/192/256 \text{ bit keys}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Information theory
        PhysicalLaw {
            id: "shannon_entropy".into(),
            name: "Shannon Entropy".into(),
            domain: PhysicsDomain::Other("Information Theory".into()),
            equation_tex: r"H(X) = -\sum_i p_i \log_2 p_i".into(),
            description: "Measure of information content".into(),
            variables: vec![
                Variable { symbol: "H".into(), name: "Entropy".into(), unit: Some("bits".into()) },
                Variable { symbol: "p_i".into(), name: "Probability".into(), unit: None },
            ],
            source: "Shannon 1948".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "channel_capacity".into(),
            name: "Shannon Channel Capacity".into(),
            domain: PhysicsDomain::Other("Information Theory".into()),
            equation_tex: r"C = B \log_2\left(1 + \frac{S}{N}\right)".into(),
            description: "Maximum reliable transmission rate".into(),
            variables: vec![
                Variable { symbol: "C".into(), name: "Channel capacity".into(), unit: Some("bits/s".into()) },
                Variable { symbol: "B".into(), name: "Bandwidth".into(), unit: Some("Hz".into()) },
                Variable { symbol: "S/N".into(), name: "Signal-to-noise ratio".into(), unit: None },
            ],
            source: "Shannon 1948".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "kolmogorov_complexity".into(),
            name: "Kolmogorov Complexity".into(),
            domain: PhysicsDomain::Other("Information Theory".into()),
            equation_tex: r"K(x) = \min\{|p| : U(p) = x\}".into(),
            description: "Shortest program length to produce string x".into(),
            variables: vec![
                Variable { symbol: "K(x)".into(), name: "Kolmogorov complexity".into(), unit: Some("bits".into()) },
                Variable { symbol: "U".into(), name: "Universal Turing machine".into(), unit: None },
            ],
            source: "Kolmogorov 1965".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Complexity
        PhysicalLaw {
            id: "master_theorem".into(),
            name: "Master Theorem".into(),
            domain: PhysicsDomain::Other("Algorithms".into()),
            equation_tex: r"T(n) = aT(n/b) + f(n)".into(),
            description: "Recurrence relation for divide-and-conquer".into(),
            variables: vec![
                Variable { symbol: "T(n)".into(), name: "Time complexity".into(), unit: None },
                Variable { symbol: "a".into(), name: "Number of subproblems".into(), unit: None },
                Variable { symbol: "b".into(), name: "Subproblem size factor".into(), unit: None },
            ],
            source: "CLRS textbook".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "amdahl_law".into(),
            name: "Amdahl's Law".into(),
            domain: PhysicsDomain::Other("Parallel Computing".into()),
            equation_tex: r"S(n) = \frac{1}{(1-p) + \frac{p}{n}}".into(),
            description: "Speedup limit from parallelization".into(),
            variables: vec![
                Variable { symbol: "S".into(), name: "Speedup".into(), unit: None },
                Variable { symbol: "p".into(), name: "Parallelizable fraction".into(), unit: None },
                Variable { symbol: "n".into(), name: "Number of processors".into(), unit: None },
            ],
            source: "Amdahl 1967".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "gustafson_law".into(),
            name: "Gustafson's Law".into(),
            domain: PhysicsDomain::Other("Parallel Computing".into()),
            equation_tex: r"S(n) = n - (n-1)(1-p)".into(),
            description: "Scaled speedup with larger problems".into(),
            variables: vec![
                Variable { symbol: "S".into(), name: "Scaled speedup".into(), unit: None },
                Variable { symbol: "n".into(), name: "Number of processors".into(), unit: None },
            ],
            source: "Gustafson 1988".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Cryptography
        PhysicalLaw {
            id: "birthday_attack".into(),
            name: "Birthday Attack Bound".into(),
            domain: PhysicsDomain::Other("Cryptography".into()),
            equation_tex: r"n \approx 1.17\sqrt{|H|}".into(),
            description: "Expected trials for hash collision".into(),
            variables: vec![
                Variable { symbol: "n".into(), name: "Expected trials".into(), unit: None },
                Variable { symbol: "|H|".into(), name: "Hash space size".into(), unit: None },
            ],
            source: "Birthday paradox application".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Network
        PhysicalLaw {
            id: "little_law".into(),
            name: "Little's Law".into(),
            domain: PhysicsDomain::Other("Queueing Theory".into()),
            equation_tex: r"L = \lambda W".into(),
            description: "Average items in system".into(),
            variables: vec![
                Variable { symbol: "L".into(), name: "Average items in system".into(), unit: None },
                Variable { symbol: "λ".into(), name: "Arrival rate".into(), unit: Some("items/s".into()) },
                Variable { symbol: "W".into(), name: "Average time in system".into(), unit: Some("s".into()) },
            ],
            source: "Little 1961".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "moore_law".into(),
            name: "Moore's Law (historical)".into(),
            domain: PhysicsDomain::Other("Computing History".into()),
            equation_tex: r"N(t) = N_0 \cdot 2^{t/T}".into(),
            description: "Transistor count doubles every ~2 years".into(),
            variables: vec![
                Variable { symbol: "N".into(), name: "Transistor count".into(), unit: None },
                Variable { symbol: "T".into(), name: "Doubling period".into(), unit: Some("years".into()) },
            ],
            source: "Moore 1965".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "landauer_limit".into(),
            name: "Landauer's Principle".into(),
            domain: PhysicsDomain::Other("Thermodynamic Computing".into()),
            equation_tex: r"E_{min} = k_B T \ln 2".into(),
            description: "Minimum energy to erase one bit".into(),
            variables: vec![
                Variable { symbol: "E_min".into(), name: "Minimum energy".into(), unit: Some("J".into()) },
                Variable { symbol: "T".into(), name: "Temperature".into(), unit: Some("K".into()) },
            ],
            source: "Landauer 1961".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        // Complexity classes
        MathTheorem {
            id: "p_np_relation".into(),
            name: "P ⊆ NP".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"P \subseteq NP".into(),
            proof_tex: Some("Every polynomial-time verifier can simulate polynomial-time solver".into()),
            references: vec![],
            source: "Complexity theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "np_complete_definition".into(),
            name: "NP-Completeness Definition".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"L \in \text{NP-complete} \iff L \in NP \land \forall L' \in NP: L' \leq_p L".into(),
            proof_tex: Some("In NP and all NP problems reduce to it".into()),
            references: vec![],
            source: "Cook 1971, Karp 1972".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "cook_levin".into(),
            name: "Cook-Levin Theorem".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"\text{SAT is NP-complete}".into(),
            proof_tex: Some("First NP-complete problem, via TM simulation".into()),
            references: vec![],
            source: "Cook 1971, Levin 1973".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "time_hierarchy".into(),
            name: "Time Hierarchy Theorem".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"DTIME(o(f(n))) \subsetneq DTIME(f(n)\log f(n))".into(),
            proof_tex: Some("More time allows solving strictly more problems".into()),
            references: vec![],
            source: "Hartmanis, Stearns 1965".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "space_hierarchy".into(),
            name: "Space Hierarchy Theorem".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"DSPACE(o(f(n))) \subsetneq DSPACE(f(n))".into(),
            proof_tex: Some("More space allows solving strictly more problems".into()),
            references: vec![],
            source: "Hartmanis, Stearns 1965".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "savitch".into(),
            name: "Savitch's Theorem".into(),
            category: TheoremCategory::Other("Computational Complexity".into()),
            statement_tex: r"NSPACE(f(n)) \subseteq DSPACE(f(n)^2)".into(),
            proof_tex: Some("Nondeterministic space quadratically simulated".into()),
            references: vec![],
            source: "Savitch 1970".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Algorithm analysis
        MathTheorem {
            id: "sorting_complexity".into(),
            name: "Comparison Sort Lower Bound".into(),
            category: TheoremCategory::Other("Algorithms".into()),
            statement_tex: r"\text{Any comparison sort: } \Omega(n \log n)".into(),
            proof_tex: Some("Decision tree has n! leaves, height ≥ log(n!)".into()),
            references: vec![],
            source: "Information-theoretic argument".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "quicksort_average".into(),
            name: "Quicksort Average Case".into(),
            category: TheoremCategory::Other("Algorithms".into()),
            statement_tex: r"T_{avg}(n) = O(n \log n)".into(),
            proof_tex: Some("With random pivot, expected comparisons ~1.39n log n".into()),
            references: vec![],
            source: "Hoare 1962".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "binary_search".into(),
            name: "Binary Search Complexity".into(),
            category: TheoremCategory::Other("Algorithms".into()),
            statement_tex: r"T(n) = O(\log n)".into(),
            proof_tex: Some("Halves search space each iteration".into()),
            references: vec![],
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "dijkstra_complexity".into(),
            name: "Dijkstra's Algorithm Complexity".into(),
            category: TheoremCategory::Other("Algorithms".into()),
            statement_tex: r"T(V,E) = O((V+E)\log V) \text{ with binary heap}".into(),
            proof_tex: Some("O(E log V) with Fibonacci heap".into()),
            references: vec![],
            source: "Dijkstra 1959".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Data structures
        MathTheorem {
            id: "bst_balance".into(),
            name: "Balanced BST Operations".into(),
            category: TheoremCategory::Other("Data Structures".into()),
            statement_tex: r"\text{Insert/Delete/Search: } O(\log n)".into(),
            proof_tex: Some("AVL, Red-Black, B-trees maintain height O(log n)".into()),
            references: vec![],
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "hash_table_average".into(),
            name: "Hash Table Average Case".into(),
            category: TheoremCategory::Other("Data Structures".into()),
            statement_tex: r"\text{Insert/Delete/Search: } O(1) \text{ expected}".into(),
            proof_tex: Some("With good hash function and load factor < 1".into()),
            references: vec![],
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "fibonacci_heap".into(),
            name: "Fibonacci Heap Amortized".into(),
            category: TheoremCategory::Other("Data Structures".into()),
            statement_tex: r"\text{Insert: } O(1), \text{ Extract-Min: } O(\log n), \text{ Decrease-Key: } O(1)".into(),
            proof_tex: Some("Amortized analysis via potential function".into()),
            references: vec![],
            source: "Fredman, Tarjan 1987".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Computability
        MathTheorem {
            id: "halting_undecidable".into(),
            name: "Halting Problem Undecidability".into(),
            category: TheoremCategory::Logic,
            statement_tex: r"\text{HALT} = \{(M, x) : M \text{ halts on } x\} \text{ is undecidable}".into(),
            proof_tex: Some("Diagonalization: assume decider H, construct contradiction".into()),
            references: vec![],
            source: "Turing 1936".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "rice_theorem".into(),
            name: "Rice's Theorem".into(),
            category: TheoremCategory::Logic,
            statement_tex: r"\text{Any non-trivial semantic property of programs is undecidable}".into(),
            proof_tex: Some("Reduces from halting problem".into()),
            references: vec!["halting_undecidable".into()],
            source: "Rice 1953".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "church_turing".into(),
            name: "Church-Turing Thesis".into(),
            category: TheoremCategory::Logic,
            statement_tex: r"\text{Effectively computable} \equiv \text{Turing computable}".into(),
            proof_tex: Some("Thesis, not theorem; equivalence of computation models".into()),
            references: vec![],
            source: "Church, Turing 1936".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Graph theory (CS perspective)
        MathTheorem {
            id: "euler_path".into(),
            name: "Eulerian Path Existence".into(),
            category: TheoremCategory::Combinatorics,
            statement_tex: r"\text{Eulerian path exists} \iff \text{exactly 0 or 2 odd-degree vertices}".into(),
            proof_tex: Some("Eulerian circuit: all even degree".into()),
            references: vec![],
            source: "Euler 1736".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "handshaking".into(),
            name: "Handshaking Lemma".into(),
            category: TheoremCategory::Combinatorics,
            statement_tex: r"\sum_{v \in V} \deg(v) = 2|E|".into(),
            proof_tex: Some("Each edge contributes 2 to total degree".into()),
            references: vec![],
            source: "Graph theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "planar_faces".into(),
            name: "Euler's Formula (Planar Graphs)".into(),
            category: TheoremCategory::Combinatorics,
            statement_tex: r"V - E + F = 2".into(),
            proof_tex: Some("For connected planar graph".into()),
            references: vec![],
            source: "Euler 1752".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "k33_k5_planar".into(),
            name: "Kuratowski's Theorem".into(),
            category: TheoremCategory::Combinatorics,
            statement_tex: r"G \text{ planar} \iff G \text{ has no } K_5 \text{ or } K_{3,3} \text{ subdivision}".into(),
            proof_tex: Some("Characterization of planar graphs".into()),
            references: vec![],
            source: "Kuratowski 1930".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Cryptography
        MathTheorem {
            id: "rsa_security".into(),
            name: "RSA Security Assumption".into(),
            category: TheoremCategory::Other("Cryptography".into()),
            statement_tex: r"\text{Factoring } n = pq \text{ is hard for large primes}".into(),
            proof_tex: Some("No known polynomial-time classical algorithm".into()),
            references: vec![],
            source: "RSA 1977".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "diffie_hellman".into(),
            name: "Diffie-Hellman Key Exchange".into(),
            category: TheoremCategory::Other("Cryptography".into()),
            statement_tex: r"g^{ab} = (g^a)^b = (g^b)^a \mod p".into(),
            proof_tex: Some("Secure under discrete log assumption".into()),
            references: vec![],
            source: "Diffie, Hellman 1976".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "one_time_pad".into(),
            name: "One-Time Pad Perfect Secrecy".into(),
            category: TheoremCategory::Other("Cryptography".into()),
            statement_tex: r"P(M|C) = P(M) \text{ when key as long as message}".into(),
            proof_tex: Some("Information-theoretically secure".into()),
            references: vec![],
            source: "Shannon 1949".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
