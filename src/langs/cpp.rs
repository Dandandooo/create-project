use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };
use std::fs::{ create_dir, write };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => {
            create_dir(name)?;
            name
        },
        None => "."
    };

    // Initialize default cpp project
    Command::new("cmake").args(["-S", name, "-B", &format!("{}/build", name)]).spawn()?;
    Command::new("cmake").arg("--build").arg(&format!("{}/build", name)).spawn()?;

    write(format!("{}/CMakeLists.txt", name), "cmake_minimum_required(VERSION 3.0)\nproject(my_project)\nadd_executable(my_project main.cpp)")?;

    write(format!("{}/main.cpp", name), "#include <iostream>\nint main() {\nstd::cout << \"Hello, World!\" << std::endl;\nreturn 0;\n}")?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
