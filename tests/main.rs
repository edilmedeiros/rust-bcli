use assert_cmd::Command;

#[test]
fn help() {
    Command::cargo_bin("bitcoin")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}
