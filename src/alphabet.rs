use unicode_segmentation::UnicodeSegmentation;

pub struct Alphabet {
    graphemes: Vec<String>,
}

impl Alphabet {
    pub fn new(graphemes: String) -> Alphabet {
        let graphemes = graphemes.graphemes(true).map(String::from).collect();
        Alphabet { graphemes }
    }

    pub fn grapheme_at(&self, index: usize) -> Option<&String> {
        self.graphemes.get(index)
    }

    pub fn index_of(&self, grapheme: &str) -> Option<usize> {
        self.graphemes.iter().position(|value| value == grapheme)
    }

    pub fn length(&self) -> usize {
        self.graphemes.len()
    }

    pub fn contains(&self, grapheme: &str) -> bool {
        self.index_of(grapheme).is_some()
    }
}
