mod lexer;
use self::lexer::Token;

use ast;

pub fn parse(s: &str) -> Result<ast::SExpr, String> {
    unimplemented!()
}


struct Parser {
    input: Vec<Token>,
}
