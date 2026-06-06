# Playback

Run `prot` with a file or directory path to start interactive playback.

```sh
prot song.prot
prot song.mka
prot take.wav
prot ./song-project
```

Supported standalone audio extensions include `wav`, `wave`, `flac`, `aif`, `aiff`, `mp3`, `m4a`, `aac`, `ogg`, and `opus`.

## Playback Options

Set initial playback gain as a percentage:

```sh
prot song.prot --gain 80
```

Seek before playback starts:

```sh
prot song.prot --seek 45
```

Load an effects chain from JSON:

```sh
prot song.prot --effects-json effects_chain.json
```

Run without the terminal UI:

```sh
prot song.prot --quiet
```

## Interactive Controls

While playback is running:

- `space` toggles play and pause.
- `s` shuffles the current Proteus selection.
- `left arrow` seeks backward five seconds.
- `right arrow` seeks forward five seconds.
- `r` toggles reverb.
- `-` lowers the reverb mix.
- `=` or `+` raises the reverb mix.
- `q` exits.
- `ctrl-c` exits.

## Buffering Options

These options tune playback startup and buffering behavior:

```sh
prot song.prot --start-buffer-ms 20
prot song.prot --start-sink-chunks 3
prot song.prot --max-sink-chunks 40
prot song.prot --startup-silence-ms 0
prot song.prot --startup-fade-ms 150
```

`--append-jitter-log-ms` logs sink append jitter events above a threshold. `--effect-boundary-log` logs per-effect discontinuities in the DSP chain.

## Duration Inspection

Print duration metadata and exit:

```sh
prot song.prot --read-durations
```

Scan packets to compute per-track durations and exit:

```sh
prot song.prot --scan-durations
```
