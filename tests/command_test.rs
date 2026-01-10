use std::path::PathBuf;

use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn runs() {
    let mut cmd = cargo_bin_cmd!("google_authenticator_extractor");

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/fixtures/migration.jpg");
    let image_path = d.to_str().unwrap();

    cmd.arg("-i").arg(image_path).assert()
        .success()
        .stdout(r#"[{"name":"Example:example@example.local","secret":"43AHZP6DT7U4CM5PAJRT652BBEHXXPT72XJVFBISFVEZ3MFB33MQBCPR","issuer":""}]
"#);
}
