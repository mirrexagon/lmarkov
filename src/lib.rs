#[cfg(test)]
mod tests;

use std::collections::HashMap;

use rand::seq::SliceRandom;

/// A Markov chain.
#[derive(Debug)]
pub struct Chain {
    /// A map from `order` words to the possible following words.
    /// `None` means the start or end of a training input.
    map: HashMap<Vec<Option<String>>, Vec<Option<String>>>,
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

            let map_entry = self.map.entry(key.to_vec()).or_insert(Vec::new());
            map_entry.push(word.clone());
        }
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut result: Vec<String> = Vec::new();

        // Start with a key of all `None` to match starting from the start of
        // one of the training inputs.
        let mut cursor = vec![None; self.order];

        loop {
            let possible_words = &self.map[&cursor];
            let next_word = possible_words.choose(&mut rng).unwrap();

            if let Some(next_word) = next_word {
                result.push(next_word.clone());
            } else {
                // Terminator word.
                break;
            }

            // Advance the cursor along by popping the front and appending the
            // new word on the end.
            cursor = cursor[1..self.order].to_vec();
            cursor.push(next_word.clone());
        }

        result.join(" ")
    }
}
