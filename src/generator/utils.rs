use crate::generator::Generator;
use crate::utils::CompileErr as CE;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn no_terminator(&self) -> bool {
        let block = self.builder.get_insert_block();
        let terminator = block.unwrap().get_terminator();
        return terminator.is_none();
    }

    pub(crate) fn gen_err_output(&self, file_id: usize, e: &CE) {
        use codespan_reporting::diagnostic::{Diagnostic, Label};
        use codespan_reporting::term;
        use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

        let diagnostic = Diagnostic::error()
            .with_message(e.message.clone())
            .with_code(e.code.clone())
            .with_labels(vec![
                Label::primary(file_id, (e.span.start)..(e.span.end)).with_message(e.label.clone())
            ]);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();

        term::emit(&mut writer.lock(), &config, &self.files, &diagnostic);
    }
}
