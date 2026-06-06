# Installation

Install Proteus CLI from crates.io:

```sh
cargo install proteus-cli
```

This installs the `prot` binary.

```sh
prot --help
prot --version
```

## Feature-Gated Commands

Some diagnostic commands are compiled behind Cargo features in the CLI crate.

Benchmark commands require the `bench` feature:

```sh
cargo install proteus-cli --features bench
prot bench dsp
```

Offline effect metering requires `effect-meter-cli`:

```sh
cargo install proteus-cli --features effect-meter-cli
prot meter effects input.wav --format table
```

Spectral effect metering requires `effect-meter-cli-spectral`:

```sh
cargo install proteus-cli --features effect-meter-cli-spectral
prot meter effects input.wav --spectral
```

## Command Shape

The root command accepts playback options and an optional input path:

```text
prot [OPTIONS] [INPUT]
```

Subcommands use this form:

```text
prot <COMMAND> [COMMAND_OPTIONS]
```
