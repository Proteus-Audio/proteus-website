# Command Reference

The installed binary is `prot`.

```text
prot [OPTIONS] [INPUT]
prot <COMMAND> [COMMAND_OPTIONS]
```

## Root Playback

```sh
prot song.prot
prot song.mka
prot input.wav
prot ./song-project
```

Options:

- `-s, --seek <TIME>` seeks to a time in seconds before playback.
- `-g, --gain <GAIN>` sets playback gain. Default: `70`.
- `-E, --effects-json <PATH>` loads a JSON `Vec<AudioEffect>`.
- `--start-buffer-ms <MS>` sets startup audio buffering. Default: `20`.
- `--start-sink-chunks <CHUNKS>` sets queued sink chunks before playback starts or resumes. Default: `3`.
- `--max-sink-chunks <CHUNKS>` sets the maximum queued sink chunks before producer wait. `0` disables the limit. Default: `40`.
- `--startup-silence-ms <MS>` adds silence pre-roll. Default: `0`.
- `--startup-fade-ms <MS>` sets playback-start fade-in. Default: `150`.
- `--append-jitter-log-ms <MS>` logs append jitter above the threshold. Default: `0`.
- `--effect-boundary-log` logs per-effect DSP boundary discontinuities.
- `--track-eos-ms <MS>` sets the container track end-of-stream threshold. Default: `1000`.
- `--read-durations` reads duration metadata and exits.
- `--scan-durations` scans packets for durations and exits.
- `-q, --quiet` suppresses the terminal UI and console output.
- `-d <debug>` enables debug output.

## `info`

Display container information in a terminal UI:

```sh
prot info song.prot
```

Print information to stdout:

```sh
prot info song.prot --print
```

Printed output includes file path, track count, channel count, sample rate, bits per sample, and per-track durations when available.

## `verify`

Probe or decode audio without playback:

```sh
prot verify probe song.prot
prot verify decode song.prot
prot verify verify song.prot
```

- `probe` reads container metadata.
- `decode` decodes packets and reports decode errors.
- `verify` decodes packets in strict mode and fails when decode errors are found.

## `peaks`

Output waveform peaks as JSON:

```sh
prot peaks json input.wav
prot peaks json input.wav --limited
```

Legacy JSON mode is also accepted:

```sh
prot peaks input.wav
```

Write binary peaks:

```sh
prot peaks write input.wav input.peaks
```

Read binary peaks as JSON:

```sh
prot peaks read input.peaks
```

Read a window with a target peak count and channel limit:

```sh
prot peaks read input.peaks --start 10 --end 25 --peaks 800 --channels 2
```

`--start` and `--end` must be provided together.

## `init`

Generate directory project files:

```sh
prot init ./song-project
```

The input must be a directory containing supported audio files.

## `create`

Print a default enabled effects chain:

```sh
prot create effects-json
```

Redirect it to a file for editing:

```sh
prot create effects-json > effects_chain.json
```

## `meter`

Run offline effect-chain metering:

```sh
prot meter effects input.wav
```

This command requires the CLI to be installed with the `effect-meter-cli` feature.

## `bench`

Run DSP benchmarks:

```sh
prot bench dsp
prot bench sweep
```

Benchmark commands require the CLI to be installed with the `bench` feature.
