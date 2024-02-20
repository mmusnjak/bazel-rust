# Rust and Bazel

## Tasks

* Run Hello World: `bazel run //hello_world`

* Run Hello World Axum app: `bazel run //axum_app`

* Format: `bazel run @rules_rust//:rustfmt`

* Check formatting: `bazel build --aspects=@rules_rust//rust:defs.bzl%rustfmt_aspect --output_groups=rustfmt_checks //...`

* Run the Clippy linter on all targets: `bazel build --output_groups=clippy_checks //...`

* Regenerate rust-analyzer config: `bazel run @rules_rust//tools/rust_analyzer:gen_rust_project`

* Repin dependencies: `CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index`

## TODOs

* Figure out how to build multiple profiles. Is the macro used in axum_app how it is supposed to be?

* Figure out how to run and debug from inside VSCode

* Figure out how to build for linux-amd64 from MacOS M1

## VSCode setup

A [Github issue](https://github.com/bazelbuild/bazel/issues/10653#issuecomment-694230015) describes a problem with finding build targets in vscode.
There are two ways to fix it:

1. Force sharing of bazel server ([comment](https://github.com/bazelbuild/bazel/issues/10653#issuecomment-1131481319))
2. Add a line to `.bazelignore`, according to [another comment](https://github.com/bazelbuild/bazel/issues/10653#issuecomment-694230015) on the same issue.

Both worked for me, choosing to stick with vscode option!
