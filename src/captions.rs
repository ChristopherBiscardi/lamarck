use camino::Utf8PathBuf;
use clap::Args;
use deepgram::{
    transcription::prerecorded::{
        audio_source::AudioSource,
        options::{Language, Options},
    },
    Deepgram, DeepgramError,
};
use indicatif::{ProgressBar, ProgressStyle};
use miette::Diagnostic;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::*;
use url::Url;

mod srt;
use srt::*;

#[derive(Args, Debug)]
pub struct Caption {
    /// captions require a deepgram API key
    #[clap(env, long)]
    deepgram_api_key: String,
    /// A path to an audio file or a URL
    #[clap(short, long, value_parser)]
    input: String,
    /// a filepath to use for the output.
    ///
    /// The filename will be preserved if it
    /// exists
    ///
    /// The file extension will be replaced if it
    /// exists
    #[clap(short, long, value_parser)]
    output_path: Option<Utf8PathBuf>,
    /// output an srt file
    #[clap(
        short,
        long,
        default_value_t = false,
        help_heading = "OUTPUT_TYPE"
    )]
    srt: bool,
    /// output a transcript
    #[clap(
        short,
        long,
        default_value_t = false,
        help_heading = "OUTPUT_TYPE"
    )]
    transcript: bool,
    /// output a markdown file with links to video
    /// timestamps
    #[clap(
        short,
        long,
        default_value_t = false,
        help_heading = "OUTPUT_TYPE"
    )]
    markdown: bool,
}

#[derive(Error, Diagnostic, Debug)]
pub enum CaptionError {
    #[error(transparent)]
    #[diagnostic(code(lamarck::io_error))]
    IoError(#[from] std::io::Error),

    #[error(
        "Failed to parse a URL or a FilePath from input"
    )]
    #[diagnostic(code(lamarck::input_parse_error))]
    InputParseError {
        url_error: url::ParseError,
        file_error: camino::FromPathBufError,
    },
    #[error("Deepgram reported an error")]
    #[diagnostic(code(lamarck::deepgram_error))]
    DeepgramError { error: DeepgramError },

    #[error(
      "The supplied output-dir doesn't exist. Create it if you wish to write files there."
    )]
    #[diagnostic(code(lamarck::output_dir_not_exist))]
    OutputDirNotExistError { output_dir: Utf8PathBuf },

    #[error(
        "Couldn't guess a mime type for the input file, try specifying it."
      )]
    #[diagnostic(code(lamarck::mime_could_not_guess))]
    MimeGuessError { filepath: Utf8PathBuf },

    #[error(
        "Media Type (mime) is not an audio file. Deepgram requires an audio file."
      )]
    #[diagnostic(code(lamarck::mime_not_audio))]
    InvalidMimeType { guess: mime_guess::Mime },
}

impl From<DeepgramError> for CaptionError {
    fn from(dg_error: DeepgramError) -> Self {
        CaptionError::DeepgramError { error: dg_error }
    }
}

pub async fn generate_captions(
    options: &Caption,
) -> Result<(), CaptionError> {
    let bar = ProgressBar::new(1);

    bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {spinner} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-"));
    bar.set_message("generating captions...");

    let output_file = options
        .output_path
        .clone()
        .unwrap_or(Utf8PathBuf::from("transcript.srt"));
    let output_dir_exists = match output_file.file_name() {
        Some(_) => {
            // if we have a file name, then make sure the
            // parent dir exists
            if let Some(parent) = output_file.parent() {
                // TODO: what if we only have a filename and
                // no parent dir
                parent.exists()
            } else {
                true
            }
        }
        None => output_file.exists(),
    };

    if !output_dir_exists {
        return Err(CaptionError::OutputDirNotExistError {
            output_dir: output_file,
        });
    }

    let source = match Url::parse(&options.input) {
        Ok(_) => Ok(AudioSource::from_url(&options.input)),
        Err(url_error) => {
            debug!("url failed to parse {:?}", url_error);
            let filepath =
                Utf8PathBuf::from(&options.input);
            let file = File::open(&filepath).await.unwrap();

            match mime_guess::from_path(&options.input)
                .first()
            {
                Some(guess) => {
                    if guess.type_() != "audio" {
                        Err(CaptionError::InvalidMimeType {
                            guess,
                        })
                    } else {
                        Ok(AudioSource::from_buffer_with_mime_type(
                            file,
                            guess.to_string(),
                        ))
                    }
                }
                None => Err(CaptionError::MimeGuessError {
                    filepath: filepath,
                }),
            }
        }
    }?;

    let dg_client =
        Deepgram::new(&options.deepgram_api_key);

    let deepgram_options = Options::builder()
        .punctuate(true)
        .language(Language::en_US)
        .utterances(true)
        .build();

    bar.set_message("waiting for deepgram");
    let response = dg_client
        .transcription()
        .prerecorded(source, &deepgram_options)
        .await?;

    bar.set_message("processing deepgram response");

    let transcript = &response.results.channels[0]
        .alternatives[0]
        .transcript;

    if options.transcript {
        let mut output = output_file.clone();
        output.set_extension("txt");
        let mut transcript_file =
            File::create(output).await?;
        transcript_file
            .write_all(transcript.as_bytes())
            .await?;
    }

    if options.srt {
        let mut output = output_file.clone();
        output.set_extension("srt");

        let srt = Srt::try_from(response).unwrap();

        let mut srt_file = File::create(output).await?;
        srt_file.write_all(srt.value.as_bytes()).await?;
    }

    if options.markdown {
        warn!("markdown output is not yet implemented");
    }

    bar.finish_with_message("created caption files");
    Ok(())
}
