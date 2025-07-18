use crate::parser::ast::common::{Location, NodeId};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Expression {
    IntLiteral(i64, NodeId, Location),
    FloatLiteral(f64, NodeId, Location),
    StringLiteral(String, NodeId, Location),
    BoolLiteral(bool, NodeId, Location),
    NullLiteral(NodeId, Location),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        id: NodeId,
        loc: Location,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
        id: NodeId,
        loc: Location,
    },
}

#[derive(Debug, Serialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug, Serialize)]
pub enum UnaryOperator {
    Neg, // -expr
    Not, // !expr
}
