use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn cli_prints_extreme_speeds() {
    let mut cmd = Command::cargo_bin("body-throwing-movement").unwrap();
    cmd.args([
        "--launch-height",
        "1.8",
        "--landing-height",
        "1.0",
        "--distance",
        "20",
        "--angle-min",
        "10",
        "--angle-max",
        "80",
        "--angle-step",
        "10",
    ]);

    cmd.assert()
        .success()
        .stdout(contains("Slowest launch"))
        .stdout(contains("Fastest launch"));
}
