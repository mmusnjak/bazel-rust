load("@rules_rust//rust:defs.bzl", "rust_binary")

# TODO: Is this the best way to build debug vs prod targets?
[
rust_binary(
    name = "axum_app" + dbg,
    srcs = ["main.rs"],
    deps = [
        "@crate_index//:tokio",
        "@crate_index//:axum",
        "//hello_lib:hello_lib"
    ],
)
 for dbg in ["_dbg", ""] ]

# TODO: How can I cross compile, to build this binary for a docker image meant to run on linux?
