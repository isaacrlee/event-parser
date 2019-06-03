// Recognize

// /// An approximate parsing result:
// struct Recognized<'a, T> {
//     value: T,
//     confidence: f32,
//     rest: &'a str,
// }

/// Trait for types that can be parsed.
pub trait Recognizable: Sized {
    type Error: std::error::Error;

    fn recognize(text: &str) -> Result<Option<Self>, Self::Error>;

    fn describe() -> &'static str;
}
