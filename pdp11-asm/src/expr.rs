use crate::error::Result;
use crate::{BinaryOp, Expr, UnaryOp};
use std::collections::HashMap;

/// Evaluate an expression to a numeric value
pub fn evaluate(expr: &Expr, symbols: &HashMap<String, i32>, location: i32) -> Result<i32> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Symbol(name) => symbols
            .get(name)
            .copied()
            .ok_or_else(|| crate::error::AsmError::new(0, 0, format!("Undefined symbol: {name}"))),
        Expr::CurrentLocation => Ok(location),
        Expr::Binary { op, left, right } => {
            let left_val = evaluate(left, symbols, location)?;
            let right_val = evaluate(right, symbols, location)?;
            Ok(match op {
                BinaryOp::Add => left_val.wrapping_add(right_val),
                BinaryOp::Sub => left_val.wrapping_sub(right_val),
                BinaryOp::Mul => left_val.wrapping_mul(right_val),
                BinaryOp::Div => {
                    if right_val == 0 {
                        return Err(crate::error::AsmError::new(0, 0, "Division by zero"));
                    }
                    left_val / right_val
                }
                BinaryOp::And => left_val & right_val,
                BinaryOp::Or => left_val | right_val,
                BinaryOp::Xor => left_val ^ right_val,
            })
        }
        Expr::Unary { op, expr } => {
            let val = evaluate(expr, symbols, location)?;
            Ok(match op {
                UnaryOp::Negate => -val,
                UnaryOp::Complement => !val,
            })
        }
    }
}
