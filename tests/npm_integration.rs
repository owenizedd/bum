use std::process::Command;

#[test]
#[ignore] // Run manually with --ignored flag (requires network)
fn test_npm_package_integration() {
    let project_root = std::env::current_dir().unwrap();

    // Check if we're in the project root
    if !project_root.join("bin.js").exists() {
        println!("Skipping - not in project root");
        return;
    }

    // Step 1: Build the native binding
    println!("🔨 Building native binding...");
    let build_output = Command::new("bun")
        .args(["run", "build"])
        .current_dir(&project_root)
        .output()
        .expect("Failed to run bun run build");

    assert!(
        build_output.status.success(),
        "Build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Step 2: Verify bin.js --version works
    println!("📦 Testing bin.js --version...");
    let version_output = Command::new("bun")
        .args(["bin.js", "--version"])
        .current_dir(&project_root)
        .output()
        .expect("Failed to run bin.js --version");

    assert!(
        version_output.status.success(),
        "bin.js --version failed: {}",
        String::from_utf8_lossy(&version_output.stderr)
    );

    let version = String::from_utf8_lossy(&version_output.stdout);
    println!("✅ Version: {}", version.trim());

    // Step 3: Test actual bum use command
    println!("📦 Testing bin.js use 1.3.3...");
    let use_output = Command::new("bun")
        .args(["bin.js", "use", "1.3.3"])
        .current_dir(&project_root)
        .output()
        .expect("Failed to run bin.js use 1.3.3");

    assert!(
        use_output.status.success(),
        "bin.js use 1.3.3 failed: {}",
        String::from_utf8_lossy(&use_output.stderr)
    );

    let use_stdout = String::from_utf8_lossy(&use_output.stdout);
    println!("{}", use_stdout);

    // Verify the output contains expected text
    assert!(
        use_stdout.contains("1.3.3") || use_stdout.contains("activated"),
        "Expected output to mention version activation"
    );

    println!("✅ npm package integration test passed!");
}
