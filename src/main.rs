extern crate numeral;
extern crate rand;

use numeral::Numeral;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

const DAYS_OF_NOVEMBER: u32 = 32; // until we have inclusive ranges (...)
const DAILY_WORD_COUNT: u64 = 1666;


// copy pasta
fn dice<R: Rng>(faces: &Range<u32>, rng: &mut R) -> u32 {
    faces.ind_sample(rng)
}

fn get_sentence(word: &str) -> (String, u64) {

    let mut rng = rand::thread_rng();
    let faces = Range::new(1, 9);
    let num = dice(&faces, &mut rng);
    let sentence_sizes = match num {
        1 | 4 | 7 => 5,
        2 | 5 | 8 => 7,
        3 | 6 | 9 => 13,
        _ => 0,
    };

    let mut sentence = String::new();
    for i in 0..sentence_sizes {
        sentence.push_str(&format!("{} ", word));
    }
    sentence = sentence.trim().to_string();
    sentence.push_str(". ");

    (sentence, sentence_sizes)
}

fn main() {
    for day in 1..DAYS_OF_NOVEMBER {
        let day_ordinal = day.ordinal();
        let mut word_count = 0;

        let header = format!("# {}\n\n", day_ordinal);
        let mut page = String::new();

        word_count = word_count + 1;

        loop {
            if word_count > DAILY_WORD_COUNT {
                break;
            }
            // page.push_str(&format!("{} ", day_ordinal));
            // word_count = word_count + 1;
            let (sentence, size) = get_sentence(&day_ordinal);
            page.push_str(&sentence.to_string());
            word_count = word_count + size;

        }
        page = page.trim().to_string();
        // page.push_str(".\n");
        println!("{}{}\n", &header, &page);
    }
}
