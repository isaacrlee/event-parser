/// An interface for dealing with parsing unstructured text. Implement this trait for your abstract syntax when parsing.
pub trait Recognizable: Sized {
    /// Takes unstructed text, and returns an instance of the abstract syntax if a match is found.
    fn recognize(text: &str) -> Option<Self>;

    /// Returns a string to describe the abstract syntax.
    fn describe() -> &'static str;
}
