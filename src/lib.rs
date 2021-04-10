#[cfg(test)]
mod tests;

use std::collections::HashMap;

#[cfg(feature = "serialization")]
use std::fmt;

use rand::seq::SliceRandom;

#[cfg(feature = "serialization")]
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serialization")]
const KEY_NO_WORD: &str = "\n";
#[cfg(feature = "serialization")]
const KEY_SEPARATOR: &str = " ";

/// A sequence of words, used as the key in a `Chain`'s map.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ChainKey(Vec<Option<String>>);

impl ChainKey {
    pub fn blank(order: usize) -> Self {
        ChainKey(vec![None; order])
    }

    pub fn from_vec(vec: Vec<Option<String>>) -> Self {
        ChainKey(vec)
    }

    pub fn to_vec(self) -> Vec<Option<String>> {
        self.0
    }

    pub fn advance(&mut self, next_word: &Option<String>) {
        self.0 = self.0[1..self.0.len()].to_vec();
        self.0.push(next_word.clone());
    }

    #[cfg(feature = "serialization")]
    fn to_string(&self) -> String {
        let mut result = String::new();

        let mut first = true;

        for word in &self.0 {
            if first {
                first = false;
            } else {
                result.push_str(KEY_SEPARATOR);
            }

            if let Some(word) = word {
                result.push_str(&word);
            } else {
                result.push_str(KEY_NO_WORD);
            }
        }

        result
    }

    /// TODO: Check input for correctness.
    #[cfg(feature = "serialization")]
    fn from_str(string: &str) -> Self {
        let mut result = Vec::new();

        for word in string.split(KEY_SEPARATOR) {
            if word == KEY_NO_WORD {
                result.push(None);
            } else {
                result.push(Some(word.to_string()));
            }
        }

        ChainKey(result)
    }
}

#[cfg(feature = "serialization")]
impl Serialize for ChainKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serialization")]
struct ChainKeyVisitor;

#[cfg(feature = "serialization")]
impl<'de> Visitor<'de> for ChainKeyVisitor {
    type Value = ChainKey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ChainKey::from_str(value))
    }
}

#[cfg(feature = "serialization")]
impl<'de> Deserialize<'de> for ChainKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChainKeyVisitor)
    }
}

/// A Markov chain.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Chain {
    /// A map from `order` words to the possible following words.
    map: HashMap<ChainKey, Vec<Option<String>>>,
    order: usize,
}

impl Chain {
    pub fn new(order: usize) -> Self {
        Chain {
            map: HashMap::new(),
            order,
        }
    }

    pub fn train(&mut self, string: &str) {
        // Create a Vec that starts with `self.order` `None`s, then all the
        // words in the string wrapped in `Some()`, then a single `None`.
        let mut words = vec![None; self.order];

        for word in string.split_whitespace() {
            words.push(Some(word.to_string()));
        }

        words.push(None);

        // Now slide a window over `words` to produce slices where the last
        // element is the resulting word and the rest is the key to that word.
        for window in words.windows(self.order + 1) {
            let key = &window[..self.order];
            let word = &window[self.order];

            let map_entry = self
                .map
                .entry(ChainKey::from_vec(key.to_vec()))
                .or_insert(Vec::new());
            map_entry.push(word.clone());
        }
    }

    // Generate a string.
    pub fn generate(&self) -> Option<String> {
        // Start with a key of all `None` to match starting from the start of
        // one of the training inputs.
        let seed = ChainKey::blank(self.order);

        self.generate_from_seed(&seed)
    }

    /// Generate a string based on some seed words.
    /// Returns `None` if there is no way to start a generated string with
    /// that seed, eg. it is longer than `self.order`.
    pub fn generate_from_seed(&self, seed: &ChainKey) -> Option<String> {
        if !self.map.contains_key(seed) {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut result: Vec<String> = Vec::new();

        let mut cursor = seed.clone();

        loop {
            let possible_words = &self.map[&cursor];

            // Any entry in the map is guaranteed to have at least one word in
            // it, so this unwrap is okay.
            let next_word = possible_words.choose(&mut rng).unwrap();

            if let Some(next_word) = next_word {
                result.push(next_word.clone());
            } else {
                // Terminator word.
                break;
            }

            // Advance the cursor along by popping the front and appending the
            // new word on the end.
            cursor.advance(next_word);
        }

        Some(result.join(" "))
    }

    /// Serialize this chain to JSON.
    #[cfg(feature = "serialization")]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Load a chain from JSON.
    #[cfg(feature = "serialization")]
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}
