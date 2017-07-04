use std::env;
use std::ops::Add;
use std::ops::Sub;
use std::ops::SubAssign;
use std::io::{self, Read};
use std::cmp::Ordering;

const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                              'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Occurrences {
    alphabet: [i32; 26],
}

impl Occurrences {
    fn null() -> Occurrences {
        Occurrences { alphabet: [0; 26] }
    }

    fn from_string(s: &str) -> Occurrences {
        let mut occ = [0; 26];
        for (i, &a) in ALPHABET.iter().enumerate() {
            occ[i] = s.chars().filter(|&ch| ch == a).count() as i32;
        }
        Occurrences { alphabet: occ }
    }
}

/* Définition d'un ordre partiel pour les Occurrences de lettres de mots
De la même manière que l'inclusion d'ensembles.
Un mot X est strictement inclus dans un autre Y si
    on peut écrire X avec les lettres de Y mais pas inversement.
Cet ordre est partiel car (A n'est pas strictement
    inclus dans B) n'implique pas que (B est strictement inclus dans A)
*/
impl PartialOrd for Occurrences {
    fn partial_cmp(&self, other: &Occurrences) -> Option<Ordering> {
        let mut exists_less = false;
        let mut exists_greater = false;
        for i in 0..self.alphabet.len() {
            match self.alphabet[i].cmp(&other.alphabet[i]) {
                Ordering::Equal => (),
                Ordering::Less => exists_less = true,
                Ordering::Greater => exists_greater = true,
            }
        }
        if exists_less && !exists_greater {
            Some(Ordering::Less)
        } else if exists_greater && !exists_less {
            Some(Ordering::Greater)
        } else if !exists_less && !exists_greater {
            Some(Ordering::Equal)
        } else {
            None
        }
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

        assert!(lang_word_occ > Occurrences::null(),
                format!("\"{}\" contains no letters : {:?}",
                        lang_word,
                        lang_word_occ));

        if !(lang_word_occ <= word_occ) {
            continue;
        }

        let mut new_solutions = Vec::new();

        for &(mut remaining_occ, ref words) in incomplete_solutions.iter() {
            let mut new_words = words.clone();

            while lang_word_occ <= remaining_occ {
                remaining_occ -= lang_word_occ;
                new_words.push(lang_word.clone());

                new_solutions.push((remaining_occ, new_words.clone()));
            }
        }

        incomplete_solutions.append(&mut new_solutions);
    }

    for (remaining_occ, words) in incomplete_solutions {
        if remaining_occ == Occurrences::null() {
            println!("{}", words.join(" "));
        }
    }
}
