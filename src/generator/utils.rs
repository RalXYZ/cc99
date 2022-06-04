use crate::ast::{BaseType, Span};
use crate::generator::Generator;
use crate::utils::CompileErr as CE;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn no_terminator(&self) -> bool {
        let block = self.builder.get_insert_block();
        let terminator = block.unwrap().get_terminator();
        terminator.is_none()
    }

    pub(crate) fn gen_err_output(&self, file_id: usize, e: &CE) {
        use codespan_reporting::term;
        use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

        let diagnostic = e.to_diagnostic(file_id);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();

        term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).expect("unreachable");
    }

    pub(crate) fn extend_struct_type(&self, t: BaseType, span: Span) -> Result<BaseType, CE> {
        match t {
            BaseType::Struct(ref name, ref _members) => {
                let members = self.global_struct_map.get(name.as_ref().unwrap());
                if members.is_none() {
                    return Err(CE::struct_not_found(name.clone().unwrap(), span));
                }
                let members = members.unwrap().clone();
                Ok(BaseType::Struct(name.clone(), Some(members)))
            }
            _ => Ok(t),
        }
    }
}
