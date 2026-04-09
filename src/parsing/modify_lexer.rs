//! Lexer for MODIFY token expressions
//! 
//! Provides basic tokenization of MODIFY expressions using string splitting.
//! Format: VarName|Operation|Value

/// Represents a token in a MODIFY expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifyToken {
    Variable(String),
    Pipe,
    Operation(String),
    Value(String),
}

/// Tokenize a MODIFY expression into parts
pub fn tokenize(input: &str) -> Result<(String, String, String), String> {
    let parts: Vec<&str> = input.split('|').collect();
    
    if parts.len() < 3 {
        return Err(format!(
            "MODIFY expression must have at least 3 pipe-separated parts, got {}",
            parts.len()
        ));
    }
    
    let var = parts[0].trim();
    let op = parts[1].trim();
    let val = parts[2..].join("|").trim().to_string();
    
    if var.is_empty() {
        return Err("MODIFY variable name cannot be empty".to_string());
    }
    
    if op.is_empty() {
        return Err("MODIFY operation cannot be empty".to_string());
    }
    
    Ok((var.to_string(), op.to_string(), val))
}
