use crate::generator::Generator;

impl<'ctx> Generator<'ctx> {
    pub(crate) fn no_terminator(&self) -> bool {
        let block = self.builder.get_insert_block();
        let terminator = block.unwrap().get_terminator();
        return terminator.is_none();
    }
}