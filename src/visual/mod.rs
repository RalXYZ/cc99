use serde::Serialize;
use wasm_bindgen::prelude::*;

use super::ast::AST;
use super::parse::parse;
use super::preprocess::preprocess;

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

#[wasm_bindgen]
pub fn compile_result(code: &str) -> String {
    let mut result = VisualResult {
        error: false,
        message: String::from(""),
        ast: Box::new(AST::GlobalDeclarations(vec![])),
    };
    let include_dirs = vec![];
    match preprocess(code, &include_dirs) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_result() {
        let result = compile_result("let x = 1;");
        let result = serde_json::to_string(&result).unwrap();
        println!("{}", result);
    }
}
