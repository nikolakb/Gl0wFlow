use glow::{build_source, check_source_at, inspect_source_at, run_source};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!(
        "glow-mother-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be valid")
            .as_nanos()
    ))
}

fn copy_example(name: &str, target_dir: &Path) {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples").join(name);
    let target = target_dir.join(name);
    fs::copy(&source, &target).unwrap_or_else(|error| {
        panic!(
            "failed to copy example '{}' to '{}': {error}",
            source.display(),
            target.display()
        )
    });
}

#[test]
fn mother_of_all_proves_the_language_end_to_end() {
    let temp_dir = unique_temp_dir();
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    for name in ["mother_of_all.glow", "mother_shared.glow", "report.txt"] {
        copy_example(name, &temp_dir);
    }

    let source_path = temp_dir.join("mother_of_all.glow");
    let source = fs::read_to_string(&source_path).expect("mother test source should load");

    check_source_at(&source, &temp_dir).expect("mother flow should typecheck");

    let inspect_output =
        inspect_source_at(&source, &temp_dir).expect("mother flow should inspect successfully");
    assert!(
        inspect_output.contains("UseMcp") && inspect_output.contains("Task"),
        "inspect output should show major language features"
    );

    let interpreted =
        run_source(&source, &temp_dir).expect("mother flow should execute in the interpreter");
    assert!(
        interpreted.contains("Mother flow start")
            && interpreted.contains("Approved:")
            && interpreted.contains("Mother summary"),
        "interpreter output should cover the main workflow"
    );

    let build_dir = unique_temp_dir();
    fs::create_dir_all(&build_dir).expect("build dir should be created");
    let built_path = build_dir.join("mother_of_all-built");
    let built = build_source(&source, &source_path, Some(&built_path))
        .expect("mother flow should build to a native runner");
    let built_output = Command::new(&built)
        .output()
        .expect("built mother flow should run");
    assert!(built_output.status.success(), "built runner should succeed");
    let native = String::from_utf8_lossy(&built_output.stdout).trim().to_string();
    assert_eq!(native, interpreted.trim(), "native output should match interpreter output");

    for artifact in [
        "mother_record.json",
        "mother_rows.csv",
        "mother_reply.txt",
        "mother_summary.txt",
        "mother_note.txt",
    ] {
        assert!(
            temp_dir.join(artifact).exists(),
            "expected artifact '{}' to be created",
            artifact
        );
    }

    let _ = fs::remove_dir_all(&build_dir);
    let _ = fs::remove_dir_all(&temp_dir);
}
