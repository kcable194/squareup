fn main() {
    // List the Cargo feature‑env vars corresponding to TLS features:
    let tls_vars = [
        "CARGO_FEATURE_DEFAULT_TLS",
        "CARGO_FEATURE_NATIVE_TLS",
        "CARGO_FEATURE_NATIVE_TLS_VENDORED",
        "CARGO_FEATURE_NATIVE_TLS_ALPN",
        "CARGO_FEATURE_RUSTLS_TLS",
    ];

    let enabled = tls_vars
        .iter()
        .filter(|&v| std::env::var(v).is_ok())
        .count();

    if enabled > 1 {
        panic!(
            "Multiple TLS features enabled ({}); please pick exactly one of: \
            default-tls, native-tls, native-tls-vendored, native-tls-alpn, \
            rustls-tls. Ensure that you have default-features = false when \
            using a TLS feature that is not default.",
            enabled
        );
    }
}
