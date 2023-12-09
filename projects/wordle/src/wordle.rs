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
        if self.string.len() != word.string.len() {panic!("Words must match length")}
        
        let mut results: Vec<Comparison> = Vec::new();

        let mut word_a_iter = self.string.chars();
        let mut word_b_iter = word.string.chars();

        loop {
            let result_comparison: Comparison;

            let char_a = match word_a_iter.next() {Some(ch) => ch, None => break};
            let char_b = match word_b_iter.next() {Some(ch) => ch, None => break};
            
            if char_a == char_b { result_comparison = Comparison::Valid}
            else { if word.string.contains(char_a) { result_comparison = Comparison::Exists}
                else { result_comparison = Comparison::Invalid } }

            results.push(result_comparison)
        }

        results
    }
}

pub enum Comparison {
    Invalid, Exists, Valid,
}