use fast_aug::text::{BaseTextAugmenter, TextAction};
use fast_aug::text::RandomCharsAugmenter;
use fast_aug::text::TextAugmentParameters;


fn main() {
    let words = "Hello, world!".to_string();

    // Time it
    let start = std::time::Instant::now();

    let words_params = TextAugmentParameters::default();
    let chars_params = TextAugmentParameters::default();

    let aug = RandomCharsAugmenter::new(TextAction::Delete, words_params, chars_params, None);

    let output = aug.augment(words.clone());

    let elapsed = start.elapsed();

    println!("Input: {}", words);
    println!("Output: {}", output);
    println!("Time elapsed: {:?}", elapsed);
}
