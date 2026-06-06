# Directory Projects

Proteus CLI can play a directory of nested audio files. This is useful for testing Proteus-style randomized playback without first exporting a `.prot` file.

```sh
prot ./song-project
```

If the directory contains `shuffle_schedule.json`, the CLI uses it to build the track layout. If it also contains `effects_chain.json`, the CLI loads that effects chain unless `--effects-json` is provided on the command line.

If no `shuffle_schedule.json` exists, the CLI discovers supported audio files recursively and groups files by their parent directory.

## Initialize Project Files

Generate `shuffle_schedule.json` and `effects_chain.json` for an existing directory of audio files:

```sh
prot init ./song-project
```

The command writes:

- `shuffle_schedule.json`, containing the discovered track groups.
- `effects_chain.json`, containing a disabled default effects chain.

## Shuffle Schedule

`shuffle_schedule.json` may be either a raw array of tracks or an object with a `tracks` array.

```json
{
  "tracks": [
    {
      "file_paths": ["drums/take-1.wav", "drums/take-2.wav"],
      "level": 1.0,
      "pan": 0.0,
      "selections_count": 1,
      "shuffle_points": []
    }
  ]
}
```

Relative paths are resolved from the project directory. Absolute paths are accepted. Missing files cause playback setup to fail.

## Track Fields

- `file_paths` lists the candidate audio files for the track.
- `level` defaults to `1.0`.
- `pan` defaults to `0.0`.
- `selections_count` defaults to `1` and is clamped to at least `1`.
- `shuffle_points` defaults to an empty list.

## Effects Chain

Create a default enabled effects chain:

```sh
prot create effects-json > effects_chain.json
```

The `init` command writes the same effect types with each effect disabled, which makes the file ready for editing without changing playback immediately.
