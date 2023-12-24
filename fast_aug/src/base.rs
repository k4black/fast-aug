

pub trait BaseAugmenter<T> {
    fn augment(&self, input: T) -> T;
}
