use crate::ast::AST;

use serde::Serialize;

use crate::parse::parse;
use crate::preprocess::preprocess;

#[derive(Debug, Serialize)]
struct ParseTreeNode {
    id: String,
    label: String,
    children: Vec<ParseTreeNode>,
}
#[derive(Debug, Serialize)]
struct VisualResult {
    error: bool,
    message: String,
    ast: Box<AST>,
}

//  TODO  use #[wasm_bindgen]
pub fn compile_result(code: &str) -> String {
    let mut result = VisualResult {
        error: false,
        message: String::from(""),
        ast: Box::new(AST::GlobalDeclaration(vec![])),
    };
    match preprocess(code) {
        Ok(code) => match parse(&code) {
            Ok(ast) => {
                result.ast = ast;
            }
            Err(error) => {
                result.error = true;
                result.message = error.to_string();
            }
        },
        Err(error) => {
            result.error = true;
            result.message = error.to_string();
        }
    };
    serde_json::to_string(&result).unwrap()
}
