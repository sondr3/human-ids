#![doc = include_str!("../README.md")]
mod constants;

pub use constants::{ADJECTIVES, ADVERBS, NOUNS, VERBS};

/// Trait for selecting elements from a collection.
pub trait Selector<'a> {
    /// Returns a random element from the collection.
    ///
    /// ## Example
    /// ```
    /// # use human_ids::Selector;
    /// let vec = vec!["a", "b", "c"];
    /// assert!(vec.contains(&vec.random()));
    /// ```
    fn random(&self) -> &'a str;

    /// Returns the longest element from the collection.
    ///
    /// ## Example
    /// ```
    /// # use human_ids::Selector;
    /// let vec = vec!["a", "ab", "abc"];
    /// assert_eq!(vec.longest(), "abc");
    /// ```
    fn longest(&self) -> &'a str;

    /// Returns the shortest element from the collection.
    ///
    /// ## Example
    /// ```
    /// # use human_ids::Selector;
    /// let vec = vec!["a", "ab", "abc"];
    /// assert_eq!(vec.shortest(), "a");
    /// ```
    fn shortest(&self) -> &'a str;
}

impl<'a> Selector<'a> for [&'a str] {
    #[inline]
    fn random(&self) -> &'a str {
        let index = fastrand::usize(..self.len());
        self[index]
    }

    fn longest(&self) -> &'a str {
        self.iter().max_by_key(|s| s.len()).unwrap_or(&"")
    }

    fn shortest(&self) -> &'a str {
        self.iter().min_by_key(|s| s.len()).unwrap_or(&"")
    }
}

/// Trait for capitalizing the first letter of a string.
trait Capitalize {
    /// Capitalizes the first letter of the string.
    ///
    /// ## Example
    /// ```
    /// let s = "hello";
    /// assert_eq!(s.capitalize(), "Hello");
    /// ```
    fn capitalize(&self) -> String;
}

impl Capitalize for &str {
    #[inline]
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        chars.next().map_or_else(String::new, |c| {
            c.to_uppercase().collect::<String>() + chars.as_str()
        })
    }
}

#[derive(Debug)]
/// Options for generating a human readable identifier.
pub struct Options<'a> {
    /// Separator to use between words.
    pub separator: Option<&'a str>,
    /// Whether to capitalize each word.
    pub capitalize: bool,
    /// Whether to add an adverb to the end of the identifier.
    pub add_adverb: bool,
    /// Number of adjectives to use.
    pub adjective_count: usize,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            separator: Some("-"),
            capitalize: true,
            add_adverb: false,
            adjective_count: 1,
        }
    }
}

impl<'a> Options<'a> {
    /// Creates a new instance of `Options`.
    ///
    /// ## Example
    /// ```
    /// use human_ids::Options;
    ///
    /// let options = Options::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the separator to use between words.
    ///
    /// ## Example
    /// ```
    /// use human_ids::Options;
    ///
    /// let options = Options::new().separator("-");
    /// ```
    pub const fn separator(mut self, separator: &'a str) -> Self {
        self.separator = Some(separator);
        self
    }

    /// Sets whether to capitalize each word.
    ///
    /// ## Example
    /// ```
    /// use human_ids::Options;
    ///
    /// let options = Options::new().capitalize(true);
    /// ```
    pub const fn capitalize(mut self, capitalize: bool) -> Self {
        self.capitalize = capitalize;
        self
    }

    /// Sets whether to add an adverb to the end of the identifier.
    ///
    /// ## Example
    /// ```
    /// use human_ids::Options;
    ///
    /// let options = Options::new().add_adverb(true);
    /// ```
    pub const fn add_adverb(mut self, add_adverb: bool) -> Self {
        self.add_adverb = add_adverb;
        self
    }

    /// Sets the number of adjectives to use.
    ///
    /// ## Example
    /// ```
    /// use human_ids::Options;
    ///
    /// let options = Options::new().adjective_count(2);
    /// ```
    pub const fn adjective_count(mut self, adjective_count: usize) -> Self {
        self.adjective_count = adjective_count;
        self
    }
}

#[derive(Debug)]
/// Structure for generating human readable identifiers.
pub struct HumanId<'a> {
    options: Options<'a>,
}

impl<'a> HumanId<'a> {
    /// Creates a new `HumanId` instance.
    ///
    /// ## Example
    ///
    /// ```
    /// use human_ids::HumanId;
    ///
    /// let human_id = HumanId::new(None);
    /// ```
    /// Or
    /// ```
    /// use human_ids::{HumanId, Options};
    ///
    /// let options = Options::new();
    /// let human_id = HumanId::new(Some(options));
    /// ```
    pub fn new(options: Option<Options<'a>>) -> Self {
        Self {
            options: options.unwrap_or_default(),
        }
    }

    /// Generates a human readable identifier.
    ///
    /// ## Example
    ///
    /// ```
    /// use human_ids::HumanId;
    ///
    /// let id = HumanId::new(None).generate();
    /// ```
    pub fn generate(&self) -> String {
        let options = &self.options;

        let mut words = Vec::with_capacity(
            options.adjective_count + 2 + if options.add_adverb { 1 } else { 0 },
        );

        for _ in 0..options.adjective_count {
            words.push(ADJECTIVES.random());
        }

        words.push(NOUNS.random());
        words.push(VERBS.random());

        if options.add_adverb {
            words.push(ADVERBS.random());
        }

        let words: Vec<_> = if options.capitalize {
            words.iter().map(|s| s.capitalize()).collect()
        } else {
            words.into_iter().map(String::from).collect()
        };

        options
            .separator
            .map_or_else(|| words.join("-"), |sep| words.join(sep))
    }
}

impl std::fmt::Display for HumanId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let vec = vec!["a", "b", "c"];
        assert!(vec.contains(&vec.random()));
    }

    #[test]
    fn test_longest() {
        let vec = vec!["a", "ab", "abc"];
        assert_eq!(vec.longest(), "abc");
    }

    #[test]
    fn test_shortest() {
        let vec = vec!["a", "ab", "abc"];
        assert_eq!(vec.shortest(), "a");
    }

    #[test]
    fn test_capitalize() {
        let s = "hello";
        assert_eq!(s.capitalize(), "Hello");
    }

    #[test]
    fn test_generate() {
        let id = HumanId::new(None).generate();

        println!("{}", id);
        assert!(!id.is_empty());
    }

    #[test]
    fn test_generate_with_options() {
        let options = Options::new()
            .separator("_")
            .capitalize(true)
            .add_adverb(true)
            .adjective_count(2);

        let id = HumanId::new(Some(options)).generate();

        println!("{}", id);
        assert!(!id.is_empty());
        assert!(id.contains("_"));
    }

    #[test]
    fn test_display() {
        let id = HumanId::new(None).to_string();

        println!("{}", id);
        assert!(!id.is_empty());
    }
}
