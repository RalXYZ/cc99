use crate::ast::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct CompileErr {
    code: String,
    message: String,
    label: String,
    span: Span,
    notes: Option<String>,
}

impl std::fmt::Display for CompileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CompileErr {
    pub fn plain_error(message: String, span: Span) -> Self {
        Self {
            code: "E000".to_string(),
            message,
            label: "error here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn duplicated_global_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E001".to_string(),
            message: format!("duplicated global variable `{}`", name),
            label: "global variable duplicated here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn duplicated_symbol(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E002".to_string(),
            message: format!("duplicated symbol `{}`", name),
            label: "symbol duplicated here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn unknown_expression(span: Span) -> CompileErr {
        CompileErr {
            code: "E003".to_string(),
            message: "unknown expression".to_string(),
            label: "unknown expression here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn invalid_default_cast(from: String, to: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E004".to_string(),
            message: "invalid default cast".to_string(),
            label: format!("type invalid here, which cannot be cast into `{}`", to),
            span,
            notes: Some(format!("invalid default cast from `{}` to `{}`", from, to)),
        }
    }

    pub fn invalid_cast(from: String, to: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E005".to_string(),
            message: "invalid cast".to_string(),
            label: "invalid cast here".to_string(),
            span,
            notes: Some(format!("invalid cast from `{}` to `{}`", from, to)),
        }
    }

    pub fn invalid_unary(span: Span) -> CompileErr {
        CompileErr {
            code: "E006".to_string(),
            message: "invalid unary operator".to_string(),
            label: "invalid unary operator here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn invalid_binary(span: Span) -> CompileErr {
        CompileErr {
            code: "E007".to_string(),
            message: "invalid binary operator".to_string(),
            label: "invalid binary operator here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn duplicated_function(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E008".to_string(),
            message: format!("duplicated function `{}`", name),
            label: "function duplicated here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn redefinition_symbol(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E009".to_string(),
            message: format!("redefinition of symbol `{}`", name),
            label: "symbol redefined here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn missing_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E010".to_string(),
            message: format!("missing variable `{}`", name),
            label: "variable missing here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn duplicated_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E011".to_string(),
            message: format!("duplicated variable `{}`", name),
            label: "variable duplicated here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn keyword_not_in_a_loop(keyword: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E012".to_string(),
            message: format!("keyword `{}` is not in a loop", keyword),
            label: "keyword here, which is not in a loop".to_string(),
            span,
            notes: None,
        }
    }

    pub fn array_dimension_mismatch(expect: usize, found: usize, span: Span) -> CompileErr {
        CompileErr {
            code: "E013".to_string(),
            message: "array dimension mismatch".to_string(),
            label: "this array subscript is invalid".to_string(),
            span,
            notes: Some(format!(
                "array has `{}` dimension, but found `{}` subscript",
                expect, found
            )),
        }
    }

    pub fn pointer_dimension_mismatch(expect: usize, found: usize, span: Span) -> CompileErr {
        CompileErr {
            code: "E014".to_string(),
            message: "pointer dimension mismatch".to_string(),
            label: "pointer dimension mismatch here".to_string(),
            span,
            notes: Some(format!(
                "pointer dimension mismatch, expect {}, found {}",
                expect, found
            )),
        }
    }

    pub fn invalid_left_value(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E015".to_string(),
            message: format!(
                "invalid left value for a not addressable variable `{}`",
                name
            )
            ,
            label: "invalid left value here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn invalid_dereference(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E016".to_string(),
            message: format!("invalid dereference for a no pointer variable `{}`", name)
                ,
            label: "invalid dereference here".to_string(),
            span,
            notes: None,
        }
    }

    pub fn parameter_count_mismatch(
        name: String,
        expect: usize,
        found: usize,
        span: Span,
    ) -> CompileErr {
        CompileErr {
            code: "E017".to_string(),
            message: "parameter count mismatch".to_string(),
            label: "function call here".to_string(),
            span,
            notes: Some(format!(
                "parameter count of function `{}` is incorrect, expect {}, found {}",
                name, expect, found
            )),
        }
    }

    pub fn to_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId> {
        Diagnostic::error()
            .with_message(self.message.clone())
            .with_code(self.code.clone())
            .with_labels(vec![Label::primary(
                file_id,
                (self.span.start)..(self.span.end),
            )
            .with_message(self.label.clone())])
            .with_notes(vec![self.notes.clone().unwrap_or("".to_string())])
    }
}
