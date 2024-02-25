use std::collections::{HashMap, HashSet};
use std::process::Command;
use crate::Res;


pub struct Dependency {
    pub name: String,
    pub install: Box<dyn Fn() -> Res>
}

fn dependencies() -> Res<HashMap<String, Dependency>> {
    let deps = [
        Dependency {
            name: "cargo".to_string(),
            install: Box::new(|| {
                Command::new("curl").args([
                    "--proto",
                    "=https",
                    "--tlsv1.2",
                    "-sSf",
                    "https://sh.rustup.rs",
                    "|",
                    "sh"
                ]).spawn();
                Ok(())
            }),
        }
    ];

    let mut out = HashMap::new();
    for dep in deps {
        out.insert(dep.name.clone(), dep);
    }
    Ok(out)
}
