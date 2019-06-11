// Recognize

// /// An approximate parsing result:
// struct Recognized<'a, T> {
//     value: T,
//     confidence: f32,
//     rest: &'a str,
// }

/// An interface for dealing with parsing unstructured text.
pub trait Recognizable: Sized {
    fn recognize(text: &str) -> Option<Self>;

    fn describe() -> &'static str;
}
