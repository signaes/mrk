/// Builds a `Vec<Node>` for use with `.children(...)`. Accepts `&'static str`,
/// `String`, and any element value (from `el(...)` or a factory).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let html = div().children(nodes![
///     "Hello, ",
///     el("strong").children(nodes!["world"]),
/// ]).render();
///
/// assert_eq!(html, "<div>Hello, <strong>world</strong></div>");
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
