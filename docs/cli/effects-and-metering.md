# Effects and Metering

Proteus CLI can load effect chains during playback and inspect effect chains offline.

## Generate Effects JSON

Create a default enabled `Vec<AudioEffect>` JSON payload:

```sh
prot create effects-json > effects_chain.json
```

Use it during playback:

```sh
prot song.prot --effects-json effects_chain.json
```

For directory projects, `effects_chain.json` is loaded automatically when it exists in the project directory. A command-line `--effects-json` path overrides the directory file.

## Offline Metering

The `meter effects` command runs an input through an effect chain and prints before/after levels.

```sh
prot meter effects input.wav --effects-json effects_chain.json
```

The command requires the `effect-meter-cli` feature:

```sh
cargo install proteus-cli --features effect-meter-cli
```

## Output Formats

Table output is the default:

```sh
prot meter effects input.wav --format table
```

Bars output prints compact per-effect meter lines:

```sh
prot meter effects input.wav --format bars
```

JSON output is useful for scripts:

```sh
prot meter effects input.wav --format json
```

## Metering Options

Seek before metering:

```sh
prot meter effects input.wav --seek 30
```

Meter only a duration after the seek point:

```sh
prot meter effects input.wav --seek 30 --duration 10
```

Choose the summary mode:

```sh
prot meter effects input.wav --summary max
prot meter effects input.wav --summary final
```

`max` reports the maximum observed level. `final` reports the final summary state.

## Spectral Buckets

Add `--spectral` to include spectral buckets for supported filter effects.

```sh
prot meter effects input.wav --effects-json effects_chain.json --spectral
```

Spectral metering requires the `effect-meter-cli-spectral` feature.
