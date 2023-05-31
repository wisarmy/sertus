pub fn default() -> &'static str {
    let v = format!(
        r#"{}
        Git: {} + {}
        Target: {}
        Features: {}
        Build: {}
        Rustc: {}"#,
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_BRANCH"),
        env!("VERGEN_GIT_DESCRIBE"),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("VERGEN_CARGO_FEATURES"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_RUSTC_SEMVER"),
    );
    Box::leak(v.into_boxed_str())
}
