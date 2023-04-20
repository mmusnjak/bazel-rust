load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test_suite")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "hello_lib",
    srcs = ["src/lib.rs"],
)

rust_test_suite(
    name = "tests",
    srcs = glob(["tests/**"]),
    deps = [":hello_lib"],
)
