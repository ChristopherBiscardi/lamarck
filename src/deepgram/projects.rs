//! Manage Deepgram Projects.
//!
//! See the [Deepgram API Reference][api] for more info.
//!
//! [api]: https://developers.deepgram.com/api-reference/#projects

use crate::deepgram::Deepgram;

/// Manage Deepgram Projects.
///
/// Constructed using [`Deepgram::projects`].
///
/// See the [Deepgram API Reference][api] for more info.
///
/// [api]: https://developers.deepgram.com/api-reference/#projects
#[derive(Debug, Clone)]
pub struct Projects<'a, K: AsRef<str>>(&'a Deepgram<K>);

impl<'a, K: AsRef<str>> Deepgram<K> {
    /// Construct a new [`Projects`] from a [`Deepgram`].
    pub fn projects(&'a self) -> Projects<'a, K> {
        self.into()
    }
}

impl<'a, K: AsRef<str>> From<&'a Deepgram<K>> for Projects<'a, K> {
    /// Construct a new [`Projects`] from a [`Deepgram`].
    fn from(deepgram: &'a Deepgram<K>) -> Self {
        Self(deepgram)
    }
}
