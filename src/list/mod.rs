use crate::{Memory, Words};

pub struct List {
    major: Memory,
}

impl List {
    pub fn parse(list: &str) -> Self {
        let mut major = Memory::new();

        for line in list.lines() {
            let parts: Vec<&str> = line.split(" - ").collect();
            if parts.len() != 2 {
                println!("Skipping line: {}", line);
                continue;
            }

            let number: usize = parts[0].parse().expect("valid usize");
            let words: Words = parts[1]
                .split("/")
                .map(|word| word.trim().to_string())
                .collect();

            let _ = major.insert(number, words);
        }

        Self { major }
    }

    pub fn size(&self) -> usize {
        self.major.len()
    }

    pub fn keys(&self) -> Vec<usize> {
        self.major.keys().cloned().collect()
    }

    pub fn memorize_words(&self, number: usize) -> Words {
        self.major
            .get(&number)
            .expect(&format!("memorize_words list [{}] index not found", number))
            .iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>()
    }
}
