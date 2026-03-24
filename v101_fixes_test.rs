use glow::{check_source_at, format_source, run_source};
use glow::runtime::webhook::{load_tls_config, ServeConfig};
use std::fs;
use std::path::Path;

#[test]
fn try_catch_catches_throw_and_recovers() {
    let source = r#"
try
  throw "test error"
catch error
  say "Caught: " + error
"#;

    let output = run_source(source, Path::new(".")).expect("try/catch should run");
    assert!(output.contains("Caught: test error"));
}

#[test]
fn recover_returns_fallback_for_missing_tool() {
    let source = r#"
set value to read file "missing.txt" recover "fallback value"
say value
"#;

    let output = run_source(source, Path::new(".")).expect("recover should return fallback");
    assert_eq!(output.trim(), "fallback value");
}

#[test]
fn throw_without_try_returns_a_readable_error() {
    let source = r#"throw "boom""#;
    let diagnostics = run_source(source, Path::new(".")).expect_err("throw should fail");
    assert!(diagnostics.iter().any(|item| item.message.contains("boom")));
}

#[test]
fn formatter_round_trips_try_and_recover() {
    let source = r#"
try
  set value to get "http://bad.invalid" recover {status: "offline"}
catch error
  say error
"#;
    let formatted = format_source(source).expect("format should succeed");
    assert!(formatted.contains("try"));
    assert!(formatted.contains("recover {status: \"offline\"}"));
}

#[test]
fn https_tls_config_reports_missing_files_cleanly() {
    let config = ServeConfig::https(
        3443,
        Path::new("missing-cert.pem").to_path_buf(),
        Path::new("missing-key.pem").to_path_buf(),
    );
    let error = load_tls_config(&config).expect_err("missing certs should fail");
    assert!(error.message.contains("Could not read TLS certificate"));
}

#[test]
fn new_examples_typecheck() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    for file in ["error_recovery.glow", "https_webhook.glow", "robust_automation.glow"] {
        let path = root.join(file);
        let source = fs::read_to_string(&path).expect("example should exist");
        check_source_at(&source, &root).expect("example should typecheck");
    }
}

#[test]
fn error_recovery_example_runs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let source = fs::read_to_string(root.join("error_recovery.glow")).expect("example should exist");
    let output = run_source(&source, &root).expect("example should run");
    assert!(output.contains("Recovered customer"));
}
