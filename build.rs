use std::process::Command;
fn main() {
    // Only build CSS in debug mode (during development)
    if cfg!(debug_assertions) {
        let output = Command::new("npm").args(&["run", "build-css"]).output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        "cargo:warning=Failed to build CSS: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run npm: {}", e);
            }
        }
    }

    println!("cargo:rerun-if-changed=src/input.css");
    println!("cargo:rerun-if-changed=templates/");
}
