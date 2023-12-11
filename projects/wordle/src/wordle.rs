#[derive(PartialEq)]
pub struct Word {
    pub string: String,
}

impl Word {
    pub fn from_string(string: &String) -> Self {
        Word {
            string: string.clone()
        }
    }
    pub fn compare_to(&self, word: &Word) -> Vec<Comparison> {

        let mut results: Vec<Comparison> = Vec::new();
        let mut word_a_iter = self.string.chars();
        let mut word_b_iter = word.string.chars();
        let mut remainder_chars: Vec<char> = word.string.chars().collect();

        loop {
            let current_comparison: Comparison;

            let char_a = match word_a_iter.next() {Some(ch) => ch, None => break};
            let char_b = match word_b_iter.next() {Some(ch) => ch, None => break};
            
            if match remainder_chars.iter().position(|&e| e == char_a) {
                Some(index) => {remainder_chars.remove(index); true},
                None => false
            } {
                if char_a == char_b {
                    current_comparison = Comparison::Valid
                } else {
                    current_comparison = Comparison::Exists
                }
            } else {
                current_comparison = Comparison::Invalid
            }

            results.push(current_comparison)
        }

        return results
    }
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Invalid, Exists, Valid
}
