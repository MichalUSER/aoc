pub struct Password {
    pub text: String,
    pub least_count: u8,
    pub most_count: u8,
    pub letter: char
}

fn count_char(c: char, text: &String) -> u8 {
    let mut counter = 0;
    for i in text.chars() {
        if i == c {
            counter += 1;
        }
    }
    return counter;
}

impl Password {
    #[allow(dead_code)]
    pub fn validate_password1(&self) -> bool {
        let count = count_char(self.letter, &self.text);
        if count >= self.least_count && count <= self.most_count {
            return true;
        } else {
            return false;
        }
    }

    pub fn validate_password2(&self) -> Option<bool> {
        let text_chars: Vec<char> = self.text.chars().collect();
        let first = text_chars.get(self.least_count as usize - 1);
        let last = text_chars.get(self.most_count as usize - 1);
        let first = match first {
            Some(first) => first,
            None => return None
        };
        let last = match last {
            Some(last) => last,
            None => return None
        };
        if *first == self.letter && *last == self.letter {
            return Some(false);
        } else if *first == self.letter || *last == self.letter {
            return Some(true);
        } else {
            return Some(false);
        } 
    }
}
