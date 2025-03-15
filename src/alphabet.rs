use unicode_segmentation::UnicodeSegmentation;

pub struct Alphabet {
    graphemes: Vec<String>,
}

impl Alphabet {
    #[must_use]
    pub fn new(graphemes: &str) -> Self {
        let graphemes = graphemes.graphemes(true).map(String::from).collect();
        Self { graphemes }
    }

    #[must_use]
    pub fn grapheme_at(&self, index: usize) -> Option<&String> {
        self.graphemes.get(index)
    }

    #[must_use]
    pub fn index_of(&self, grapheme: &str) -> Option<usize> {
        self.graphemes.iter().position(|value| value == grapheme)
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.graphemes.len()
    }

    #[must_use]
    pub fn contains(&self, grapheme: &str) -> bool {
        self.index_of(grapheme).is_some()
    }
}
