use deepgram::transcription::prerecorded::response::Response;
use itertools::Itertools;
use time::Duration;

#[derive(Debug)]
pub struct BeastCaptions {
    pub channels: Vec<Vec<String>>,
}

impl From<Response> for BeastCaptions {
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
                            .chunks(1)
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

        BeastCaptions { channels: channels }
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
