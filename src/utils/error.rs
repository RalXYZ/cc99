use crate::ast::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct CompileErr {
    pub code: String,
    pub message: String,
    pub label: String,
    pub span: Span,
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
        }
    }

    pub fn duplicated_global_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E001".to_string(),
            message: format!("duplicated global variable: `{}`", name),
            label: "global variable duplicated here".to_string(),
            span,
        }
    }

    pub fn duplicated_symbol(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E002".to_string(),
            message: format!("duplicated symbol: `{}`", name),
            label: "symbol duplicated here".to_string(),
            span,
        }
    }

    pub fn unknown_expression(span: Span) -> CompileErr {
        CompileErr {
            code: "E003".to_string(),
            message: "unknown expression".to_string(),
            label: "unknown expression here".to_string(),
            span,
        }
    }

    pub fn invalid_default_cast(from: String, to: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E004".to_string(),
            message: format!("invalid default cast between {} and {}", from, to),
            label: "invalid default cast here".to_string(),
            span,
        }
    }

    pub fn invalid_cast(from: String, to: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E005".to_string(),
            message: format!("invalid cast from {} to {}", from, to),
            label: "invalid cast here".to_string(),
            span,
        }
    }

    pub fn invalid_unary(span: Span) -> CompileErr {
        CompileErr {
            code: "E006".to_string(),
            message: "invalid unary operator".to_string(),
            label: "invalid unary operator here".to_string(),
            span,
        }
    }

    pub fn invalid_binary(span: Span) -> CompileErr {
        CompileErr {
            code: "E007".to_string(),
            message: "invalid binary operator".to_string(),
            label: "invalid binary operator here".to_string(),
            span,
        }
    }

    pub fn duplicated_function(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E008".to_string(),
            message: format!("duplicated function `{}`", name),
            label: "function duplicated here".to_string(),
            span,
        }
    }

    pub fn redefinition_symbol(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E009".to_string(),
            message: format!("redefinition of symbol `{}`", name),
            label: "symbol redefined here".to_string(),
            span,
        }
    }

    pub fn missing_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E010".to_string(),
            message: format!("missing variable `{}`", name),
            label: "variable missing here".to_string(),
            span,
        }
    }

    pub fn duplicated_variable(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E011".to_string(),
            message: format!("duplicated variable `{}`", name),
            label: "variable duplicated here".to_string(),
            span,
        }
    }

    pub fn keyword_not_in_a_loop(keyword: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E012".to_string(),
            message: format!("keyword `{}` is not in a loop", keyword),
            label: "keyword here, which is not in a loop".to_string(),
            span,
        }
    }

    pub fn array_dimension_mismatch(expect: usize, found: usize, span: Span) -> CompileErr {
        CompileErr {
            code: "E013".to_string(),
            message: format!("array dimension mismatch, expect {}, found {}", expect, found).to_string(),
            label: "array dimension mismatch here".to_string(),
            span,
        }
    }

    pub fn pointer_dimension_mismatch(expect: usize, found: usize, span: Span) -> CompileErr {
        CompileErr {
            code: "E014".to_string(),
            message: format!("pointer dimension mismatch, expect {}, found {}", expect, found).to_string(),
            label: "pointer dimension mismatch here".to_string(),
            span,
        }
    }

    pub fn invalid_left_value(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E015".to_string(),
            message: format!("invalid left value for a not addressable variable `{}`", name).to_string(),
            label: "invalid left value here".to_string(),
            span,
        }
    }

    pub fn invalid_dereference(name: String, span: Span) -> CompileErr {
        CompileErr {
            code: "E016".to_string(),
            message: format!("invalid dereference for a no pointer variable `{}`", name).to_string(),
            label: "invalid dereference here".to_string(),
            span,
        }
    }

    pub fn parameter_count_mismatch(name: String, expect: usize, found: usize, span: Span) -> CompileErr {
        CompileErr {
            code: "E017".to_string(),
            message: format!("parameter count of `{}` mismatch, expect {}, found {}", name, expect, found).to_string(),
            label: "parameter count mismatch here".to_string(),
            span,
        }
    }
}
