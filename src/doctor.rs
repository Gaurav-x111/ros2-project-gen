use crate::error::ProjectGenError;
use colored::Colorize;
use std::process::Command;
use which::which;

#[derive(Debug, Clone)]
pub struct DoctorCheck {
    pub name: String,
    pub found: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

impl DoctorCheck {
    fn new(name: &str, found: bool, version: Option<String>, path: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            found,
            version,
            path,
        }
    }
}

pub fn run_doctor() -> Result<Vec<DoctorCheck>, ProjectGenError> {
    let checks = vec![
        check_command("rustc", &["--version"]),
        check_command("cargo", &["--version"]),
        check_command("python3", &["--version"]),
        check_command("cmake", &["--version"]),
        check_command("colcon", &["--version"]),
        check_ros2(),
    ];

    print_doctor_results(&checks);

    Ok(checks)
}

/// Per-template dependency checks, run in addition to the base toolchain
/// checks. Returns `DoctorCheck` rows so `print_doctor_results` reuses the
/// same table format.
pub fn template_specific_checks(template: &str) -> Vec<DoctorCheck> {
    match template {
        "ai_perception" => vec![
            command_status_check("pkg-config --exists onnxruntime", "pkg-config", &["--exists", "onnxruntime"]),
            command_status_check("nvidia-smi", "nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"]),
            py_module_check("onnxruntime", "onnxruntime"),
        ],
        "matlab_bridge" => vec![
            env_check("MATLAB_ROOT"),
            command_status_check("matlab engine", "matlab", &["-batch", "disp('ok')"]),
        ],
        "visualization" => vec![
            command_status_check("rviz2", "rviz2", &["--version"]),
        ],
        "perception" | "minimal_ros2" => vec![
            py_module_check("cv_bridge", "cv_bridge"),
        ],
        _ => vec![],
    }
}

fn check_command(name: &str, version_args: &[&str]) -> DoctorCheck {
    match which(name) {
        Ok(path) => {
            let version = Command::new(name)
                .args(version_args)
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .and_then(|s| s.lines().next().map(|l| l.trim().to_string()));
            DoctorCheck::new(
                name,
                true,
                version,
                Some(path.to_string_lossy().to_string()),
            )
        }
        Err(_) => DoctorCheck::new(name, false, None, None),
    }
}

fn check_ros2() -> DoctorCheck {
    let ros_paths = [
        "/opt/ros/jazzy/bin/ros2",
        "/opt/ros/humble/bin/ros2",
        "/opt/ros/iron/bin/ros2",
    ];
    for path in ros_paths {
        if std::path::Path::new(path).exists() {
            let version = Command::new(path)
                .args(["--version"])
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .and_then(|s| s.lines().next().map(|l| l.trim().to_string()));
            return DoctorCheck::new("ros2", true, version, Some(path.to_string()));
        }
    }
    DoctorCheck::new("ros2", false, None, None)
}

/// Check whether a tool exists on PATH and exits with status 0.
fn command_status_check(label: &str, cmd: &str, args: &[&str]) -> DoctorCheck {
    match which(cmd) {
        Ok(path) => {
            let ok = Command::new(cmd)
                .args(args)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            DoctorCheck::new(label, ok, None, Some(path.to_string_lossy().to_string()))
        }
        Err(_) => DoctorCheck::new(label, false, None, None),
    }
}

/// Check whether an environment variable is set.
fn env_check(var: &str) -> DoctorCheck {
    let found = std::env::var(var).is_ok();
    let path = std::env::var(var).ok();
    DoctorCheck::new(&format!("${var}"), found, None, path)
}

/// Check whether a Python module imports successfully.
fn py_module_check(module: &str, label: &str) -> DoctorCheck {
    use std::process::Stdio;
    let found = Command::new("python3")
        .args(["-c", &format!("import {module}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    DoctorCheck::new(&format!("py:{label}"), found, None, None)
}

/// Render template-specific checks in the same table style as the base
/// toolchain output.
pub fn print_template_checks(template: &str) {
    let checks = template_specific_checks(template);
    if checks.is_empty() {
        println!("  (no template-specific requirements to check)");
        return;
    }
    println!("{}", "═".repeat(60));
    println!(
        "  {} {}",
        "Template-specific checks for".bold(),
        template.bold().green()
    );
    println!("{}", "═".repeat(60));
    println!();
    for check in &checks {
        let status = if check.found {
            "✓ FOUND".green().bold()
        } else {
            "✗ MISSING".red().bold()
        };
        let path = check
            .path
            .as_ref()
            .map(|p| format!(" at {}", p))
            .unwrap_or_default();
        println!(
            "  {:<12} {} {}",
            check.name,
            status,
            path.dimmed()
        );
    }
    let found_count = checks.iter().filter(|c| c.found).count();
    println!();
    println!(
        "  Summary: {}/{} required tools available",
        found_count,
        checks.len()
    );
    println!("{}", "═".repeat(60));
}

fn print_doctor_results(checks: &[DoctorCheck]) {
    println!("\n{}", "═".repeat(60));
    println!("{}", " ROS2 Project Generator - Environment Doctor ".bold());
    println!("{}", "═".repeat(60));
    println!();

    for check in checks {
        let status = if check.found {
            "✓ FOUND".green().bold()
        } else {
            "✗ MISSING".red().bold()
        };

        let version = check
            .version
            .as_ref()
            .map(|v| format!(" ({})", v))
            .unwrap_or_default();
        let path = check
            .path
            .as_ref()
            .map(|p| format!(" at {}", p))
            .unwrap_or_default();

        println!(
            "  {:<12} {} {}{}",
            check.name,
            status,
            version.dimmed(),
            path.dimmed()
        );
    }

    println!();
    let found_count = checks.iter().filter(|c| c.found).count();
    let total_count = checks.len();
    println!("  Summary: {}/{} tools available", found_count, total_count);
    println!("{}", "═".repeat(60));
}
