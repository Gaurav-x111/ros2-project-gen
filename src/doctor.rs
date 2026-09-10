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
