
use std::env::args;
use create_project::*;
use args::*;
use langs::*;

fn main() {
    let program = run();
    if let Err(err) = program {
        eprint!("{err}");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn run() -> Result<(), String> {

    let global_args = global_args();
    let langs = supported_languages();
    let cmd = Command::parse(args())?;
    cmd.exec(&global_args, &langs)?;

    Ok(())
}
