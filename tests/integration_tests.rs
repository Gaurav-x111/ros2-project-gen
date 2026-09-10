use std::fs;
use tempfile::TempDir;

const BINARY: &str = env!("CARGO_BIN_EXE_ros2-project-gen");

fn run_cli(args: &[&str]) -> (bool, String, String) {
    let output = std::process::Command::new(BINARY)
        .args(args)
        .output()
        .expect("Failed to execute binary");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

#[test]
fn test_help_works() {
    let (success, stdout, stderr) = run_cli(&["--help"]);
    assert!(success, "Help should succeed: {}", stderr);
    assert!(stdout.contains("ros2-project-gen"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("doctor"));
}

#[test]
fn test_list_works() {
    let (success, stdout, stderr) = run_cli(&["list"]);
    assert!(success, "List should succeed: {}", stderr);
    assert!(stdout.contains("minimal"));
    assert!(stdout.contains("perception"));
    assert!(stdout.contains("ai_perception"));
    assert!(stdout.contains("minimal_ros2"));
}

#[test]
fn test_list_shows_descriptions() {
    let (success, stdout, stderr) = run_cli(&["list"]);
    assert!(success, "List should succeed: {}", stderr);
    assert!(
        stdout.contains("Minimal ROS2 workspace with bringup package"),
        "Should show minimal description"
    );
    assert!(
        stdout.contains("Perception pipeline with Python ROS2 node"),
        "Should show perception description"
    );
    assert!(
        stdout.contains("AI perception stack with ML inference"),
        "Should show ai_perception description"
    );
    assert!(
        stdout.contains("Rust/Python/C++ nodes"),
        "Should show minimal_ros2 description"
    );
}

#[test]
fn test_list_shows_features() {
    let (success, stdout, stderr) = run_cli(&["list"]);
    assert!(success, "List should succeed: {}", stderr);
    assert!(stdout.contains("Optional:"), "Should show optional features");
    assert!(stdout.contains("docker"), "Should list docker feature");
    assert!(stdout.contains("Required:"), "Should show required features");
}

#[test]
fn test_init_minimal_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_robot");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "Init minimal failed: {}", stderr);

    // Check directory structure
    assert!(project_path.exists(), "Project directory should exist");
    assert!(
        project_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        project_path.join("LICENSE").exists(),
        "LICENSE should exist"
    );
    assert!(
        project_path.join(".gitignore").exists(),
        ".gitignore should exist"
    );
    assert!(
        project_path.join("ros2_ws").exists(),
        "ros2_ws should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/test_robot_bringup").exists(),
        "bringup package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_robot_bringup/package.xml")
            .exists(),
        "bringup package.xml should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_robot_bringup/CMakeLists.txt")
            .exists(),
        "bringup CMakeLists.txt should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_robot_bringup/launch/bringup.launch.py")
            .exists(),
        "launch file should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_robot_bringup/config/params.yaml")
            .exists(),
        "config should exist"
    );
    assert!(
        project_path.join("tests").exists(),
        "tests dir should exist"
    );
    assert!(project_path.join("docs").exists(), "docs dir should exist");
    assert!(
        project_path.join(".github/workflows/ci.yml").exists(),
        "CI workflow should exist"
    );
}

#[test]
fn test_init_perception_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_perception");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "perception",
    ]);
    assert!(success, "Init perception failed: {}", stderr);

    // Check directory structure
    assert!(project_path.exists(), "Project directory should exist");
    assert!(
        project_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_perception_bringup")
            .exists(),
        "bringup package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_perception_perception_node")
            .exists(),
        "perception_node package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_perception_perception_node/perception_node/__init__.py")
            .exists(),
        "__init__.py should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_perception_perception_node/perception_node/perception_node.py")
            .exists(),
        "perception_node.py should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_perception_perception_node/setup.py")
            .exists(),
        "setup.py should exist"
    );
}

#[test]
fn test_init_ai_perception_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_ai");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "ai_perception",
    ]);
    assert!(success, "Init ai_perception failed: {}", stderr);

    // Check directory structure
    assert!(project_path.exists(), "Project directory should exist");
    assert!(
        project_path.join("ros2_ws").exists(),
        "ros2_ws should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/bringup").exists(),
        "bringup package should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/bringup/CMakeLists.txt").exists(),
        "bringup CMakeLists.txt should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/bringup/launch/ai_perception.launch.py")
            .exists(),
        "launch file should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/bringup/config/ai_perception.yaml")
            .exists(),
        "config should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/ml_inference").exists(),
        "ml_inference package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/ml_inference/Cargo.toml")
            .exists(),
        "ml_inference Cargo.toml should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/ml_inference/src/main.rs")
            .exists(),
        "ml_inference main.rs should exist"
    );

    // No .tera files in output
    for entry in walkdir::WalkDir::new(&project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        assert!(
            !entry.path().to_string_lossy().ends_with(".tera"),
            "No .tera files should be in output: {}",
            entry.path().display()
        );
    }
}

#[test]
fn test_init_minimal_ros2_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_minimal_ros2");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal_ros2",
    ]);
    assert!(success, "Init minimal_ros2 failed: {}", stderr);

    // Check directory structure
    assert!(project_path.exists(), "Project directory should exist");
    assert!(
        project_path.join("ros2_ws").exists(),
        "ros2_ws should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/test_minimal_ros2_bringup").exists(),
        "bringup package should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/cpp_node").exists(),
        "cpp_node should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/cpp_node/src/talker.cpp").exists(),
        "cpp_node talker.cpp should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/cpp_node/src/listener.cpp").exists(),
        "cpp_node listener.cpp should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/python_node").exists(),
        "python_node should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/python_node/python_node_src/__init__.py")
            .exists(),
        "python_node __init__.py should exist"
    );
    assert!(
        project_path.join("ros2_ws/src/rust_node").exists(),
        "rust_node should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/rust_node/Cargo.toml")
            .exists(),
        "rust_node Cargo.toml should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/rust_node/src/main.rs")
            .exists(),
        "rust_node main.rs should exist"
    );

    // No .tera files in output
    for entry in walkdir::WalkDir::new(&project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        assert!(
            !entry.path().to_string_lossy().ends_with(".tera"),
            "No .tera files should be in output: {}",
            entry.path().display()
        );
    }
}

#[test]
fn test_readme_contains_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("my_awesome_robot");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "Init failed: {}", stderr);

    let readme_content = fs::read_to_string(project_path.join("README.md")).unwrap();
    assert!(
        readme_content.contains("my_awesome_robot"),
        "README should contain project name"
    );
    assert!(
        readme_content.contains("MY_AWESOME_ROBOT"),
        "README should contain uppercase project name"
    );
}

#[test]
fn test_invalid_project_names_rejected() {
    let _temp_dir = TempDir::new().unwrap();

    // Test empty name
    let (success, _, _) = run_cli(&["init", "", "--template", "minimal"]);
    assert!(!success, "Empty name should be rejected");

    // Test path traversal
    let (success, _, _) = run_cli(&["init", "../bad", "--template", "minimal"]);
    assert!(!success, "Path traversal should be rejected");

    // Test slashes
    let (success, _, _) = run_cli(&["init", "bad/name", "--template", "minimal"]);
    assert!(!success, "Slash in name should be rejected");

    // Test spaces
    let (success, _, _) = run_cli(&["init", "bad name", "--template", "minimal"]);
    assert!(!success, "Spaces in name should be rejected");
}

#[test]
fn test_existing_non_empty_directory_not_overwritten() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("existing_project");

    // Create directory with content
    fs::create_dir_all(&project_path).unwrap();
    fs::write(project_path.join("existing_file.txt"), "content").unwrap();

    let (success, _, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(
        !success,
        "Should not overwrite non-empty directory: {}",
        stderr
    );
    assert!(stderr.contains("not empty"));
}

#[test]
fn test_missing_ros2_does_not_crash_doctor() {
    let (success, stdout, stderr) = run_cli(&["doctor"]);
    assert!(
        success,
        "Doctor should succeed even without ROS2: {}",
        stderr
    );
    assert!(stdout.contains("rustc"));
    assert!(stdout.contains("cargo"));
    assert!(stdout.contains("python3"));
    assert!(stdout.contains("cmake"));
    assert!(stdout.contains("colcon"));
    assert!(stdout.contains("ros2"));
}

#[test]
fn test_template_selection_works() {
    let temp_dir = TempDir::new().unwrap();

    // Test minimal template
    let (success, _, stderr) = run_cli(&[
        "init",
        temp_dir.path().join("t1").to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "Minimal template failed: {}", stderr);
    assert!(temp_dir.path().join("t1/ros2_ws/src/t1_bringup").exists());

    // Test perception template
    let (success, _, stderr) = run_cli(&[
        "init",
        temp_dir.path().join("t2").to_str().unwrap(),
        "--template",
        "perception",
    ]);
    assert!(success, "Perception template failed: {}", stderr);
    assert!(temp_dir
        .path()
        .join("t2/ros2_ws/src/t2_perception_node")
        .exists());
}

#[test]
fn test_generated_files_non_empty() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_files");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "Init failed: {}", stderr);

    // Check all generated files are non-empty
    for entry in walkdir::WalkDir::new(&project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let metadata = entry.metadata().unwrap();
            assert!(
                metadata.len() > 0,
                "File should not be empty: {}",
                entry.path().display()
            );
        }
    }
}

#[test]
fn test_second_init_attempt_behaves_correctly() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("second_attempt");

    // First init
    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "First init failed: {}", stderr);

    // Second init should fail
    let (success, _, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(!success, "Second init should fail: {}", stderr);
    assert!(stderr.contains("not empty"));
}

#[test]
fn test_features_flag_shown_in_output() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_features");

    let (success, stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
        "--features",
        "docker,ci",
    ]);
    assert!(success, "Init with features failed: {}", stderr);
    assert!(
        stdout.contains("docker, ci"),
        "Should display enabled features"
    );
}

#[test]
fn test_no_template_toml_in_output() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_no_meta");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "minimal",
    ]);
    assert!(success, "Init failed: {}", stderr);

    // template.toml should NOT be in output
    assert!(
        !project_path.join("template.toml").exists(),
        "template.toml should not be copied to output"
    );
}

#[test]
fn test_init_matlab_bridge_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_matlab");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "matlab_bridge",
    ]);
    assert!(success, "Init matlab_bridge failed: {}", stderr);

    assert!(project_path.join("ros2_ws").exists(), "ros2_ws should exist");
    assert!(
        project_path
            .join("ros2_ws/src/test_matlab_matlab_bridge")
            .exists(),
        "matlab_bridge package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_matlab_matlab_bridge/src/matlab_bridge_node.cpp")
            .exists(),
        "matlab_bridge_node.cpp should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_matlab_bringup")
            .exists(),
        "bringup package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_matlab_bringup/launch/matlab_bridge.launch.py")
            .exists(),
        "launch file should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_matlab_bringup/config/bridge_config.yaml")
            .exists(),
        "config file should exist"
    );
    assert!(
        project_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        project_path.join("LICENSE").exists(),
        "LICENSE should exist"
    );
}

#[test]
fn test_init_visualization_creates_expected_tree() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_viz");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "visualization",
    ]);
    assert!(success, "Init visualization failed: {}", stderr);

    assert!(project_path.join("ros2_ws").exists(), "ros2_ws should exist");
    assert!(
        project_path
            .join("ros2_ws/src/visualization_node")
            .exists(),
        "visualization_node package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/visualization_node/visualization_node/marker_publisher.py")
            .exists(),
        "marker_publisher.py should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/visualization_node/visualization_node/tf_broadcaster.py")
            .exists(),
        "tf_broadcaster.py should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_viz_bringup")
            .exists(),
        "bringup package should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/src/test_viz_bringup/launch/visualization.launch.py")
            .exists(),
        "launch file should exist"
    );
    assert!(
        project_path
            .join("ros2_ws/rviz/test_viz_view.rviz")
            .exists(),
        "RViz config should exist"
    );
    assert!(
        project_path.join("README.md").exists(),
        "README.md should exist"
    );
}

#[test]
fn test_visualization_rviz_config_contains_name() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_viz_content");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "visualization",
    ]);
    assert!(success, "Init visualization failed: {}", stderr);

    let rviz_content = fs::read_to_string(
        project_path.join("ros2_ws/rviz/test_viz_content_view.rviz"),
    )
    .unwrap();
    assert!(
        rviz_content.contains("test_viz_content"),
        "RViz config should contain project name"
    );
    assert!(
        rviz_content.contains("visualization/markers"),
        "RViz config should reference markers topic"
    );
}

#[test]
fn test_matlab_bridge_cpp_substituted() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_matlab_sub");

    let (success, _stdout, stderr) = run_cli(&[
        "init",
        project_path.to_str().unwrap(),
        "--template",
        "matlab_bridge",
    ]);
    assert!(success, "Init matlab_bridge failed: {}", stderr);

    let cpp_content = fs::read_to_string(
        project_path.join("ros2_ws/src/test_matlab_sub_matlab_bridge/src/matlab_bridge_node.cpp"),
    )
    .unwrap();
    assert!(
        cpp_content.contains("test_matlab_sub"),
        "C++ should contain project name"
    );
    assert!(
        !cpp_content.contains("{{ name }}"),
        "C++ should have no placeholders"
    );
}

#[test]
fn test_all_templates_generate_non_empty_files() {
    let temp_dir = TempDir::new().unwrap();

    for template in [
        "minimal",
        "perception",
        "ai_perception",
        "minimal_ros2",
        "matlab_bridge",
        "visualization",
    ] {
        let project_path = temp_dir.path().join(format!("test_{}", template));
        let (success, _stdout, stderr) = run_cli(&[
            "init",
            project_path.to_str().unwrap(),
            "--template",
            template,
        ]);
        assert!(
            success,
            "Init {} failed: {}",
            template,
            stderr
        );

        for entry in walkdir::WalkDir::new(&project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let metadata = entry.metadata().unwrap();
                assert!(
                    metadata.len() > 0,
                    "{}: File should not be empty: {}",
                    template,
                    entry.path().display()
                );
                assert!(
                    !entry.path().to_string_lossy().ends_with(".tera"),
                    "{}: No .tera files should be in output: {}",
                    template,
                    entry.path().display()
                );
            }
        }
    }
}