load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "0c2ff9f58bbd6f2a4fc4fbea3a34e85fe848e7e4317357095551a18b2405a01c",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.25.0/rules_rust-v0.25.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.70.0"
    ],
)

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

rust_analyzer_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    packages = {
        "tokio": crate.spec(
            version = "1.29.0",
            features = ["full"]
        ),
        "hyper": crate.spec(
            version = "0.14.27",
            features = ["full"]
        ),
        "tower": crate.spec(
            version = "0.4.13",
        ),
        "axum": crate.spec(
            version = "0.6.18",
        )
    },
    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
