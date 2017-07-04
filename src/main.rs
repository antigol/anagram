use std::env;
use std::ops::Add;
use std::ops::Sub;
use std::ops::SubAssign;
use std::io::{self, Read};

const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                              'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

#[derive(Debug, Clone, Copy)]
struct Occurrences {
    alphabet: [i32; 26],
}

impl Occurrences {
    fn from_string(s: &str) -> Occurrences {
        let mut occ = [0; 26];
        for (i, &a) in ALPHABET.iter().enumerate() {
            occ[i] = s.chars().filter(|&ch| ch == a).count() as i32;
        }
        Occurrences { alphabet: occ }
    }

    fn is_null(&self) -> bool {
        self.sum() == 0
    }

    fn is_nonnegative(&self) -> bool {
        for &x in self.alphabet.iter() {
            if x < 0 {
                return false;
            }
        }
        true
    }

    fn sum(&self) -> i32 {
        let mut s = 0;
        for &o in self.alphabet.iter() {
            s += o;
        }
        s
    }
}

impl Add for Occurrences {
    type Output = Occurrences;
    fn add(self, other: Occurrences) -> Occurrences {
        let mut occ = self.alphabet.clone();
        for (i, &x) in other.alphabet.iter().enumerate() {
            occ[i] += x;
        }
        Occurrences { alphabet: occ }
    }
}

impl Sub for Occurrences {
    type Output = Occurrences;
    fn sub(mut self, other: Occurrences) -> Occurrences {
        self -= other;
        self
    }
}

impl SubAssign for Occurrences {
    fn sub_assign(&mut self, other: Occurrences) {
        for (i, &x) in other.alphabet.iter().enumerate() {
            self.alphabet[i] -= x;
        }
    }
}

fn main() {
    let word = env::args().nth(1).unwrap().to_uppercase();

    let mut language = String::new();
    io::stdin().read_to_string(&mut language).unwrap();

    let language = language.split('\n').filter(|x| !x.is_empty()).map(|x| x.to_uppercase());

    let word_occ = Occurrences::from_string(&word);

    let mut incomplete_solutions: Vec<(Occurrences, Vec<String>)> = vec![(word_occ, Vec::new())];

    for lang_word in language {
        let lang_word_occ = Occurrences::from_string(&lang_word);

        assert!(!lang_word_occ.is_null(),
                format!("\"{}\" contains no letters : {:?}",
                        lang_word,
                        lang_word_occ));

        if !(word_occ - lang_word_occ).is_nonnegative() {
            continue;
        }

        let mut new_solutions = Vec::new();

        for &(mut remaining_occ, ref words) in incomplete_solutions.iter() {
            remaining_occ -= lang_word_occ;
            let mut n = 1;

            while remaining_occ.is_nonnegative() {
                let mut new_words = words.clone();
                new_words.append(&mut vec![lang_word.to_string(); n]);
                new_solutions.push((remaining_occ, new_words));

                remaining_occ -= lang_word_occ;
                n += 1;
            }
        }

        incomplete_solutions.append(&mut new_solutions);
    }

    for (remaining_occ, words) in incomplete_solutions {
        if remaining_occ.is_null() {
            println!("{}", words.join(" "));
        }
    }
}
