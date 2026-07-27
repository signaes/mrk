//! Property bag and value types supplied at render time.
//!
//! `Props` is the runtime input for `Component::render`. The set of
//! keys is determined by the rendered expression tree (the `Expr`
//! variant determines which props are required) but the *types* are
//! loose: `PropType::String`, `Number`, `Bool`, `List`, or `Dictionary`.

use std::borrow::Cow;
use std::collections::HashMap;

/// A bag of named typed values supplied at render time.
///
/// Backed by a `HashMap`; keys are typically borrowed from a serialized
/// form (the `.mrk` wire format uses `Cow<'static, str>` keys).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Props(pub HashMap<Cow<'static, str>, PropType>);

impl Props {
    /// Construct an empty `Props`.
    pub fn new() -> Self {
        Props(HashMap::new())
    }

    /// Insert or replace a typed value under `key`.
    pub fn insert(&mut self, key: impl Into<Cow<'static, str>>, value: PropType) {
        self.0.insert(key.into(), value);
    }

    /// Look up a typed value by key.
    pub fn get(&self, key: &str) -> Option<&PropType> {
        self.0.get(key)
    }

    /// `true` if there are no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<HashMap<Cow<'static, str>, PropType>> for Props {
    fn from(map: HashMap<Cow<'static, str>, PropType>) -> Self {
        Props(map)
    }
}

impl FromIterator<(Cow<'static, str>, PropType)> for Props {
    fn from_iter<I: IntoIterator<Item = (Cow<'static, str>, PropType)>>(iter: I) -> Self {
        Props(iter.into_iter().collect())
    }
}

/// All possible prop value types.
///
/// Variants are an open enumeration; new types can be added without
/// breaking the wire format. Each variant has a stable
/// [`type_name`](PropType::type_name) for error messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropType {
    /// A string value. Rendered as `Node::Text` when substituted
    /// into text positions.
    String(Cow<'static, str>),
    /// A numeric value with preserved textual form.
    Number(Number),
    /// A boolean. Used by `Either` and `Maybe` conditions.
    Bool(bool),
    /// A homogeneous list of prop values. Used by `Map` iteration.
    List(Vec<PropType>),
    /// A string-keyed map of prop values.
    Dictionary(HashMap<Cow<'static, str>, PropType>),
}

impl PropType {
    /// Render this prop as a text representation (used when substituting
    /// into `Expr::Prop` positions).
    ///
    /// Dictionaries are summarized as `<dict N keys>` to keep the
    /// representation predictable and bounded.
    pub fn to_text(&self) -> Cow<'static, str> {
        match self {
            PropType::String(s) => s.clone(),
            PropType::Number(n) => n.repr.clone(),
            PropType::Bool(b) => Cow::Owned(b.to_string()),
            PropType::List(items) => Cow::Owned(
                items
                    .iter()
                    .map(|p| p.to_text())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            PropType::Dictionary(map) => Cow::Owned(format!("<dict {} keys>", map.len())),
        }
    }

    /// Stable type name for error messages. Used by `RenderError`.
    pub fn type_name(&self) -> &'static str {
        match self {
            PropType::String(_) => "string",
            PropType::Number(_) => "number",
            PropType::Bool(_) => "bool",
            PropType::List(_) => "list",
            PropType::Dictionary(_) => "dictionary",
        }
    }
}

/// A numeric prop that preserves the original textual form.
///
/// `repr` is the exact string the value was constructed with, so
/// round-tripping through the wire format never changes how the
/// number renders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// Original textual form (e.g. `"42"`, `"3.14"`).
    pub repr: Cow<'static, str>,
    /// Integer or float kind tag.
    pub kind: NumberKind,
}

impl Number {
    /// Integer literal. The string is kept as-is.
    pub fn int(repr: impl Into<Cow<'static, str>>) -> Self {
        Number {
            repr: repr.into(),
            kind: NumberKind::Int,
        }
    }

    /// Floating-point literal. The string is kept as-is.
    pub fn float(repr: impl Into<Cow<'static, str>>) -> Self {
        Number {
            repr: repr.into(),
            kind: NumberKind::Float,
        }
    }

    /// Try to parse the number as `i64`.
    pub fn parse_i64(&self) -> Option<i64> {
        self.repr.parse().ok()
    }

    /// Try to parse the number as `f64`.
    pub fn parse_f64(&self) -> Option<f64> {
        self.repr.parse().ok()
    }
}

/// Integer vs float kind for [`Number`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    /// Integer (e.g. `"42"`).
    Int,
    /// Floating-point (e.g. `"3.14"`).
    Float,
}

impl NumberKind {
    /// Wire-format tag: `"i"` or `"f"`.
    pub fn tag(self) -> &'static str {
        match self {
            NumberKind::Int => "i",
            NumberKind::Float => "f",
        }
    }

    /// Parse a wire-format tag. Returns `None` if `s` is unknown.
    pub fn from_tag(s: &str) -> Option<Self> {
        match s {
            "i" => Some(NumberKind::Int),
            "f" => Some(NumberKind::Float),
            _ => None,
        }
    }
}
