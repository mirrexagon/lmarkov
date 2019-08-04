#[cfg(test)]
mod tests;

use std::collections::HashMap;

/// A Markov chain.
#[derive(Debug)]
pub struct Chain {
    /// A map from `order` words to the possible following words.
    map: HashMap<Vec<Option<String>>, Vec<String>>,
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

            if let Some(word) = word {
                let mut map_entry = self.map.entry(key).or_insert(Vec::new());
                map_entry.push(word.to_string());
            }
        }
    }
}
