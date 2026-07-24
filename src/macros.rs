/// Builds a `Vec<Node>` for use with `.children(...)`, accepting any mix of
/// `text(...)` and `el(...)` / factory results. Element values are auto-wrapped
/// as nodes.
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let html = div().children(children![
///     text("Hello, "),
///     el("strong").children(children![text("world")]),
/// ]).render();
///
/// assert_eq!(html, "<div>Hello, <strong>world</strong></div>");
/// ```
#[macro_export]
macro_rules! children {
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
