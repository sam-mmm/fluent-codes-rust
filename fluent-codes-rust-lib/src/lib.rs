// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Rust fluent codes
//!
//! This library can be use to generate fluent codes.
//!
//! ## Example
//!
//! To generate code with four words:
//!
//! ```rust
//! use fluent_codes_rust::FluentCodes;
//! println!("{}",FluentCodes::generate_code_with_four_words())
//! ```
//! ```text
//! Output: fluffy-vacuum-misuse-deadly
//! ```
//!
//! To generate code with three words and six digits:
//!
//! ```rust
//! use fluent_codes_rust::FluentCodes;
//! println!("{}",FluentCodes::generate_code_with_three_words_and_six_digits())
//! ```
//! ```text
//! Output: calmer-taints-fourty-887709
//! ```
//! Or you can use builder methods:
//!
//! ```rust
//! use fluent_codes_rust::FluentCodes;
//! println!("{}",FluentCodes::default()
//!             .with_min_length(3).with_max_length(8)
//!             .with_joiner("..{-_-}..".to_string())
//!             .adjective().adposition().adverb()
//!             .auxiliary().coordinating_conjunction().determiner().interjection()
//!             .noun().particle().pronoun().proper_noun()
//!             .punctuation().subordinating_conjunction().symbol().verb().six_digits()
//!             .to_string())
//! ```
//!
//! ```text
//! Output: fused..{-_-}..jpg..{-_-}..reliably..{-_-}..lolcat..{-_-}..jdlugosz..{-_-}..resarted..{-_-}..878533
//! ```
//!
//! ### Words
//!
//! Words are generated using code @ https://github.com/sam-mmm/word_generator
//!
//! Definitions of terms adjective, adposition, adverb, auxiliary, coordinatingConjunction, determiner, interjection, noun, numeral, particle, pronoun, properNoun, punctuation, subordinatingConjunction, symbol, verb
//! are from https://universaldependencies.org/u/pos/
//! ### License
//!
//! http://www.apache.org/licenses/LICENSE-2.0
use rand::Rng;
use rusqlite::{Connection, Result};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "db"]
struct Asset;

/// Implementation struct
#[derive(Debug)]
pub struct FluentCodes {
    words: Vec<String>,
    connection: Option<Connection>,
    joiner: String,
    min_length: i32,
    max_length: i32,
}

/// Default trait implemented  for FluentCodes struct
impl Default for FluentCodes {
    fn default() -> Self {
        FluentCodes {
            words: vec![],
            connection: Option::None,
            joiner: "-".to_string(),
            min_length: 6,
            max_length: 6,
        }
    }
}

/// ToString trait implemented  for FluentCodes struct
impl ToString for FluentCodes {
    fn to_string(&self) -> String {
        return self.words.join(&self.joiner);
    }
}

/// code generation builder methods for FluentCodes struct
impl FluentCodes {
    pub fn with_joiner(&mut self, joiner: String) -> &mut FluentCodes {
        self.joiner = joiner;
        return self;
    }
    pub fn with_min_length(&mut self, length: i32) -> &mut FluentCodes {
        self.min_length = length;
        return self;
    }
    pub fn with_max_length(&mut self, length: i32) -> &mut FluentCodes {
        self.max_length = length;
        return self;
    }
}

impl FluentCodes {
    fn connection_check(&mut self) {
        if self.connection.is_none() {
            let path = "./db/words_release.db";
            self.connection = Connection::open(path).ok();
        }
    }
    fn select_word(&mut self, table: &str) {
        self.connection_check();
        let sql = format!(
            "SELECT LOWER(word) FROM {} where length(word) between {} and  {} \
                ORDER BY RANDOM() LIMIT 1",
            table, self.min_length, self.max_length
        );
        let val: Result<String, _> = self
            .connection
            .as_ref()
            .unwrap()
            .query_row(&sql, [], |row| row.get(0));
        self.words.push(val.unwrap());
    }
    pub fn adjective(&mut self) -> &mut FluentCodes {
        self.select_word("adj");
        return self;
    }

    pub fn adposition(&mut self) -> &mut FluentCodes {
        self.select_word("adp");
        return self;
    }
    pub fn adverb(&mut self) -> &mut FluentCodes {
        self.select_word("adv");
        return self;
    }
    pub fn auxiliary(&mut self) -> &mut FluentCodes {
        self.select_word("aux");
        return self;
    }
    pub fn coordinating_conjunction(&mut self) -> &mut FluentCodes {
        self.select_word("cconj");
        return self;
    }
    pub fn determiner(&mut self) -> &mut FluentCodes {
        self.select_word("det");
        return self;
    }
    pub fn interjection(&mut self) -> &mut FluentCodes {
        self.select_word("intj");
        return self;
    }
    pub fn noun(&mut self) -> &mut FluentCodes {
        self.select_word("noun");
        return self;
    }
    pub fn particle(&mut self) -> &mut FluentCodes {
        self.select_word("part");
        return self;
    }
    pub fn pronoun(&mut self) -> &mut FluentCodes {
        self.select_word("pron");
        return self;
    }
    pub fn proper_noun(&mut self) -> &mut FluentCodes {
        self.select_word("propn");
        return self;
    }
    pub fn punctuation(&mut self) -> &mut FluentCodes {
        self.select_word("punct");
        return self;
    }
    pub fn subordinating_conjunction(&mut self) -> &mut FluentCodes {
        self.select_word("sconj");
        return self;
    }
    pub fn symbol(&mut self) -> &mut FluentCodes {
        self.select_word("sym");
        return self;
    }
    pub fn verb(&mut self) -> &mut FluentCodes {
        self.select_word("verb");
        return self;
    }
    pub fn six_digits(&mut self) -> &mut FluentCodes {
        let mut rng = rand::thread_rng();
        self.words.push(format!("{:#06}", rng.gen_range(0..999999)));
        return self;
    }
    pub fn generate_code_with_four_words() -> String {
        FluentCodes::default()
            .adjective()
            .verb()
            .noun()
            .adjective()
            .to_string()
    }
    pub fn generate_code_with_three_words_and_six_digits() -> String {
        FluentCodes::default()
            .adjective()
            .verb()
            .noun()
            .six_digits()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::FluentCodes;

    #[test]
    fn print_codes() {
        FluentCodes::default();
        let val1 = FluentCodes::default()
            .adjective()
            .adposition()
            .adverb()
            .with_joiner("_".to_string())
            .to_string();
        println!("{}", val1);
        let val2 = FluentCodes::default()
            .adjective()
            .adposition()
            .adverb()
            .auxiliary()
            .coordinating_conjunction()
            .determiner()
            .interjection()
            .noun()
            .particle()
            .pronoun()
            .proper_noun()
            .punctuation()
            .subordinating_conjunction()
            .symbol()
            .verb()
            .six_digits()
            .to_string();
        println!("{}", val2);
        let val3 = FluentCodes::default()
            .with_max_length(8)
            .with_min_length(3)
            .with_joiner("..{-_-}..".to_string())
            .adjective()
            .adposition()
            .adverb()
            .auxiliary()
            .coordinating_conjunction()
            .determiner()
            .interjection()
            .noun()
            .particle()
            .pronoun()
            .proper_noun()
            .punctuation()
            .subordinating_conjunction()
            .symbol()
            .verb()
            .six_digits()
            .to_string();
        println!("{}", val3);
        let val4 = FluentCodes::default()
            .with_max_length(8)
            .with_min_length(3)
            .with_joiner("^".to_string())
            .adjective()
            .adposition()
            .adverb()
            .noun()
            .proper_noun()
            .verb()
            .six_digits()
            .to_string();
        println!("{}", val4);
        println!("{}", FluentCodes::generate_code_with_four_words());
        println!(
            "{}",
            FluentCodes::generate_code_with_three_words_and_six_digits()
        );
    }
}
