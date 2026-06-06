# Proteus CLI

Proteus CLI is the command-line tool for playing and inspecting Proteus audio from a terminal.

Install it with Cargo:

```sh
cargo install proteus-cli
```

The installed command is `prot`.

```sh
prot path/to/song.prot
```

## What It Supports

- Play `.prot` and `.mka` containers.
- Play a single audio file.
- Play a directory of nested audio files as a Proteus-style randomized project.
- Generate directory project files with `shuffle_schedule.json` and `effects_chain.json`.
- Inspect container metadata.
- Probe, decode, and verify audio without starting playback.
- Extract waveform peaks as JSON or binary peak files.
- Generate and inspect audio effect chains.

Start with [installation](/docs/cli/installation), then use the [playback guide](/docs/cli/playback) or the [command reference](/docs/cli/commands).
