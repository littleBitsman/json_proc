use std::env;

use rustc_version::Channel;

fn main() {
    export_var("PROFILE", &env::var("PROFILE").unwrap());
    println!(r#"cargo::rustc-check-cfg=cfg(profile, values("dev","release"))"#);
    println!(
        r#"cargo::rustc-cfg=profile="{}""#,
        env::var("PROFILE").unwrap()
    );
    println!(r#"cargo::rustc-check-cfg=cfg(compiler, values("dev","nightly","beta","stable"))"#);
    let compiler = rustc_version::version_meta()
        .expect("Failed to get rustc version")
        .channel;
    println!(
        r#"cargo::rustc-cfg=compiler="{}""#,
        match compiler {
            Channel::Dev => "dev",
            Channel::Nightly => "nightly",
            Channel::Beta => "beta",
            Channel::Stable => "stable",
        }
    );
    println!(r#"cargo::rustc-check-cfg=cfg(lints_enabled)"#);
    if compiler == Channel::Nightly {
        println!(r#"cargo::rustc-cfg=lints_enabled"#)
    }
}

/// this is stolen from some git repo
fn export_var(name: &str, value: &str) {
    println!("cargo:rustc-env={}={}", name, value);
}
