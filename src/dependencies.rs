use std::collections::{HashMap, HashSet};
use std::process::{Command, Stdio};
use crate::Res;


pub struct Dependency {
    pub name: String,
    pub install: InstallType,
}

pub enum InstallType {
    UPT,
    Custom(Box<dyn Fn() -> Res>) 
}

pub fn dependencies() -> Res<HashMap<String, Dependency>> {
    let deps = [
        Dependency {
            name: "cargo".to_string(),
            install: InstallType:: Custom (Box::new(|| {
                Command::new("curl").args([
                    "--proto",
                    "=https",
                    "--tlsv1.2",
                    "-sSf",
                    "https://sh.rustup.rs",
                    "|",
                    "sh"
                ]).output()?;
                Ok(())
            })),
        },

        Dependency {
            name: "upt".to_string(),
            install: InstallType:: Custom (Box::new(|| {
                Command::new("cargo").args([
                    "install",
                    "upt",
                ]).output()?;
                Ok(())
            })),
        },

        Dependency {
            name: "git".to_string(),
            install: InstallType::UPT,
        },
    ];

    let mut out = HashMap::new();
    for dep in deps {
        out.insert(dep.name.clone(), dep);
    }
    Ok(out)
}

pub fn ensure_installed<'a, I>(ensure: I, supported_deps: &HashMap<String, Dependency>) -> Res
where
I: Iterator<Item = &'a String>
{
    for dep_name in ensure {
        if let Ok(_) = Command::new("which").args([&dep_name]).stdin(Stdio::null()).output() {
            continue;
        }

        let dep = match supported_deps.get(dep_name) {
            Some(d) => d,
            None => return Err(Box::from(format!("dependency {} not supported", &dep_name)))
        };

        match &dep.install {
            InstallType::UPT =>  match  Command::new("sudo").args(["upt", "install", &dep_name]).spawn() {
                Ok(_) => Ok(()),
                Err(e) => return Err(Box::new(e)),
            },
            InstallType::Custom(i) => i(),
        }?
    }
    Ok(())
}
