use crate::deepgram::transcription::prerecorded::response::Response;
use time::Duration;

#[derive(Debug)]
pub struct Srt {
    pub value: String,
}

impl TryFrom<Response> for Srt {
    type Error = String;

    fn try_from(
        response: Response,
    ) -> Result<Self, Self::Error> {
        let srt = response
            .results
            .utterances
            .ok_or("response.results.utterances does not exist. Creating an srt requires a transcript that was generated with the utterances feature".to_string())?
            .iter()
            .enumerate()
            .map(|(index, utterance)| {
                format!(
                    "{}\n{} --> {}\n{}\n\n",
                    index + 1,
                    seconds_to_timestamp(
                        (utterance.start * 1000.) as i64
                    ),
                    seconds_to_timestamp(
                        (utterance.end * 1000.) as i64
                    ),
                    utterance.transcript
                )
            })
            .collect::<String>();

        Ok(Srt { value: srt })
    }
}

fn seconds_to_timestamp(milliseconds: i64) -> String {
    let d = Duration::milliseconds(milliseconds);
    format!(
        "{}:{}:{},{}",
        d.whole_hours(),
        d.whole_minutes() % 60,
        d.whole_seconds() % 60,
        d.whole_milliseconds() % 1000
    )
}

#[test]
fn to_srt_test() {
    // note that this response has results.utterances[*].words as an empty array for brevity
    // a true response would also have that filled in
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
              "start": 10.648263,
              "end": 333317.190998,
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
    assert_eq!(srt.value, "1\n0:0:0,419 --> 0:0:5,430\nfour score and seven years ago our fathers brought forth on this continent a new nation\n\n2\n0:0:5,888 --> 0:0:9,880\nconceived liberty and dedicated to the proposition that all men are created equal\n\n3\n0:0:10,648 --> 92:35:17,190\nnow we are engaged in a great civil war testing whether that nation or any nations open conceived and so dedicated can long endure\n\n");
}
