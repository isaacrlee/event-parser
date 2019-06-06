// Recognize

// /// An approximate parsing result:
// struct Recognized<'a, T> {
//     value: T,
//     confidence: f32,
//     rest: &'a str,
// }

/// An interface for dealing with parsing slices into an abstract syntax.
pub trait Recognizable: Sized {
    type Error: std::error::Error;

    fn recognize(text: &str) -> Result<Option<Self>, Self::Error>;

    fn describe() -> &'static str;
}
