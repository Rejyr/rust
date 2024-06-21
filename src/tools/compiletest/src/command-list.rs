/// This was originally generated by collecting directives from ui tests and then extracting their
/// directive names. This is **not** an exhaustive list of all possible directives. Instead, this is
/// a best-effort approximation for diagnostics. Add new headers to this list when needed.
const KNOWN_DIRECTIVE_NAMES: &[&str] = &[
    // tidy-alphabetical-start
    "assembly-output",
    "aux-bin",
    "aux-build",
    "aux-codegen-backend",
    "aux-crate",
    "build-aux-docs",
    "build-fail",
    "build-pass",
    "check-fail",
    "check-pass",
    "check-run-results",
    "check-stdout",
    "check-test-line-numbers-match",
    "compare-output-lines-by-subset",
    "compile-flags",
    "dont-check-compiler-stderr",
    "dont-check-compiler-stdout",
    "dont-check-failure-status",
    "edition",
    "error-pattern",
    "exec-env",
    "failure-status",
    "filecheck-flags",
    "forbid-output",
    "force-host",
    "ignore-16bit",
    "ignore-32bit",
    "ignore-64bit",
    "ignore-aarch64",
    "ignore-aarch64-unknown-linux-gnu",
    "ignore-android",
    "ignore-apple",
    "ignore-arm",
    "ignore-avr",
    "ignore-beta",
    "ignore-cdb",
    "ignore-compare-mode-next-solver",
    "ignore-compare-mode-polonius",
    "ignore-cross-compile",
    "ignore-debug",
    "ignore-eabi",
    "ignore-emscripten",
    "ignore-endian-big",
    "ignore-freebsd",
    "ignore-fuchsia",
    "ignore-gdb",
    "ignore-gdb-version",
    "ignore-gnu",
    "ignore-haiku",
    "ignore-horizon",
    "ignore-i686-pc-windows-msvc",
    "ignore-illumos",
    "ignore-ios",
    "ignore-linux",
    "ignore-lldb",
    "ignore-llvm-version",
    "ignore-loongarch64",
    "ignore-macabi",
    "ignore-macos",
    "ignore-mode-assembly",
    "ignore-mode-codegen",
    "ignore-mode-codegen-units",
    "ignore-mode-coverage-map",
    "ignore-mode-coverage-run",
    "ignore-mode-crashes",
    "ignore-mode-debuginfo",
    "ignore-mode-incremental",
    "ignore-mode-js-doc-test",
    "ignore-mode-mir-opt",
    "ignore-mode-pretty",
    "ignore-mode-run-make",
    "ignore-mode-run-pass-valgrind",
    "ignore-mode-rustdoc",
    "ignore-mode-rustdoc-json",
    "ignore-mode-ui",
    "ignore-mode-ui-fulldeps",
    "ignore-msp430",
    "ignore-msvc",
    "ignore-musl",
    "ignore-netbsd",
    "ignore-nightly",
    "ignore-none",
    "ignore-nto",
    "ignore-nvptx64",
    "ignore-nvptx64-nvidia-cuda",
    "ignore-openbsd",
    "ignore-pass",
    "ignore-remote",
    "ignore-riscv64",
    "ignore-s390x",
    "ignore-sgx",
    "ignore-spirv",
    "ignore-stable",
    "ignore-stage1",
    "ignore-stage2",
    "ignore-test",
    "ignore-thumb",
    "ignore-thumbv8m.base-none-eabi",
    "ignore-thumbv8m.main-none-eabi",
    "ignore-tvos",
    "ignore-unix",
    "ignore-unknown",
    "ignore-uwp",
    "ignore-visionos",
    "ignore-vxworks",
    "ignore-wasi",
    "ignore-wasm",
    "ignore-wasm32",
    "ignore-wasm32-bare",
    "ignore-wasm64",
    "ignore-watchos",
    "ignore-windows",
    "ignore-windows-gnu",
    "ignore-x32",
    "ignore-x86",
    "ignore-x86_64",
    "ignore-x86_64-apple-darwin",
    "ignore-x86_64-unknown-linux-gnu",
    "incremental",
    "known-bug",
    "llvm-cov-flags",
    "min-cdb-version",
    "min-gdb-version",
    "min-lldb-version",
    "min-llvm-version",
    "min-system-llvm-version",
    "needs-asm-support",
    "needs-dlltool",
    "needs-dynamic-linking",
    "needs-force-clang-based-tests",
    "needs-git-hash",
    "needs-llvm-components",
    "needs-profiler-support",
    "needs-relocation-model-pic",
    "needs-run-enabled",
    "needs-rust-lld",
    "needs-rust-lldb",
    "needs-sanitizer-address",
    "needs-sanitizer-cfi",
    "needs-sanitizer-dataflow",
    "needs-sanitizer-hwaddress",
    "needs-sanitizer-kcfi",
    "needs-sanitizer-leak",
    "needs-sanitizer-memory",
    "needs-sanitizer-memtag",
    "needs-sanitizer-safestack",
    "needs-sanitizer-shadow-call-stack",
    "needs-sanitizer-support",
    "needs-sanitizer-thread",
    "needs-symlink",
    "needs-threads",
    "needs-unwind",
    "needs-wasmtime",
    "needs-xray",
    "no-auto-check-cfg",
    "no-prefer-dynamic",
    "normalize-stderr-32bit",
    "normalize-stderr-64bit",
    "normalize-stderr-test",
    "normalize-stdout-test",
    "only-16bit",
    "only-32bit",
    "only-64bit",
    "only-aarch64",
    "only-apple",
    "only-arm",
    "only-avr",
    "only-beta",
    "only-bpf",
    "only-cdb",
    "only-gnu",
    "only-i686-pc-windows-msvc",
    "only-ios",
    "only-linux",
    "only-loongarch64",
    "only-loongarch64-unknown-linux-gnu",
    "only-macos",
    "only-mips",
    "only-mips64",
    "only-msp430",
    "only-msvc",
    "only-nightly",
    "only-nvptx64",
    "only-riscv64",
    "only-sparc",
    "only-sparc64",
    "only-stable",
    "only-thumb",
    "only-tvos",
    "only-unix",
    "only-visionos",
    "only-wasm32",
    "only-wasm32-bare",
    "only-wasm32-wasip1",
    "only-watchos",
    "only-windows",
    "only-x86",
    "only-x86_64",
    "only-x86_64-fortanix-unknown-sgx",
    "only-x86_64-pc-windows-gnu",
    "only-x86_64-pc-windows-msvc",
    "only-x86_64-unknown-linux-gnu",
    "pp-exact",
    "pretty-compare-only",
    "pretty-expanded",
    "pretty-mode",
    "regex-error-pattern",
    "remap-src-base",
    "revisions",
    "run-fail",
    "run-flags",
    "run-pass",
    "run-rustfix",
    "rustc-env",
    "rustfix-only-machine-applicable",
    "should-fail",
    "should-ice",
    "stderr-per-bitwidth",
    "test-mir-pass",
    "unset-exec-env",
    "unset-rustc-env",
    // Used by the tidy check `unknown_revision`.
    "unused-revision-names",
    // tidy-alphabetical-end
];
