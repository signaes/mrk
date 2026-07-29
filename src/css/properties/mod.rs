//! Property-value AST: [`Value`] enum and the [`define_property!`] macro.
//!
//! [`Value`] is the central enum that wraps every typed CSS value.
//! It provides `Display`, `From` impls for every value type, and
//! `pub(crate)` helpers for the renderer.

use std::borrow::Cow;
use std::fmt;

use crate::css::values::{
    Angle, Color, CssString, CustomProperty, EasingFunction, Frequency, Ident, Integer, Length,
    Number, Percentage, Resolution, Time, Url,
};

/// A typed CSS property value.
///
/// Every variant wraps a strongly-typed [`values`] entry.
/// `Value::Raw` is `pub(crate)` and used for unknown / custom
/// property values that bypass the type system.
#[derive(Debug, Clone)]
pub enum Value {
    /// Wraps a [`Color`] value.
    Color(Color),
    /// Wraps a [`Length`] value.
    Length(Length),
    /// Wraps a [`Percentage`] value.
    Percentage(Percentage),
    /// Wraps a [`Time`] value.
    Time(Time),
    /// Wraps an [`Angle`] value.
    Angle(Angle),
    /// Wraps a [`Frequency`] value.
    Frequency(Frequency),
    /// Wraps a [`Resolution`] value.
    Resolution(Resolution),
    /// Wraps a [`Number`] value.
    Number(Number),
    /// Wraps an [`Integer`] value.
    Integer(Integer),
    /// Wraps a [`CssString`] value.
    String(CssString),
    /// Wraps a [`Url`] value.
    Url(Url),
    /// Wraps an [`Ident`] value.
    Identifier(Ident),
    /// Wraps a [`CustomProperty`] value.
    CustomProperty(CustomProperty),
    /// Wraps an [`EasingFunction`] value.
    EasingFunction(EasingFunction),
    /// A functional notation: `name(args...)`.
    Function {
        /// Function name.
        name: Cow<'static, str>,
        /// Function arguments.
        args: Vec<Value>,
    },
    /// A space-separated value list.
    List(Vec<Value>),
    /// A raw CSS string (crate-internal escape hatch).
    Raw(Cow<'static, str>),
}

// ── From impls ──────────────────────────────────────────────────────

impl From<Color> for Value {
    fn from(v: Color) -> Self { Value::Color(v) }
}
impl From<Length> for Value {
    fn from(v: Length) -> Self { Value::Length(v) }
}
impl From<Percentage> for Value {
    fn from(v: Percentage) -> Self { Value::Percentage(v) }
}
impl From<Time> for Value {
    fn from(v: Time) -> Self { Value::Time(v) }
}
impl From<Angle> for Value {
    fn from(v: Angle) -> Self { Value::Angle(v) }
}
impl From<Frequency> for Value {
    fn from(v: Frequency) -> Self { Value::Frequency(v) }
}
impl From<Resolution> for Value {
    fn from(v: Resolution) -> Self { Value::Resolution(v) }
}
impl From<Number> for Value {
    fn from(v: Number) -> Self { Value::Number(v) }
}
impl From<Integer> for Value {
    fn from(v: Integer) -> Self { Value::Integer(v) }
}
impl From<CssString> for Value {
    fn from(v: CssString) -> Self { Value::String(v) }
}
impl From<Url> for Value {
    fn from(v: Url) -> Self { Value::Url(v) }
}
impl From<Ident> for Value {
    fn from(v: Ident) -> Self { Value::Identifier(v) }
}
impl From<CustomProperty> for Value {
    fn from(v: CustomProperty) -> Self { Value::CustomProperty(v) }
}
impl From<EasingFunction> for Value {
    fn from(v: EasingFunction) -> Self { Value::EasingFunction(v) }
}

impl From<&'static str> for Value {
    fn from(s: &'static str) -> Self {
        Value::Raw(Cow::Borrowed(s))
    }
}
impl From<f32> for Value {
    fn from(v: f32) -> Self { Value::Number(v.into()) }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self { Value::Number(v.into()) }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self { Value::Integer(v.into()) }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Raw(Cow::Owned(s))
    }
}

// ── Display ─────────────────────────────────────────────────────────

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Color(v) => fmt::Display::fmt(v, f),
            Value::Length(v) => fmt::Display::fmt(v, f),
            Value::Percentage(v) => fmt::Display::fmt(v, f),
            Value::Time(v) => fmt::Display::fmt(v, f),
            Value::Angle(v) => fmt::Display::fmt(v, f),
            Value::Frequency(v) => fmt::Display::fmt(v, f),
            Value::Resolution(v) => fmt::Display::fmt(v, f),
            Value::Number(v) => fmt::Display::fmt(v, f),
            Value::Integer(v) => fmt::Display::fmt(v, f),
            Value::String(v) => fmt::Display::fmt(v, f),
            Value::Url(v) => fmt::Display::fmt(v, f),
            Value::Identifier(v) => fmt::Display::fmt(v, f),
            Value::CustomProperty(v) => fmt::Display::fmt(v, f),
            Value::EasingFunction(v) => fmt::Display::fmt(v, f),
            Value::Function { name, args } => {
                let mut s = String::from(name.as_ref());
                s.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { s.push_str(", "); }
                    s.push_str(&arg.to_string());
                }
                s.push(')');
                f.write_str(&s)
            }
            Value::List(items) => {
                let mut s = String::new();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { s.push(' '); }
                    s.push_str(&item.to_string());
                }
                f.write_str(&s)
            }
            Value::Raw(s) => f.write_str(s),
        }
    }
}

impl Value {
    /// Render this value to a CSS string. Called by the pretty-printer.
    #[allow(dead_code)]
    pub(crate) fn into_string(self) -> String {
        self.to_string()
    }
}

/// Define a property setter on a builder type.
///
/// # Syntax
///
/// ```ignore
/// define_property!(RuleBuilder, "color" => color, "Set the foreground color.");
/// define_property!(RuleBuilder, "background" => background, "Set the background.", shorthand);
/// ```
///
/// The macro generates:
/// ```ignore
/// impl RuleBuilder {
///     #[doc = "Set the foreground color."]
///     pub fn color(self, value: impl Into<Value>) -> RuleBuilder {
///         self.decl(Declaration::new(Cow::Borrowed("color"), value.into()))
///     }
/// }
/// ```
///
/// When `shorthand` is present, the generated method accepts
/// `Into<Value>` but is marked with a doc-comment note that it
/// accepts a shorthand value. The expansion is otherwise identical.
#[macro_export]
macro_rules! define_property {
    ($builder:ident, $name:literal => $method:ident, $doc:literal) => {
        define_property!(@inner $builder, $name, $method, $doc, false);
    };
    ($builder:ident, $name:literal => $method:ident, $doc:literal, shorthand) => {
        define_property!(@inner $builder, $name, $method, $doc, true);
    };
    (@inner $builder:ident, $name:expr, $method:ident, $doc:expr, $_shorthand:expr) => {
        #[doc = $doc]
        pub fn $method(self, value: impl Into<$crate::css::Value>) -> $builder {
            self.decl($crate::css::Declaration::new(
                std::borrow::Cow::Borrowed($name),
                value.into(),
            ))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from_color() {
        let v: Value = Color::rgb(255, 0, 0).into();
        assert!(format!("{:?}", v).contains("Color("));
    }

    #[test]
    fn value_from_length() {
        let v: Value = Length::px(16.0).into();
        assert!(format!("{:?}", v).contains("Length("));
    }

    #[test]
    fn value_from_percentage() {
        let v: Value = Percentage::new(50.0).into();
        assert!(format!("{:?}", v).contains("Percentage("));
    }

    #[test]
    fn value_from_time() {
        let v: Value = Time::s(1.5).into();
        assert!(format!("{:?}", v).contains("Time("));
    }

    #[test]
    fn value_from_angle() {
        let v: Value = Angle::deg(45.0).into();
        assert!(format!("{:?}", v).contains("Angle("));
    }

    #[test]
    fn value_from_frequency() {
        let v: Value = Frequency::hz(100.0).into();
        assert!(format!("{:?}", v).contains("Frequency("));
    }

    #[test]
    fn value_from_resolution() {
        let v: Value = Resolution::dpi(96.0).into();
        assert!(format!("{:?}", v).contains("Resolution("));
    }

    #[test]
    fn value_from_number() {
        let v: Value = Number::new(1.5).into();
        assert!(format!("{:?}", v).contains("Number("));
    }

    #[test]
    fn value_from_integer() {
        let v: Value = Integer::new(42).into();
        assert!(format!("{:?}", v).contains("Integer("));
    }

    #[test]
    fn value_from_css_string() {
        let v: Value = CssString::new("hello").into();
        assert!(format!("{:?}", v).contains("String("));
    }

    #[test]
    fn value_from_url() {
        let v: Value = Url::local("style.css").into();
        assert!(format!("{:?}", v).contains("Url("));
    }

    #[test]
    fn value_from_ident() {
        let v: Value = Ident::from("auto").into();
        assert!(format!("{:?}", v).contains("Identifier("));
    }

    #[test]
    fn value_from_custom_property() {
        let v: Value = CustomProperty::new("--my-var").unwrap().into();
        assert!(format!("{:?}", v).contains("CustomProperty("));
    }

    #[test]
    fn value_from_easing() {
        let v: Value = EasingFunction::Ease.into();
        assert!(format!("{:?}", v).contains("EasingFunction("));
    }

    #[test]
    fn value_from_static_str() {
        let v: Value = Value::from("raw-value");
        assert!(format!("{:?}", v).contains("Raw("));
    }

    #[test]
    fn value_display_color() {
        assert_eq!(Value::Color(Color::named("red")).to_string(), "red");
    }

    #[test]
    fn value_display_length() {
        assert_eq!(Value::Length(Length::px(16.0)).to_string(), "16px");
    }

    #[test]
    fn value_display_percentage() {
        assert_eq!(Value::Percentage(Percentage::new(50.0)).to_string(), "50%");
    }

    #[test]
    fn value_display_function() {
        let v = Value::Function {
            name: Cow::Borrowed("var"),
            args: vec![Value::Identifier(Ident::from("--my-var"))],
        };
        assert_eq!(v.to_string(), "var(--my-var)");
    }

    #[test]
    fn value_display_function_multi_args() {
        let v = Value::Function {
            name: Cow::Borrowed("rgb"),
            args: vec![
                Value::Number(Number::new(255.0)),
                Value::Number(Number::new(0.0)),
                Value::Number(Number::new(0.0)),
            ],
        };
        assert_eq!(v.to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn value_display_list() {
        let v = Value::List(vec![
            Value::Length(Length::px(8.0)),
            Value::Length(Length::px(16.0)),
        ]);
        assert_eq!(v.to_string(), "8px 16px");
    }

    #[test]
    fn value_display_raw() {
        let v = Value::Raw(Cow::Borrowed("some-raw-value"));
        assert_eq!(v.to_string(), "some-raw-value");
    }

    #[test]
    fn value_into_string() {
        let v = Value::Color(Color::named("red"));
        assert_eq!(v.into_string(), "red");
    }

    #[test]
    fn value_from_f64() {
        let v: Value = 1.5f64.into();
        assert!(format!("{:?}", v).contains("Number("));
    }

    #[test]
    fn value_from_i32() {
        // Iterate over a mix of inputs so the same matches! line is
        // hit with both true (Integer from i32) and false (non-Integer
        // from str).
        let cases: [(Box<dyn Fn() -> Value>, bool); 4] = [
            (Box::new(|| 0i32.into()), true),
            (Box::new(|| 42i32.into()), true),
            (Box::new(|| "hello".into()), false),
            (Box::new(|| "world".into()), false),
        ];
        for (make, expected_int) in &cases {
            let v = make();
            let is_int = matches!(v, Value::Integer(_));
            assert_eq!(is_int, *expected_int);
        }
        let v: Value = 0i32.into();
        assert_eq!(v.to_string(), "0");
    }

    #[test]
    fn value_from_string() {
        let v: Value = String::from("hi").into();
        assert!(format!("{:?}", v).contains("Raw("));
    }

    #[test]
    fn value_display_time() {
        let v = Value::Time(Time::s(1.5));
        assert_eq!(v.to_string(), "1.5s");
    }

    #[test]
    fn value_display_angle() {
        let v = Value::Angle(Angle::deg(45.0));
        assert_eq!(v.to_string(), "45deg");
    }

    #[test]
    fn value_display_frequency() {
        let v = Value::Frequency(Frequency::hz(100.0));
        assert_eq!(v.to_string(), "100hz");
    }

    #[test]
    fn value_display_resolution() {
        let v = Value::Resolution(Resolution::dpi(96.0));
        assert_eq!(v.to_string(), "96dpi");
    }

    #[test]
    fn value_display_url() {
        let v = Value::Url(Url::local("foo.css"));
        assert_eq!(v.to_string(), "url(\"foo.css\")");
    }

    #[test]
    fn value_display_custom_property() {
        let v = Value::CustomProperty(CustomProperty::new("--my-var").unwrap());
        assert_eq!(v.to_string(), "--my-var");
    }

    #[test]
    fn value_display_easing() {
        let v = Value::EasingFunction(EasingFunction::linear());
        assert_eq!(v.to_string(), "linear");
    }
}
