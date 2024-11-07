use std::{collections::HashMap, fmt::Display, fs};

pub struct WordCloud {
    //                  Word
    //                    |   Frequency
    //                    V      V
    word_cloud: HashMap<String, i32>,
    total_weight: i32
}

pub struct WordScore {
    word_weight: i32,
    total_weight: i32,
    score_ratio: f32
}

impl WordScore {
    pub fn new(word_weight: i32, total_weight: i32) -> Self {
        Self {
            word_weight,
            total_weight,
            score_ratio: word_weight as f32 / total_weight as f32
        }
    }

    #[allow(unused)]
    pub fn get_score_ratio(&self) -> f32 {
        self.score_ratio
    }

    pub fn get_word_weight(&self) -> i32 {
        self.word_weight
    }
}

impl Display for WordScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Word Weight: {} / {} = {}", self.word_weight, self.total_weight, self.score_ratio)?;
        
        Ok(())
    }
}

impl WordCloud {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            word_cloud: HashMap::new(),
            total_weight: 0
        }
    }

    #[allow(unused)]
    pub fn create_from_text_file(filename: String) -> Self {
        // Reads from a file and removes extra spaces and new lines and then collects it into a vector
        let mut binding = fs::read_to_string(filename)
        .unwrap()
        .trim().to_string()
        .to_lowercase();

        binding.retain(|c| c.is_alphabetic() || c.is_whitespace());
        
        let contents = binding
            .split_whitespace()
            .collect::<Vec<&str>>();

        let mut string_contents: Vec<String> = Vec::new();

        for content in contents {
            string_contents.push(content.to_string());
        }

        let mut ret = Self::new();
        for string in string_contents {
            match ret.word_cloud.get_mut(&string) {
                Some(value) => {
                    *value += 1;
                },
                None => {
                    ret.word_cloud.insert(string, 1);
                },
            }
            ret.total_weight += 1;
        }

        ret
    }

    pub fn get_word_score(&self, string: String) -> WordScore {
        let mut score = 0;

        for word in string.split(" ").collect::<Vec<&str>>() {
            match self.word_cloud.get(&word.to_ascii_lowercase().to_string()) {
                Some(weight) => {
                    score += weight;
                },
                None => {},
            }
        }

        WordScore::new(score, self.total_weight)
    }
}

impl Display for WordCloud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Word Cloud:")?;
        writeln!(f, "    Total Weight: {}", self.total_weight)?;
        writeln!(f, "                    Word : Weight")?;
        for (word, weight) in self.word_cloud.iter() {
            writeln!(f, "        {:>16} : {}", word, weight)?;
        }

        Ok(())
    }
}
