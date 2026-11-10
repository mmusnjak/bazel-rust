load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test", "rust_test_suite")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "hello_lib",
    srcs = [
        "src/lib.rs",
        "src/uuidv7.rs",
    ],
    deps = [
        "@crate_index//:base32",
        "@crate_index//:hmac",
        "@crate_index//:sha2",
        "@crate_index//:uuid",
    ],
)

rust_test(
    name = "hello_lib_unit_test",
    crate = ":hello_lib",
)

rust_test_suite(
    name = "all",
    srcs = glob(["tests/**"]),
    deps = [
        ":hello_lib",
        "@crate_index//:base32",
        "@crate_index//:uuid",
    ],
)
