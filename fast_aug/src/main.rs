use fast_aug::models::text::AlphabetModel;

fn main() {
    let language_tag = "sr-Latn-ME";
    let start_time = std::time::Instant::now();
    let model = AlphabetModel::from_locale_str(&language_tag);
    let end_time = std::time::Instant::now();

    println!("Time elapsed: {:?}", end_time.duration_since(start_time));

    println!("main: {:?} {:?}", model.main.len(), model.main);
    println!("main_capitalized: {:?} {:?}", model.main_capitalized.len(), model.main_capitalized);
    println!("index: {:?} {:?}", model.index.len(), model.index);
    println!("auxiliary: {:?} {:?}", model.auxiliary.len(), model.auxiliary);
    println!("punctuation: {:?} {:?}", model.punctuation.len(), model.punctuation);
    println!("numbers: {:?} {:?}", model.numbers.len(), model.numbers);
}
