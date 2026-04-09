//! Parser for MODIFY token expressions
//!
//! MODIFY tokens have the format: `VarName|Operation|Value`
//! where Operation is one of: ADD, SET, SOLVE
//! and Value can be a number, formula, or identifier
//!
//! Uses a lexer-based approach for tokenization and parsing.

use std::fmt;
use crate::parsing::modify_lexer::tokenize;

/// Represents a parsed MODIFY expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyExpression {
    pub variable: String,
    pub operation: ModifyOp,
    pub value: String,
}

/// The operation type in a MODIFY expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifyOp {
    Add,
    Set,
    Solve,
}

impl fmt::Display for ModifyOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifyOp::Add => write!(f, "ADD"),
            ModifyOp::Set => write!(f, "SET"),
            ModifyOp::Solve => write!(f, "SOLVE"),
        }
    }
}

impl ModifyOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ADD" => Some(ModifyOp::Add),
            "SET" => Some(ModifyOp::Set),
            "SOLVE" => Some(ModifyOp::Solve),
            _ => None,
        }
    }
}

// Doctest removed: module parsing is private, see unit tests in mod tests below
pub fn parse_modify(input: &str) -> Result<ModifyExpression, String> {
    // Use the lexer/tokenizer
    let (var, op_str, val) = tokenize(input)?;
    
    let operation = ModifyOp::from_str(&op_str)
        .ok_or_else(|| format!("Unknown operation: {}. Expected ADD, SET, or SOLVE", op_str))?;
    
    Ok(ModifyExpression {
        variable: var,
        operation,
        value: val,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modify_add() {
        let result = parse_modify("AndroidUpgradeSlotTaken|ADD|1").unwrap();
        assert_eq!(result.variable, "AndroidUpgradeSlotTaken");
        assert_eq!(result.operation, ModifyOp::Add);
        assert_eq!(result.value, "1");
    }

    #[test]
    fn test_parse_modify_set_dice() {
        let result = parse_modify("Damage|SET|10d6").unwrap();
        assert_eq!(result.variable, "Damage");
        assert_eq!(result.operation, ModifyOp::Set);
        assert_eq!(result.value, "10d6");
    }

    #[test]
    fn test_parse_modify_set_identifier() {
        let result = parse_modify("CHASCORE|SET|Score").unwrap();
        assert_eq!(result.variable, "CHASCORE");
        assert_eq!(result.operation, ModifyOp::Set);
        assert_eq!(result.value, "Score");
    }

    #[test]
    fn test_parse_modify_with_compound_var() {
        let result = parse_modify("ABILITYPOOL=Traits|SOLVE|2").unwrap();
        assert_eq!(result.variable, "ABILITYPOOL=Traits");
        assert_eq!(result.operation, ModifyOp::Solve);
        assert_eq!(result.value, "2");
    }

    #[test]
    fn test_parse_modify_with_complex_formula() {
        let result = parse_modify("ABILITYPOOL=Traits|SOLVE|value()+if(count(\"ABILITY\",\"Special Ability\",\"GROUP=Drawback\"))>=1,1,0)").unwrap();
        assert_eq!(result.variable, "ABILITYPOOL=Traits");
        assert_eq!(result.operation, ModifyOp::Solve);
        assert_eq!(result.value, "value()+if(count(\"ABILITY\",\"Special Ability\",\"GROUP=Drawback\"))>=1,1,0)");
    }

    #[test]
    fn test_parse_modify_with_pipes_in_value() {
        let result = parse_modify("Var|SET|value|with|pipes").unwrap();
        assert_eq!(result.variable, "Var");
        assert_eq!(result.operation, ModifyOp::Set);
        assert_eq!(result.value, "value|with|pipes");
    }
}
