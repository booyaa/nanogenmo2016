extern crate numeral;

use numeral::Numeral;

const DAYS_OF_NOVEMBER: u32 = 32; // until we have inclusive ranges (...)
const DAILY_WORD_COUNT: u64 = 1666;

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
            page.push_str(&format!("{} ", day_ordinal));
            word_count = word_count + 1;
        }
        page = page.trim().to_string();
        page.push_str(".\n");
        println!("{}{}", &header, &page);
    }
}
