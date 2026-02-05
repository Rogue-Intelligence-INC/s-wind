//! 加载所有下载的 OEIS 序列

use s_wind::{OeisLoader, KnowledgeBase};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              加载 OEIS b-files                                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    let mut loader = OeisLoader::new();
    
    // 加载内置序列
    let builtin = loader.load_builtin();
    println!("内置序列: {} 个", builtin);
    
    // 加载下载的 b-files
    match loader.load_bfiles_from_dir("data/oeis") {
        Ok(count) => println!("下载序列: {} 个", count),
        Err(e) => println!("加载失败: {}", e),
    }
    
    // 统计
    println!("\n=== 总计 ===");
    println!("缓存序列数: {}", loader.search(&[]).len()); // 搜索空数组返回所有
    
    // 测试搜索
    println!("\n=== 搜索测试 ===");
    
    // Fibonacci
    let fib = loader.search(&[0, 1, 1, 2, 3, 5, 8, 13]);
    println!("搜索 [0,1,1,2,3,5,8,13]: {:?}", fib.iter().map(|s| &s.id).collect::<Vec<_>>());
    
    // Primes
    let primes = loader.search(&[2, 3, 5, 7, 11, 13]);
    println!("搜索 [2,3,5,7,11,13]: {:?}", primes.iter().map(|s| &s.id).collect::<Vec<_>>());
    
    // Squares
    let squares = loader.search(&[0, 1, 4, 9, 16, 25]);
    println!("搜索 [0,1,4,9,16,25]: {:?}", squares.iter().map(|s| &s.id).collect::<Vec<_>>());
    
    // 添加到知识库
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let added = loader.add_to_knowledge_base(&mut kb);
    println!("\n添加到知识库: {} 个序列", added);
    println!("知识库已知序列总数: {}", kb.known_sequences.len());
    
    println!("\n完成!");
}
