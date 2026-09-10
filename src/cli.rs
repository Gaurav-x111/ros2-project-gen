use crate::doctor::{print_template_checks, run_doctor};
use crate::error::ProjectGenError;
use crate::generator::ProjectGenerator;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ros2-project-gen")]
#[command(about = "Scaffold ROS2 projects from templates", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new ROS2 project from a template
    Init {
        /// Project name (alphanumeric, hyphens, underscores only)
        name: String,
        /// Template to use (minimal, perception, ai_perception, minimal_ros2)
        #[arg(short, long, default_value = "minimal")]
        template: String,
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
        /// Optional features to enable (comma-separated, e.g. docker,ci)
        #[arg(short, long, value_delimiter = ',')]
        features: Vec<String>,
    },
    /// List available templates
    List,
    /// Check system dependencies (optionally per-template)
    Doctor {
        /// Also check dependencies required by this template
        /// (ai_perception, matlab_bridge, visualization, perception, minimal_ros2)
        #[arg(short, long)]
        template: Option<String>,
    },
}

use colored::Colorize;

fn detect_shell() -> &'static str {
    match std::env::var("SHELL") {
        Ok(shell) if shell.contains("zsh") => "zsh",
        _ => "bash",
    }
}

fn print_next_steps(project_name: &str, template: &str) {
    let shell = detect_shell();
    let setup_file = if shell == "zsh" { "setup.zsh" } else { "setup.bash" };
    let install_cmd = format!("source install/{}", setup_file);

    println!();
    println!("{}", "═".repeat(60));
    println!(
        "  {} {}",
        "Next steps for".bold(),
        template.bold().green()
    );
    println!("{}", "═".repeat(60));

    match template {
        "minimal" => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{}", setup_file);
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     {}", install_cmd);
            println!();
            println!("  4. Launch the bringup:");
            println!(
                "     ros2 launch {}_bringup bringup.launch.py",
                project_name
            );
            println!();
            println!("  5. Run tests:");
            println!(
                "     colcon test --packages-select {}_bringup",
                project_name
            );
            println!(
                "     colcon test-result --verbose"
            );
        }
        "perception" => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
            println!();
            println!("  4. Launch the perception pipeline:");
            println!(
                "     ros2 launch {}_bringup bringup.launch.py",
                project_name
            );
            println!();
            println!("  5. Check running nodes:");
            println!("     ros2 node list");
            println!("     ros2 topic list");
        }
        "ai_perception" => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
            println!();
            println!("  4. Place your ONNX model:");
            println!("     cp yolov8n.onnx {}/ros2_ws/src/{}/ml_inference/models/", project_name, project_name);
            println!();
            println!("  5. Launch the AI perception stack:");
            println!(
                "     ros2 launch {}_bringup ai_perception.launch.py",
                project_name
            );
            println!();
            println!("  6. Subscribe to detections:");
            println!("     ros2 topic echo /{}/detections", project_name);
        }
        "minimal_ros2" => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace (C++ and Python packages):");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
            println!();
            println!("  4. Launch all nodes (C++ + Python + bringup):");
            println!(
                "     ros2 launch {}_bringup bringup.launch.py",
                project_name
            );
            println!();
            println!("  5. Test individual nodes:");
            println!("     ros2 run {}_cpp talker", project_name);
            println!("     ros2 run {}_python talker", project_name);
        }
        "matlab_bridge" => {
            println!();
            println!("  === ROS 2 Side ===");
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
            println!();
            println!("  4. Launch the MATLAB bridge node:");
            println!(
                "     ros2 launch {}_bringup matlab_bridge.launch.py",
                project_name
            );
            println!();
            println!("  === MATLAB Side ===");
            println!();
            println!("  5. Open MATLAB and install ROS 2 Toolbox:");
            println!("     >> ver ROS2");
            println!();
            println!("  6. Connect to ROS 2 network from MATLAB:");
            println!("     >> rosinit");
            println!();
            println!("  7. Subscribe to data from ROS 2:");
            println!("     >> poseSub = rossubscriber('/{}/robot/pose');", project_name);
            println!("     >> cameraSub = rossubscriber('/{}/camera/image_raw');", project_name);
            println!();
            println!("  8. Send commands from MATLAB to ROS 2:");
            println!(
                "     >> [pub, msg] = rospublisher('/{}/cmd_vel', 'geometry_msgs/TwistStamped');",
                project_name
            );
            println!("     >> msg.Twist.Linear.X = 0.5;");
            println!("     >> send(pub, msg);");
            println!();
            println!("  9. For Simulink co-simulation:");
            println!("     >> open_system('{}_model');", project_name);
            println!("     >> set_param('{}_model', 'SimulationMode', 'Normal');", project_name);
            println!("     >> sim('{}_model');", project_name);
            println!();
            println!("  Topics available:");
            println!("     /{}/camera/image_raw    (ROS2 -> MATLAB)", project_name);
            println!("     /{}/lidar/points         (ROS2 -> MATLAB)", project_name);
            println!("     /{}/robot/pose           (ROS2 -> MATLAB)", project_name);
            println!("     /{}/cmd_vel              (MATLAB -> ROS2)", project_name);
            println!("     /{}/trajectory/waypoint  (MATLAB -> ROS2)", project_name);
        }
        "visualization" => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
            println!();
            println!("  4. Launch visualization (markers + TF + RViz):");
            println!(
                "     ros2 launch {}_bringup visualization.launch.py",
                project_name
            );
            println!();
            println!("  This opens RViz2 with:");
            println!("    - Grid overlay on the ground plane");
            println!("    - TF frames (map -> odom -> base_link -> laser/camera)");
            println!("    - Markers: cubes, arrows, path trail, text labels");
            println!();
            println!("  5. Or run individual nodes:");
            println!(
                "     ros2 run {}_visualization_node marker_publisher",
                project_name
            );
            println!(
                "     ros2 run {}_visualization_node tf_broadcaster",
                project_name
            );
            println!();
            println!("  6. Subscribe to marker topic from your own node:");
            println!(
                "     ros2 topic echo /{}/visualization/markers",
                project_name
            );
            println!();
            println!("  7. Add custom markers programmatically:");
            println!("     Edit: ros2_ws/src/{name}/visualization_node/marker_publisher.py",
                name = project_name);
        }
        _ => {
            println!();
            println!("  1. Source ROS 2 environment:");
            println!("     source /opt/ros/jazzy/{setup_file}");
            println!();
            println!("  2. Build the workspace:");
            println!("     cd {}/ros2_ws && colcon build", project_name);
            println!();
            println!("  3. Source the workspace:");
            println!("     source install/{setup_file}");
        }
    }
    println!();
    println!("{}", "═".repeat(60));
}

impl Cli {
    pub fn execute(&self) -> Result<(), ProjectGenError> {
        match &self.command {
            Commands::Init {
                name,
                template,
                output,
                features,
            } => {
                let generator = ProjectGenerator::new()?;

                // Validate the full input path first (before extracting basename)
                if name.contains("..") {
                    return Err(ProjectGenError::PathTraversal(name.clone()));
                }

                // If name is a path, extract the basename and use parent as output
                let (project_name, actual_output) = if name.contains('/') || name.contains('\\') {
                    let path = std::path::Path::new(name);
                    let project_name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| ProjectGenError::InvalidName(name.clone()))?;
                    let parent = path.parent().unwrap_or(std::path::Path::new("."));
                    (project_name, parent)
                } else {
                    (name.as_str(), output.as_path())
                };

                let project_path = generator.generate(project_name, template, actual_output)?;
                println!(
                    "Project '{}' created at: {}",
                    project_name,
                    project_path.display()
                );
                if !features.is_empty() {
                    println!("Enabled features: {}", features.join(", "));
                }
                print_next_steps(project_name, template);
            }
            Commands::List => {
                let generator = ProjectGenerator::new()?;
                println!("Available templates:");
                for meta in generator.list_templates() {
                    let ver = if meta.version.is_empty() {
                        String::new()
                    } else {
                        format!(" (v{})", meta.version)
                    };
                    println!("  {:<15} {}{}", meta.name, meta.description, ver);
                    if !meta.required_features.is_empty() {
                        println!(
                            "                  Required: {}",
                            meta.required_features.join(", ")
                        );
                    }
                    if !meta.optional_features.is_empty() {
                        println!(
                            "                  Optional: {}",
                            meta.optional_features.join(", ")
                        );
                    }
                }
            }
            Commands::Doctor { template } => {
                run_doctor()?;
                if let Some(t) = template {
                    println!();
                    print_template_checks(t);
                }
            }
        }
        Ok(())
    }
}