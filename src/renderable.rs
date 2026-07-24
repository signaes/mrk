/// Types that can be rendered to an HTML string.
///
/// Implement this trait to make your own types renderable via [`render`].
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// struct Page(&'static str);
///
/// impl Renderable for Page {
///     fn render(&self) -> String {
///         format!("<title>{}</title>", self.0)
///     }
/// }
///
/// assert_eq!(render(Page("Home")), "<title>Home</title>");
/// ```
pub trait Renderable {
    /// Returns the HTML representation of this value.
    fn render(&self) -> String;
}

/// Renders any [`Renderable`] type to its HTML string.
#[allow(dead_code)]
pub fn render(t: impl Renderable) -> String {
    t.render()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Single(&'static str);
    impl Renderable for Single {
        fn render(&self) -> String {
            self.0.to_string()
        }
    }

    struct Pair(&'static str, &'static str);
    impl Renderable for Pair {
        fn render(&self) -> String {
            format!("{}-{}", self.0, self.1)
        }
    }

    #[test]
    fn render_table() {
        let cases = [
            ("simple", render(Single("hi")), "hi"),
            ("empty", render(Single("")), ""),
            ("pair_combined", render(Pair("a", "b")), "a-b"),
            ("pair_other", render(Pair("x", "y")), "x-y"),
        ];

        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }
}
