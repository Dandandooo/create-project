use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

use std::fs::{File, create_dir, write};

pub fn init(config: &CommandConfig) -> Res {


    let sample_code = b"#include <stdio.h>\nint main() {\n    // printf() displays the string inside quotation\n    printf(\"Hello, World!\")\n    return 0;\n}";

    let dir = match config.vars.get("name") {
        Some(name) => {
            create_dir(name)?;
            name
        },
        None => "."
    };

    // Create directories
    create_dir(format!("{}/src", dir))?;
    create_dir(format!("{}/include", dir))?;
    create_dir(format!("{}/lib", dir))?;
    create_dir(format!("{}/bin", dir))?;
    create_dir(format!("{}/obj", dir))?;
    create_dir(format!("{}/test", dir))?;

    write(format!("{}/src/main.c", dir), sample_code)?;

    write(format!("{}/Makefile", dir), b"CC=gcc\nCFLAGS=-Iinclude\nLDFLAGS=-Llib\n\nall: main\n\nmain: obj/main.o\n\t$(CC) -o bin/main obj/main.o $(LDFLAGS)\n\nobj/main.o: src/main.c\n\t$(CC) -c -o obj/main.o src/main.c $(CFLAGS)\n\nclean:\n\trm -f bin/* obj/*\n")?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
