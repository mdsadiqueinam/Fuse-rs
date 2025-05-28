// Example demonstrating the query parser functionality
use fuse_rs::{Expression, FuseOptions, ParseOptions, parse};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    // Example 1: Simple key-value query
    println!("=== Simple Key-Value Query ===");
    let mut simple_query = HashMap::new();
    simple_query.insert("title".to_string(), "rust programming".to_string());
    let simple_expr = Expression::KeyValue(simple_query);
    
    let result = parse(simple_expr, &options, &parse_opts)?;
    println!("Parsed simple query: {:#?}", result);

    // Example 2: Path-based query
    println!("\n=== Path-Based Query ===");
    let path_expr = Expression::PathValue {
        path: vec!["author".to_string(), "name".to_string()],
        val: "John Doe".to_string(),
    };
    
    let result = parse(path_expr, &options, &parse_opts)?;
    println!("Parsed path query: {:#?}", result);

    // Example 3: AND query with multiple conditions
    println!("\n=== AND Query ===");
    let mut title_query = HashMap::new();
    title_query.insert("title".to_string(), "rust".to_string());
    
    let mut author_query = HashMap::new();
    author_query.insert("author".to_string(), "smith".to_string());

    let and_expr = Expression::And {
        and: vec![
            Expression::KeyValue(title_query),
            Expression::KeyValue(author_query),
        ],
    };
    
    let result = parse(and_expr, &options, &parse_opts)?;
    println!("Parsed AND query: {:#?}", result);

    // Example 4: OR query with nested conditions
    println!("\n=== OR Query ===");
    let or_expr = Expression::Or {
        or: vec![
            Expression::PathValue {
                path: vec!["category".to_string()],
                val: "tech".to_string(),
            },
            Expression::PathValue {
                path: vec!["category".to_string()],
                val: "programming".to_string(),
            },
        ],
    };
    
    let result = parse(or_expr, &options, &parse_opts)?;
    println!("Parsed OR query: {:#?}", result);

    Ok(())
}