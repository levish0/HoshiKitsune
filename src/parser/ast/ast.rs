use crate::parser::ast::expr::Expression;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Program<'a> {
    pub body: Vec<Element<'a>>,
}

#[derive(Debug, Serialize)]
pub enum Element<'a> {
    Expression(Expression<'a>),
    // 향후 이곳에 구문(Statement), 함수 선언(FunctionDeclaration) 등
    // 다른 최상위 요소들을 추가할 수 있습니다.
}
