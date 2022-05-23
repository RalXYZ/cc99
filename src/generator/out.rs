use crate::generator::Generator;
use anyhow::Result;
use inkwell::passes::{PassManager, PassManagerBuilder};
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
    pub fn out_asm_or_obj(
        &mut self,
        is_obj: bool,
        output_file: Option<String>,
        opt: OptimizationLevel,
    ) -> Result<()> {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let triple = TargetMachine::get_default_triple();
        let cpu = match opt {
            OptimizationLevel::None => std::env::consts::ARCH.to_string().replace("_", "-"),
            _ => TargetMachine::get_host_cpu_name().to_string(),
        };
        let features = match opt {
            OptimizationLevel::None => "".to_string(),
            _ => TargetMachine::get_host_cpu_features().to_string(),
        };
        let pass_manager = match opt {
            OptimizationLevel::None => {
                let fpm = PassManager::create(());
                let pb = PassManagerBuilder::create();
                pb.set_optimization_level(opt);
                pb.set_disable_unroll_loops(true);
                pb.populate_module_pass_manager(&fpm);
                fpm
            }
            _ => {
                let fpm = PassManager::create(());
                let pb = PassManagerBuilder::create();
                pb.set_optimization_level(opt);
                pb.set_disable_unroll_loops(false);
                pb.populate_module_pass_manager(&fpm);
                fpm
            }
        };

        let machine = Target::from_triple(&triple)
            .unwrap()
            .create_target_machine(
                &triple,
                &cpu,
                &features,
                opt,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();
        machine.add_analysis_passes(&pass_manager);

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
