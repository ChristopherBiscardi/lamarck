# Lamarck

A CLI based audio/visual toolkit

## Installation

```
cargo install lamarck
```

## Getting Started

Currently the only available subcommand is `lamarck caption` which takes any audio file, detects the media type, and generates some output including transcript and .srt files.

```
lamarck-caption 0.1.0
Generate captions using deepgram

USAGE:
    lamarck caption [OPTIONS] --deepgram-api-key <DEEPGRAM_API_KEY> --input <INPUT>

OPTIONS:
        --deepgram-api-key <DEEPGRAM_API_KEY>
            captions require a deepgram API key

    -h, --help
            Print help information

    -i, --input <INPUT>
            A path to an audio file or a URL

    -o, --output-path <OUTPUT_PATH>
            a filepath to use for the output

    -V, --version
            Print version information

OUTPUT_TYPE:
    -m, --markdown <markdown>        output a markdown file with links to video timestamps [default:
                                     false]
    -s, --srt <srt>                  output an srt file [default: false]
    -t, --transcript <transcript>    output a transcript [default: false]
```


##### Naming

Lamarck is short for [Lamarckdromia Beagle](https://hu.wikipedia.org/wiki/Lamarckdromia_beagle) and is also French for "the mark", like the marks you'd make on a live stream to pull out clips.