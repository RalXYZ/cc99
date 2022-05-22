use crate::generator::Generator;
use anyhow::Result;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;
use std::path::PathBuf;

impl<'ctx> Generator<'ctx> {
    /// * `output_file` - None if use default filename
    pub fn out_bc(&mut self, output_file: Option<String>) -> bool {
        let mut target_path;
        if let Some(output_file) = output_file {
            target_path = PathBuf::from(output_file);
        } else {
            target_path = PathBuf::from(self.module_name);
            target_path.set_extension("bc");
        }
        self.module.write_bitcode_to_path(target_path.as_path())
    }

    /// * `is_obj` - true if the output file is an object file, false if it is a asm file
    /// * `output_file` - None if use default filename
    pub fn out_asm_or_obj(&mut self, is_obj: bool, output_file: Option<String>, opt: OptimizationLevel) -> Result<()> {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let triple = TargetMachine::get_default_triple();
        let machine = Target::from_triple(&triple)
            .unwrap()
            .create_target_machine(
                &triple,
                &TargetMachine::get_host_cpu_name().to_string(),
                &TargetMachine::get_host_cpu_features().to_string(),
                opt,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        let mut target_path;
        if let Some(output_file) = output_file {
            target_path = PathBuf::from(output_file);
        } else {
            target_path = PathBuf::from(self.module_name);
            target_path.set_extension(match is_obj {
                true => "o",
                false => "s",
            });
        }

        machine
            .write_to_file(
                &self.module,
                match is_obj {
                    true => FileType::Object,
                    false => FileType::Assembly,
                },
                target_path.as_ref(),
            )
            .unwrap();

        return Ok(());
    }
}
