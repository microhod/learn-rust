use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use toml;

#[derive(Serialize)]
#[derive(Deserialize)]
struct CargoToml {
    workspace: CargoTomlWorkspace
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct CargoTomlWorkspace {
    resolver: String,
    members: Vec<String>,
}

pub fn add_to_cargo_toml(path: &str, member: &str) -> Result<(), String> {
    let current = match read_to_string(path) {
        Ok(out) => out,
        Err(err) => return Err(err.to_string())
    };
    let mut cargo = match toml::from_str::<CargoToml>(&current) {
        Ok(t) => t,
        Err(err) => return Err(err.to_string())
    };

    // add to members
    cargo.workspace.members.push(member.to_string());

    let updated = match toml::to_string_pretty(&cargo) {
        Ok(out) => out,
        Err(err) => return Err(err.to_string())
    };
    if let Err(err) = write(path, updated.as_str()) {
        return Err(err.to_string())
    };

    return Ok(());
}
