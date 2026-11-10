# Rust and Bazel

This repository uses Bzlmod. `MODULE.bazel` declares the `rules_rust` version,
the Rust toolchain version, the Rust edition, and the crate dependencies. There
is no `WORKSPACE` file. Bazel 9 removed the `WORKSPACE` mechanism, and
`rules_rust` removed `WORKSPACE` support in release 0.71.0.

## Versions

| Component | Version | File |
| --- | --- | --- |
| Bazel | 9.3.2 | `.bazeliskrc` |
| rules_rust | 0.75.0 | `MODULE.bazel` |
| Rust toolchain | 1.99.1 | `MODULE.bazel` |
| Rust edition | 2024 | `MODULE.bazel` |

## Tasks

* Run Hello World: `bazel run //hello_world`

* Run Hello World Axum app: `bazel run //axum_app`

* Format: `bazel run @rules_rust//:rustfmt`

* Check formatting: `bazel build --aspects=@rules_rust//rust:defs.bzl%rustfmt_aspect --output_groups=rustfmt_checks //...`

* Regenerate rust-analyzer config: `bazel run @rules_rust//tools/rust_analyzer:gen_rust_project`

* Repin dependencies: `CARGO_BAZEL_REPIN=1 bazel mod deps --lockfile_mode=update`

## Clippy

`.bazelrc` applies the Clippy aspect to every build. A plain `bazel build //...`
therefore runs the linter, and a Clippy error fails the build. To build without
the linter, use `bazel build --output_groups=-clippy_checks //...`.

## Dependencies

`MODULE.bazel` holds one `crate.spec` tag for each direct crate dependency. The
`crate.from_specs` tag then resolves the full dependency tree. This repository
has no Cargo workspace and no hand-written `Cargo.toml` file.

To add a crate, do these three steps:

1. Add a `crate.spec` tag to `MODULE.bazel`.
2. Run `CARGO_BAZEL_REPIN=1 bazel mod deps --lockfile_mode=update`.
3. Add `@crate_index//<crate-name>` to the `deps` attribute of your target.

Two lock files record the result. Cargo writes `Cargo.lock`. The crate universe
generator writes `Cargo.Bazel.lock`. Commit both files.

Do not use the root alias form `@crate_index//:<crate-name>`. `rules_rust` plans
to remove those aliases under the `incompatible_no_root_alias_targets` flag.

## Debugging

Debugging works with Bazel 9.3.2, rules_rust 0.75.0, and Rust 1.99.1. The steps
below are verified on MacOS with an aarch64 CPU.

### Build with debug information

```sh
bazel build //axum_app -c dbg
```

The command writes two outputs:

1. The binary, at `bazel-bin/axum_app/axum_app`. The name has no suffix. The
   compilation mode does not change the name of the target.
2. A `.dSYM` bundle, at `bazel-bin/axum_app/axum_app.dSYM`. LLDB reads it
   automatically, because the bundle sits next to the binary.

Build with `-c dbg` each time, directly before you debug. `bazel-bin` is a
symlink to the output directory of the last build. A build in another
compilation mode moves that symlink.

### Debug in VSCode

Install the CodeLLDB extension (`vadimcn.vscode-lldb`). Press F5, then select
"Debug axum_app" or "Debug hello_world". Each configuration in
`.vscode/launch.json` runs its `-c dbg` build task first.

### Debug on the command line

```sh
lldb -o "settings set target.source-map . $(pwd)" bazel-bin/axum_app/axum_app
```

### How the source paths resolve

rules_rust compiles each file with a relative path. The DWARF data records
`DW_AT_comp_dir` as `"."`, and records the source of the Axum app as
`./axum_app/main.rs`. LLDB therefore finds the sources in two ways:

1. The debugger runs with the workspace root as the current directory. The
   `cwd` key of the launch configuration sets this.
2. The debugger maps `.` to the workspace root. The `sourceMap` key of the
   launch configuration sets this.

The launch configuration sets both keys. Do not remove either one.

### Rust data formatters

The launch configuration sets `"sourceLanguages": ["rust"]`. CodeLLDB then loads
the LLDB data formatters of the Rust toolchain. The formatters change the output
of the debugger in two ways:

1. A `String` field shows its text. A `Greeter` value prints as
   `{greeting:"Hello"}`.
2. A `&str` value shows the correct number of characters. Without the
   formatters, LLDB prints the text together with adjacent bytes of the string
   table.

CodeLLDB looks for the formatters in the rustup toolchain. Two conditions apply:

1. If you have no rustup installation, set the `lldb.script.lang.rust.sysroot`
   setting of VSCode. Point it at the toolchain that Bazel downloads.
2. Keep the rustup toolchain at the same version as `MODULE.bazel`. The
   formatters read the internal layout of the standard library types.

## TODOs

* Build multiple compilation profiles. See the `default_alias_rule_name`
  attribute of `crate.render_config`, which can transition `compilation_mode`.

* Build for linux-amd64 from MacOS M1. See the `extra_target_triples` attribute
  of `rust.toolchain` in `MODULE.bazel`.


## VSCode setup

[Bazel issue 10653](https://github.com/bazelbuild/bazel/issues/10653) described a
problem with the discovery of build targets in VSCode. The issue is now closed.
Two fixes exist:

1. Share the Bazel server, as
   [one comment](https://github.com/bazelbuild/bazel/issues/10653#issuecomment-1131481319)
   describes. `.vscode/settings.json` uses this fix.
2. Add a line to `.bazelignore`, as
   [another comment](https://github.com/bazelbuild/bazel/issues/10653#issuecomment-694230015)
   describes. This repository has no `.bazelignore` file.
