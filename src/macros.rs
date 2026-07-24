/// Builds a `Vec<Node>` for use with `.children(...)`. Accepts `&'static str`,
/// `String`, and any element value (from `el(...)` or a factory).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let tree = el("p").children(nodes!["Hello, ", "world"]);
/// assert_eq!(tree.children.len(), 2);
/// ```
#[macro_export]
macro_rules! nodes {
    () => {
        ::std::vec::Vec::<$crate::Node>::new()
    };
    ($($child:expr),+ $(,)?) => {{
        let mut v: ::std::vec::Vec<$crate::Node> = ::std::vec::Vec::new();
        $(
            v.push(<_ as ::std::convert::Into<$crate::Node>>::into($child));
        )+
        v
    }};
}
