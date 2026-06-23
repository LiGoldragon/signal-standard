use std::path::PathBuf;

use schema_rust::build::{CargoSchemaMetadata, GenerationDriver, GenerationPlan, ModuleEmission};

/// The signal-standard build: lowers `schema/lib.schema` to `src/schema/lib.rs`
/// through the DECLARATION-MODULE emission target — pure cross-component
/// vocabulary, no operation roots, no wire codec, no daemon runtime. This is
/// the shape a shared standards library needs; the wire-contract crates use
/// `ContractCrateBuild` instead, which emits the frame codec over their roots.
struct StandardCrateBuild {
    crate_root: PathBuf,
    crate_name: String,
    schema_version: String,
    module: String,
    update_environment_variable: String,
}

impl StandardCrateBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(
                std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set"),
            ),
            crate_name: "signal-standard".to_owned(),
            schema_version: "0.1.0".to_owned(),
            module: "lib".to_owned(),
            update_environment_variable: "SIGNAL_STANDARD_UPDATE_SCHEMA_ARTIFACTS".to_owned(),
        }
    }

    fn generation_plan(&self) -> GenerationPlan {
        GenerationPlan::new(&self.crate_root, &self.crate_name, &self.schema_version)
            .with_module(ModuleEmission::declaration_module(&self.module))
    }

    fn print_cargo_directives(&self) {
        println!("cargo:rerun-if-changed=schema/{}.schema", self.module);
        println!("cargo:rerun-if-changed=src/schema/{}.rs", self.module);
        CargoSchemaMetadata::new(&self.crate_name).emit_schema_directory(&self.crate_root);
    }

    fn run(&self) {
        self.print_cargo_directives();
        GenerationDriver::new(self.generation_plan())
            .generate()
            .expect("lower signal-standard schema")
            .write_or_check(&self.update_environment_variable)
            .expect("checked-in declaration schema artifacts are fresh");
    }
}

fn main() {
    StandardCrateBuild::from_environment().run();
}
