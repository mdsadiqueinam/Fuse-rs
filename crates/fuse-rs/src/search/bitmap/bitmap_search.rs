use crate::FuseOptions;
use std::{borrow::Cow, collections::HashMap};
use crate::helpers::str_ext::StrExt;
use crate::search::bitmap::constants::MAX_BITS;
use crate::search::bitmap::create_pattern_alphabet::create_pattern_alphabet;
use crate::search::bitmap::search::{search, SearchResult};

struct PatternChunk {
    /// The pattern segment
    pattern: String,
    
    /// The alphabet bitmap for this chunk
    alphabet: HashMap<char, u32>,
    
    /// The starting index of this chunk in the original pattern
    start_index: usize,
}

pub struct BitmapSearch<'a> {
    pattern: String,

    options: Cow<'a, FuseOptions<'a>>,

    chunks: Vec<PatternChunk>,
}

impl<'a> BitmapSearch<'a> {
    pub fn new(pattern: Cow<'a, str>, options: Cow<'a, FuseOptions<'a>>) -> Self {
        let mut new_pattern = if options.is_case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };
        
        if options.ignore_diacritics {
            new_pattern = new_pattern.strip_diacritics();
        }
        
        let mut bitmap_search = Self {
            pattern: new_pattern,
            options,
            chunks: Vec::new(),
        };

        // Initialize the bitmap search with the pattern
        if bitmap_search.pattern.is_empty() {
            return bitmap_search;
        }

        bitmap_search.add_chunk_from_pattern();
        
        bitmap_search
    }

    fn add_chunk_from_pattern(&mut self) {
        let len = self.pattern.len();

        if len > MAX_BITS {
            let remainder = len % MAX_BITS;
            let end = len - remainder;

            for i in (0..end).step_by(MAX_BITS) {
                // substr pattern from i to i + MAX_BITS
                let chunk = self.pattern[i..i + MAX_BITS].to_string();
                self.add_chunk(&chunk, i);
            }

            if remainder > 0 {
                let start_index = len - MAX_BITS;
                let chunk = self.pattern[start_index..].to_string();
                self.add_chunk(&chunk, start_index);
            }
        } else {
            // If the pattern is less than or equal to MAX_BITS, add it as a single chunk
            self.add_chunk(&self.pattern.to_string(), 0);
        }
    }

    fn add_chunk(&mut self, pattern: &str, start_index: usize) {
        let alphabet = create_pattern_alphabet(pattern);
        let chunk = PatternChunk {
            pattern: pattern.to_string(),
            alphabet,
            start_index,
        };
        self.chunks.push(chunk);
    }

    fn search_in(&self, text: &str) -> SearchResult {
        let mut text = if self.options.is_case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        text = if self.options.ignore_diacritics {
            text.strip_diacritics()
        } else {
            text
        };

        if text == self.pattern {
            return SearchResult {
                is_match: true,
                score: 0f64,
                indices: if self.options.include_matches {
                    Some(vec![(0, text.len() - 1)])
                } else {
                    None
                }
            }
        };

        let mut all_indices = Vec::new();
        let mut total_score = 0f64;
        let mut has_matches = false;

        self.chunks.iter().for_each(|chunk| {
            // Create a new FuseOptions with updated location for this chunk
            let mut chunk_options = FuseOptions {
                location: self.options.location + chunk.start_index,
                ..(*self.options).clone()
            };
            let result = search(&text, &chunk.pattern, &chunk.alphabet, &chunk_options);

            match result { 
                Ok(val) => {
                    has_matches = val.is_match;
                    total_score += val.score;
                    
                    if val.is_match && val.indices.is_some() {
                        let indices = val.indices.unwrap();
                        all_indices.extend(indices);
                    }
                },
                Err(_) => {
                    // Handle error if needed
                    return;
                }
            }
        });

        SearchResult {
            is_match: has_matches,
            score: if has_matches { total_score / (self.chunks.len() as f64) } else { 1f64 },
            indices: if self.options.include_matches && has_matches {
                Some(all_indices)
            } else {
                None
            }
        }
    }
}
