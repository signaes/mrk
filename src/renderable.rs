/// Types that can be rendered to an HTML string.
///
/// Implement this trait to make your own types renderable via [`render`].
///
/// # Why no `Display` blanket impl
///
/// `Renderable` is deliberately *not* bridged to
/// [`fmt::Display`](std::fmt::Display): doing so would make
/// `format!("{}", el)` produce the HTML rendering instead of the
/// derived `Debug` repr, which would silently change the meaning of
/// `Display` for those types. Use [`render()`] (or `.render()`) when
/// you want HTML, and `format!("{:?}", el)` for the struct repr.
/// For the `.mrk` wire format, see the
/// [`mrk-ir`](https://crates.io/crates/mrk-ir) crate.
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

/// References render through to the underlying value, so `&sheet`
/// can be passed anywhere a [`Renderable`] is expected.
impl<T: Renderable + ?Sized> Renderable for &T {
    fn render(&self) -> String {
        (**self).render()
    }
}

/// Boxes render through to the underlying value, so
/// `Box<dyn Renderable>` can be passed anywhere a [`Renderable`] is
/// expected (useful for heterogeneous collections).
impl<T: Renderable + ?Sized> Renderable for Box<T> {
    fn render(&self) -> String {
        (**self).render()
    }
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

    #[test]
    fn references_render_through() {
        let s = Single("hi");
        assert_eq!(render(&s), "hi");
        let r: &Single = &s;
        assert_eq!(r.render(), "hi");
    }

    #[test]
    fn boxes_render_through() {
        let b: Box<dyn Renderable> = Box::new(Single("boxed"));
        assert_eq!(render(b), "boxed");
        let b = Box::new(Pair("a", "b"));
        assert_eq!(b.render(), "a-b");
    }
}
