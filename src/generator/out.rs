use crate::generator::Generator;
use anyhow::Result;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;
use std::path::PathBuf;

impl<'ctx> Generator<'ctx> {
    pub fn out_bc(&mut self) -> bool {
        let mut target_path = PathBuf::from(self.module_name);
        target_path.set_extension("bc");
        self.module.write_bitcode_to_path(target_path.as_path())
    }

    pub fn out_asm(&mut self) -> Result<()> {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let triple = TargetMachine::get_default_triple();

        let machine = Target::from_triple(&triple)
            .unwrap()
            .create_target_machine(
                &triple,
                &TargetMachine::get_host_cpu_name().to_string(),
                &TargetMachine::get_host_cpu_features().to_string(),
                OptimizationLevel::None,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        let mut target_path = PathBuf::from(self.module_name);
        target_path.set_extension("asm");
        machine
            .write_to_file(&self.module, FileType::Assembly, target_path.as_ref())
            .unwrap();

        return Ok(());
    }
}
