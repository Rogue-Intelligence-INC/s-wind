//! Knowledge Base Storage
//! 
//! Persistent storage for knowledge base using JSON format.
//! TeX import/export for human-readable format.

use super::KnowledgeBase;
use std::fs;
use std::path::Path;

/// Default storage path
pub const DEFAULT_KNOWLEDGE_PATH: &str = "knowledge_base.json";

/// Save knowledge base to JSON file
pub fn save_to_json(kb: &KnowledgeBase, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(kb)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    fs::write(path, json)
        .map_err(|e| format!("File write error: {}", e))?;
    
    Ok(())
}

/// Load knowledge base from JSON file
pub fn load_from_json(path: &str) -> Result<KnowledgeBase, String> {
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }
    
    let json = fs::read_to_string(path)
        .map_err(|e| format!("File read error: {}", e))?;
    
    let kb: KnowledgeBase = serde_json::from_str(&json)
        .map_err(|e| format!("Deserialization error: {}", e))?;
    
    Ok(kb)
}

/// Load or create new knowledge base
pub fn load_or_create(path: &str) -> KnowledgeBase {
    match load_from_json(path) {
        Ok(kb) => {
            println!("Loaded knowledge base from {}", path);
            println!("{}", kb.stats());
            kb
        }
        Err(_) => {
            println!("Creating new knowledge base...");
            let mut kb = KnowledgeBase::new();
            kb.initialize_fundamentals();
            println!("{}", kb.stats());
            
            // Save immediately
            if let Err(e) = save_to_json(&kb, path) {
                eprintln!("Warning: Could not save knowledge base: {}", e);
            }
            
            kb
        }
    }
}

/// Export knowledge base to TeX format (for human review)
pub fn export_to_tex(kb: &KnowledgeBase) -> String {
    let mut tex = String::new();
    
    tex.push_str(r"\documentclass[11pt]{article}
\usepackage[utf8]{inputenc}
\usepackage{amsmath,amssymb}
\usepackage{longtable}
\usepackage{geometry}
\geometry{margin=1in}

\title{S-wind Knowledge Base Export}
\date{\today}

\begin{document}
\maketitle

");
    
    // Physical Constants
    tex.push_str(r"\section{Physical Constants}
\begin{longtable}{|l|l|l|l|}
\hline
\textbf{Symbol} & \textbf{Name} & \textbf{Value} & \textbf{Unit} \\
\hline
\endhead
");
    
    for c in &kb.physical_constants {
        tex.push_str(&format!(
            "${} $ & {} & {} & {} \\\\\n\\hline\n",
            c.symbol, c.name, c.value, c.unit
        ));
    }
    tex.push_str(r"\end{longtable}

");
    
    // Mathematical Constants
    tex.push_str(r"\section{Mathematical Constants}
\begin{longtable}{|l|l|l|}
\hline
\textbf{Symbol} & \textbf{Name} & \textbf{Value} \\
\hline
\endhead
");
    
    for c in &kb.math_constants {
        tex.push_str(&format!(
            "${} $ & {} & {:.10} \\\\\n\\hline\n",
            c.symbol, c.name, c.value_approx
        ));
    }
    tex.push_str(r"\end{longtable}

");
    
    // Theorems
    tex.push_str(r"\section{Mathematical Theorems}

");
    
    for t in &kb.theorems {
        tex.push_str(&format!(
            r"\subsection{{{}}}\n${}$\n\n",
            t.name, t.statement_tex
        ));
    }
    
    // Physical Laws
    tex.push_str(r"\section{Physical Laws}

");
    
    for l in &kb.physical_laws {
        tex.push_str(&format!(
            r"\subsection{{{}}}\n${}$\n\n{}\n\n",
            l.name, l.equation_tex, l.description
        ));
    }
    
    // Discovered Equations
    if !kb.discovered.is_empty() {
        tex.push_str(r"\section{Discovered Equations (S-wind)}

");
        
        for d in &kb.discovered {
            let status = match d.status {
                super::KnowledgeStatus::Verified => "Verified",
                super::KnowledgeStatus::PendingReview => "\\textbf{PENDING REVIEW}",
                super::KnowledgeStatus::Rejected => "Rejected",
            };
            
            tex.push_str(&format!(
                r"\subsection{{{} [{}]}}\n${}$\n\n{}\n\n",
                d.id, status, d.equation_tex, d.description
            ));
        }
    }
    
    tex.push_str(r"\end{document}
");
    
    tex
}

/// Export to TeX file
pub fn export_to_tex_file(kb: &KnowledgeBase, path: &str) -> Result<(), String> {
    let tex = export_to_tex(kb);
    fs::write(path, tex)
        .map_err(|e| format!("File write error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_save_load_roundtrip() {
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        let test_path = "/tmp/test_kb.json";
        
        // Save
        save_to_json(&kb, test_path).unwrap();
        
        // Load
        let loaded = load_from_json(test_path).unwrap();
        
        assert_eq!(kb.physical_constants.len(), loaded.physical_constants.len());
        assert_eq!(kb.math_constants.len(), loaded.math_constants.len());
        assert_eq!(kb.theorems.len(), loaded.theorems.len());
        
        // Cleanup
        let _ = fs::remove_file(test_path);
    }
    
    #[test]
    fn test_export_tex() {
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        let tex = export_to_tex(&kb);
        
        assert!(tex.contains("Physical Constants"));
        assert!(tex.contains("Mathematical Theorems"));
        assert!(tex.contains("Physical Laws"));
    }
}
