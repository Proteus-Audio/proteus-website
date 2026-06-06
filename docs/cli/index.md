# CLI Overview

Proteus CLI is installed from the `proteus-cli` crate and run with the `prot` binary.

```sh
cargo install proteus-cli
prot path/to/file.prot
```

The root command starts interactive playback when an input path is provided. Subcommands handle non-playback workflows such as `info`, `verify`, `peaks`, `init`, `create`, `meter`, and `bench`.

## Common Workflows

Play a packaged Proteus file:

```sh
prot song.prot
```

Print container information:

```sh
prot info song.prot --print
```

Create project JSON files for a directory of audio takes:

```sh
prot init ./song-project
```

Generate waveform peaks:

```sh
prot peaks json take.wav
```

Generate a default effects JSON payload:

```sh
prot create effects-json > effects_chain.json
```
