use deepgram::transcription::prerecorded::response::Response;
use itertools::Itertools;
use time::Duration;

#[derive(Debug)]
pub struct Srt {
    pub channels: Vec<Vec<String>>,
}

impl From<Response> for Srt {
    fn from(response: Response) -> Self {
        let channels =
            response
                .results
                .channels
                .iter()
                .map(|channel| {
                    channel
                        .alternatives
                        .iter()
                        .map(|alt| {
                            alt.words
                            .iter()
                            .chunks(5)
                            .into_iter()
                            .enumerate()
                            .map(|(index, words_chunk)| {
                             let (start, end, words) = words_chunk.map(|word| (
                                    word.start,
                                    word.end,
                                    String::from(if let Some(word) = &word.punctuated_word {
                                        word
                                    } else {
                                        &word.word
                                    })
                                )).reduce(|mut acc, item| {
                                        acc.1 = item.1;
                                        acc.2.push(' ');
                                        acc.2.push_str(&item.2);
                                        acc
                                }).unwrap();

                                format!(
                                    "{}\n{} --> {}\n{}\n\n",
                                    index + 1,
                                    seconds_to_timestamp(
                                        (start * 1000.)
                                            as i64
                                    ),
                                    seconds_to_timestamp(
                                        (end * 1000.)
                                            as i64
                                    ),
                                    words
                                )
                            })
                            .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                })
                .collect();

        Srt { channels: channels }
    }
}

fn seconds_to_timestamp(milliseconds: i64) -> String {
    let d = Duration::milliseconds(milliseconds);
    format!(
        "{:02}:{:02}:{:02},{:03}",
        d.whole_hours(),
        d.whole_minutes() % 60,
        d.whole_seconds() % 60,
        d.whole_milliseconds() % 1000
    )
}

#[test]
fn to_srt_test() {
    // note that this response has
    // results.utterances[*].words as an empty array
    // for brevity a true response would also have
    // that filled in
    let data = r#"
    {
        "metadata": {
          "transaction_key": "string",
          "request_id": "1e60a5d3-b237-4627-8334-7256e341ef67",
          "sha256": "string",
          "created": "string",
          "duration": 0,
          "channels": 0
        },
        "results": {
          "channels": [],
          "utterances": [
            {
              "start": 0.41915998,
              "end": 5.43012,
              "confidence": 0.88172823,
              "channel": 0,
              "transcript": "four score and seven years ago our fathers brought forth on this continent a new nation",
              "words": [],
              "id": "2d8211a4-3a5b-4053-8939-edf2b2b389fa"
            },
            {
              "start": 5.8882,
              "end": 9.880199,
              "confidence": 0.9834162,
              "channel": 0,
              "transcript": "conceived liberty and dedicated to the proposition that all men are created equal",
              "words": [],
              "id": "e88264de-a8cf-44e9-a7db-848ad5bab7a5"
            },
            {
              "start": 10.048263,
              "end": 433317.190998,
              "confidence": 0.9015952,
              "channel": 0,
              "transcript": "now we are engaged in a great civil war testing whether that nation or any nations open conceived and so dedicated can long endure",
              "words": [],
              "id": "1e60a5d3-b537-4627-8334-7256e341ef67"
            }
          ]
        }
      }
    "#;

    let resp: Response =
        serde_json::from_str(data).unwrap();

    let srt =
        Srt::try_from(resp).expect("subtitle srt failed");
    assert_eq!(srt.value, "1\n00:00:00,419 --> 00:00:05,430\nfour score and seven years ago our fathers brought forth on this continent a new nation\n\n2\n00:00:05,888 --> 00:00:09,880\nconceived liberty and dedicated to the proposition that all men are created equal\n\n3\n00:00:10,048 --> 120:21:57,190\nnow we are engaged in a great civil war testing whether that nation or any nations open conceived and so dedicated can long endure\n\n");
}
