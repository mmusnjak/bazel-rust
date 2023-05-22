load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "axum_app",
    srcs = ["main.rs"],
    deps = [
        "@crate_index//:tokio",
        "@crate_index//:axum",
        "//hello_lib:hello_lib"
    ],
)

# TODO: How can I cross compile, to build this binary for a docker image meant to run on linux?
