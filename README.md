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
    -l, --lang
            which language the audio file is in            

OUTPUT_TYPE:
    -m, --markdown <markdown>        output a markdown file with links to video timestamps [default:
                                     false]
    -s, --srt <srt>                  output an srt file [default: false]
    -t, --transcript <transcript>    output a transcript [default: false]
```


##### Naming

Lamarck is short for [Lamarckdromia Beagle](https://hu.wikipedia.org/wiki/Lamarckdromia_beagle) and is also French for "the mark", like the marks you'd make on a live stream to pull out clips.

##### Forking Notes
We added a `--lang flag`
usage is now `target/debug/lamarck caption --deepgram-api-key <key> --input test_german.mp3 --lang de`

history log

 1119  target/debug/lamarck caption --deepgram-api-key bce30426da3ae2e613e3a5ae459f0a70e533f515 --input audio/test_german.mp3 --lang de
 1120  mkdir audio
 1121  target/debug/lamarck caption --deepgram-api-key bce30426da3ae2e613e3a5ae459f0a70e533f515 --input audio/test_german.mp3 --lang de
 1122  gst
 1123  gst 
 1124  git add .
 1125  git commit -m"remove strum, and string match enum languages"
 1126  git push
 1127  cargo build --release
 1128  gst
 1129  git push
 1130  touch Dockerfile
 1131  docker
 1132  docker run -p 8080:8080 my_program_image
 1133  cargo clean
 1134  cargo build --release --verbose
 1135  cd target/
 1136  cd release/
 1137  ls -la
 1138  cd ..
 1139  docker build -t lamarck
 1140  docker build -t lamarck .
 1141  docker run -it lamarck /bin/bash
 1142  docker images
 1143  docker build -t lamarck .
 1144  docker run -it lamarck /bin/bash
 1145  docker images
 1147  docker run -it lamarck /bin/bash
 1148  docker build -t lamarck .
 1161  cargo clean
 1162  cargo build --release --verbose
 1163  docker build -t lamarck .
 1164  docker run -it lamarck /bin/bash
 1165  cargo clean
 1166  cargo build --release --verbose --target x86_64-pc-windows-msvc
 1167  cargo build --release --verbose
 1168  docker build -t lamarck .
 1169  docker run -it lamarck /bin/bash
 1170  cargo clean
 1171  cargo build --release --verbose --target x86_64-pc-windows-msvc
 1172  rustup target add x86_64-pc-windows-msvc
 1173  cargo clean
 1174  cargo build --release --verbose --target x86_64-pc-windows-msvc
 1175  docker build -t lamarck .
 1176  docker run -it lamarck /bin/bash
 1177  cargo clean
 1178  cargo build --release --verbose --target x86_64-unknown-linux-gnu
 1179  history 500
