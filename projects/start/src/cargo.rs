use std::process::Command;

pub fn create_project(name: &str) -> Result<(), String> {
    let out = match Command::new("cargo")
        .args(["new", name])
        .output()
    {
        Ok(out) => out,
        Err(err) => return Err(format!("failed to run cargo: {err}")),
    };

    if out.status.success() {
        return Ok(());
    };

    return Err(format!(
        "cargo returned status [{}]: stderr = '{}'",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    ));
}
