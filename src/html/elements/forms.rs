//! Form elements (`<form>`, `<input>`, `<button>`, `<select>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlForm, "form", all,
    action(r#"URL to which the form will be submitted.

If omitted, the form is submitted to the current document URL."#),
    method(r#"HTTP method to use when submitting the form.

One of:
- `get` (default; encoded into the URL as a query string)
- `post` (sent in the request body)
- `dialog` (closes the containing `<dialog>` and submits the form, but does not submit in the usual sense)

Tokens are case-insensitive."#),
    enctype(r#"Encoding type for form data when `method="post"`.

One of:
- `application/x-www-form-urlencoded` (default)
- `multipart/form-data` (required when uploading files via `<input type="file">`)
- `text/plain` (not recommended; not interoperable)"#),
    target(r#"Browsing context in which to display the form submission response.

One of:
- `_self` (default)
- `_blank`
- `_parent`
- `_top`
- a navigable target name"#),
    autocomplete(r#"Whether the user agent is allowed to autofill the form's controls.

One of:
- `on` (allow autofill; default)
- `off` (do not allow autofill)"#),
    novalidate(r#"Boolean attribute. When present, the form is not validated before submission.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    name(r#"Name of the form.

Used to identify the form in the document; also exposed on `document.forms`."#),
    accept_charset(r#"Space- and/or comma-separated list of character encodings the server accepts.

Example: `UTF-8 ISO-8859-1`. UTF-8 is implied; the attribute is rarely used in practice."#),
    rel(r#"Relationship between the current document and the form's submission target.

A space-separated list of link types. Currently meaningful for forms whose submission navigates to a URL (e.g. `opener`, `noopener`, `noreferrer`)."#));
define_html_element!(HtmlLabel, "label", all,
    for_attr(r#"ID of the form control with which the label is associated.

The label's activation (click, focus) targets the labeled control. Exactly one labeled control per label."#),
    form_attr(r#"ID of the `<form>` element the label is associated with.

Used when the label is not a descendant of its form."#));
define_html_element!(HtmlInput, "input", all,
    type_attr(r#"Type of input control.

One of:
- `hidden` (no displayed control, value sent with the form)
- `text` (default; single-line text)
- `search` (search-style text)
- `tel` (telephone number)
- `url` (absolute URL)
- `email` (email address or list)
- `password` (masked text)
- `date` (year-month-day)
- `month` (year-month)
- `week` (year-week)
- `time` (HH:MM)
- `datetime-local` (year-month-day HH:MM, no timezone)
- `number` (numeric value)
- `range` (numeric value in a slider)
- `color` (hex color)
- `checkbox` (zero or one selected)
- `radio` (one selected among a group sharing `name`)
- `file` (one or more file uploads)
- `submit` (form submission button)
- `image` (image submission button; uses `src`/`alt`)
- `reset` (form reset button)
- `button` (push button with no default behavior)

Custom values are allowed; the input is treated as `text` until the user agent recognizes the value."#),
    name(r#"Name of the control, submitted with the form as part of the name/value pair.

For grouped radio buttons, every button in the group shares the same `name`."#),
    value(r#"Current value of the control.

Sent as the value of the name/value pair on submission. For `<input type="checkbox">` and `<input type="radio">`, the value is sent only when the control is checked."#),
    placeholder(r#"Short hint displayed inside the control when its value is empty.

Not a substitute for a `<label>`. Line breaks are not rendered."#),
    required(r#"Boolean attribute. When present, the control must have a non-empty value for the form to be submitted.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    disabled(r#"Boolean attribute. When present, the control is non-interactive and is not submitted with the form.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Disabled controls are excluded from the submitted data."#),
    readonly(r#"Boolean attribute. When present, the control is not editable but is still focusable and submitted with the form.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Applies to `text`, `search`, `url`, `tel`, `email`, `password`, `date`, `month`, `week`, `time`, `datetime-local`, and `number`."#),
    checked(r#"Boolean attribute. When present, the checkbox or radio button is initially checked.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Applies to `checkbox` and `radio`."#),
    maxlength(r#"Maximum number of characters the value may contain (a valid non-negative integer).

Applies to text-entry types: `text`, `search`, `url`, `tel`, `email`, `password`, and `textarea`."#),
    minlength(r#"Minimum number of characters required for the value to be valid (a valid non-negative integer).

Applies to the same text-entry types as `maxlength`. The form is invalid if the value is shorter."#),
    pattern(r#"Regular expression the value must match for the control to be valid.

A JavaScript-style regex without leading/trailing slashes (e.g. `[a-z]+`). Compile with anchors implied: the entire value must match."#),
    size(r#"Display width in characters (a valid non-negative integer; default `20`).

Sets the visible width of the control, not a limit on value length. Applies to text-entry types."#),
    min(r#"Lower bound for the value.

For numeric types (`number`, `range`): the minimum numeric value.
For date/time types: the minimum date/time as a valid date or time string."#),
    max(r#"Upper bound for the value.

For numeric types: the maximum numeric value.
For date/time types: the maximum date/time as a valid date or time string."#),
    step(r#"Granularity of acceptable values (a positive number or `any`).

Default is `1` for numeric types, `60` for `time`, and `1` day for date types. `any` disables the step constraint."#),
    src(r#"URL of an image for `<input type="image">` only.

Click coordinates are submitted as `name.x` and `name.y` query parameters. `alt` is required."#),
    alt(r#"Alternative text for `<input type="image">`.

Required when `type="image"`. Describes the image for screen readers and is shown when the image cannot be loaded."#),
    accept(r#"Hint for which file types are accepted by `<input type="file">`.

A comma-separated list of:
- A file extension starting with `.` (e.g. `.jpg`, `.png`)
- A valid MIME type with no extension (e.g. `image/jpeg`)
- A MIME type wildcard like `image/*`, `audio/*`, `video/*`

The browser may also offer a camera/microphone capture option by adding the `capture` attribute (not exposed here)."#),
    autocomplete(r#"Autofill hint for the control.

Standard tokens (case-insensitive) include:
- `on` / `off` (general autofill toggle)
- `name` (full name)
- `given-name`, `family-name`
- `email`
- `username`
- `current-password`, `new-password`
- `one-time-code`
- `organization`, `organization-title`
- `street-address`, `address-line1`, `address-line2`, `address-level1`, `address-level2`, `postal-code`, `country`, `cc-name`, `cc-number`, `cc-exp`, `cc-csc`
- `tel`, `tel-national`
- `url`
- `photo`
- `bday`, `bday-day`, `bday-month`, `bday-year`

A section name may prefix the token (e.g. `section-blue shipping street-address`)."#),
    autofocus(r#"Boolean attribute. When present, the control receives focus when the document or dialog is loaded.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Only one element per document may autofocus."#),
    form_attr(r#"ID of the `<form>` element with which the control is associated.

Allows the control to be a form participant even when not nested inside the form."#),
    formaction(r#"URL to submit the form to, overriding the form's `action` attribute.

Applies to `<input type="submit">` and `<input type="image">`."#),
    formenctype(r#"Encoding type for form data on submission, overriding the form's `enctype` attribute.

Applies to `<input type="submit">` and `<input type="image">`. Same values as `enctype`:
- `application/x-www-form-urlencoded` (default)
- `multipart/form-data`
- `text/plain`"#),
    formmethod(r#"HTTP method to use on submission, overriding the form's `method` attribute.

Applies to `<input type="submit">` and `<input type="image">`.

One of:
- `get`
- `post`
- `dialog` (close the containing dialog)"#),
    formnovalidate(r#"Boolean attribute. When present, the form is not validated before submission, overriding the form's `novalidate` attribute.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Applies to `<input type="submit">` and `<input type="image">`."#),
    formtarget(r#"Browsing context for the submission response, overriding the form's `target` attribute.

Applies to `<input type="submit">` and `<input type="image">`. Same values as `target`:
- `_self` (default)
- `_blank`
- `_parent`
- `_top`
- a navigable target name"#),
    height(r#"Display height in CSS pixels (a valid non-negative integer) for `<input type="image">`."#),
    width(r#"Display width in CSS pixels (a valid non-negative integer) for `<input type="image">`."#),
    list(r#"ID of a `<datalist>` element providing suggested values for the control.

The datalist is shown as a dropdown of options when the user interacts with the control. The value is not constrained to the suggestions."#),
    multiple(r#"Boolean attribute. When present, the control accepts more than one value.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Applies to `email`, `file`, and `select`."#),
    spellcheck(r#"Hint for the user agent's spell checking behavior.

One of:
- `true` (enable spell checking)
- `false` (disable spell checking)
- `default` (let the user agent decide; this is the default in HTML)

The attribute is part of the global `spellcheck` definition."#),
    datalist(r#"Non-standard alias for the `list` attribute. See `list` for the standard form."#),
    popovertarget(r#"ID of the popover element to toggle when the button is activated.

Applies to `<input type="button">` and `<input type="reset">` and `<input type="submit">` (effectively any input acting as a button). Standardized alongside the Popover API."#),
    popovertargetaction(r#"Action the popover performs when the button is activated.

One of:
- `toggle` (default; show the popover if hidden, hide if showing)
- `show` (always show)
- `hide` (always hide)"#));
define_html_element!(HtmlButton, "button", all,
    type_attr(r#"Behavior of the button.

One of:
- `submit` (default; submits the form)
- `reset` (resets the form's controls to their initial values)
- `button` (no default behavior; use with JavaScript)"#),
    disabled(r#"Boolean attribute. When present, the button is non-interactive and cannot be activated.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Disabled buttons are not submitted with the form."#),
    form_attr(r#"ID of the `<form>` element with which the button is associated.

Allows the button to submit or reset a form even when not nested inside it."#),
    name(r#"Name of the button, submitted with the form as part of the name/value pair.

Only submitted if the button itself activated the submission."#),
    value(r#"Button's value, submitted with the form as part of the name/value pair when the button activates submission.

Distinct from the button's text content; sent to the server instead of the text."#),
    autofocus(r#"Boolean attribute. When present, the button receives focus when the document or dialog is loaded.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Only one element per document may autofocus."#),
    popovertarget(r#"ID of the popover element to toggle when the button is activated."#),
    popovertargetaction(r#"Action the popover performs when the button is activated.

One of:
- `toggle` (default)
- `show`
- `hide`"#));
define_html_element!(HtmlSelect, "select", all,
    name(r#"Name of the control, submitted with the form as part of the name/value pair."#),
    required(r#"Boolean attribute. When present, a non-disabled option must be selected for the form to be valid.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. An empty selection is invalid."#),
    disabled(r#"Boolean attribute. When present, the control is non-interactive and not submitted.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    multiple(r#"Boolean attribute. When present, more than one option may be selected.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Browsers typically render a list box when set."#),
    size(r#"Number of visible options (a valid non-negative integer; default `1` if `multiple` is absent, `4` if present).

When greater than 1 the control is rendered as a list box."#),
    autofocus(r#"Boolean attribute. When present, the control receives focus when the document or dialog is loaded.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    form_attr(r#"ID of the `<form>` element with which the control is associated."#));
define_html_element!(HtmlDatalist, "datalist", no_aria);
define_html_element!(HtmlOptgroup, "optgroup", all,
    disabled(r#"Boolean attribute. When present, all options in the group are non-interactive and not selectable.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    label(r#"User-visible label for the group of options.

Required. Browsers render it as a non-selectable group heading."#));
define_html_element!(HtmlOption, "option", all,
    disabled(r#"Boolean attribute. When present, the option is non-interactive and not selectable.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    label(r#"User-visible label for the option.

If absent, the text content of the option element is used. Used in the rendered list (rather than the value sent on submission)."#),
    selected(r#"Boolean attribute. When present, the option is initially selected.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. In a single-select list, this deselects any previously selected option."#),
    value(r#"Value submitted with the form when this option is selected.

If absent, the text content of the option element is submitted instead."#));
define_html_element!(HtmlTextarea, "textarea", all,
    name(r#"Name of the control, submitted with the form as part of the name/value pair."#),
    rows(r#"Visible height in lines of text (a valid non-negative integer; default `2`).

The textarea scrolls when content exceeds this height."#),
    cols(r#"Visible width in characters (a valid non-negative integer; default `20`).

The textarea wraps when content exceeds this width, depending on `wrap`."#),
    placeholder(r#"Short hint displayed inside the control when its value is empty.

Not a substitute for a `<label>`. Line breaks are not rendered."#),
    required(r#"Boolean attribute. When present, the value must be non-empty for the form to be valid.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    disabled(r#"Boolean attribute. When present, the control is non-interactive and not submitted.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    readonly(r#"Boolean attribute. When present, the control is not editable but is still focusable and submitted with the form.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    maxlength(r#"Maximum number of characters the value may contain (a valid non-negative integer)."#),
    minlength(r#"Minimum number of characters required for the value to be valid (a valid non-negative integer)."#),
    autocomplete(r#"Autofill hint for the control.

Same token set as the `autocomplete` attribute on `<input>`: standard tokens like `on`, `off`, `name`, `email`, `street-address`, `postal-code`, etc."#),
    autofocus(r#"Boolean attribute. When present, the control receives focus when the document or dialog is loaded.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    wrap(r#"Line wrapping behavior of the submitted value.

One of:
- `soft` (default; visual wrap only, no line breaks sent)
- `hard` (visual wrap and `CR LF` line breaks inserted at wrap points; the `cols` attribute is required)"#),
    form_attr(r#"ID of the `<form>` element with which the control is associated."#),
    spellcheck(r#"Hint for the user agent's spell checking behavior.

One of:
- `true` (enable spell checking)
- `false` (disable spell checking)
- `default` (let the user agent decide)"#));
define_html_element!(HtmlOutput, "output", all,
    for_attr(r#"Space-separated list of IDs of input elements whose values contribute to the calculation shown in the output.

A purely structural relationship; not used by the browser, but assistive technologies and the `output.htmlFor` IDL property expose it."#),
    form_attr(r#"ID of the `<form>` element with which the output is associated.

Allows `output` to participate in form submission even when not nested inside the form."#),
    name(r#"Name of the output element, submitted with the form as part of the name/value pair."#));
define_html_element!(HtmlProgress, "progress", all,
    value(r#"Current value of the progress indicator (a valid floating-point number).

Must be between 0 and `max` (or 1 if `max` is omitted). Determines the position of the indicator."#),
    max(r#"Upper bound of the range (a valid floating-point number; default `1`).

The displayed value is the ratio `value / max`."#));
define_html_element!(HtmlMeter, "meter", all,
    value(r#"Current value of the gauge (a valid floating-point number).

Required. Must be between `min` and `max`."#),
    min(r#"Lower bound of the gauge (a valid floating-point number; default `0`)."#),
    max(r#"Upper bound of the gauge (a valid floating-point number; default `1`).

Must be greater than `min`."#),
    low(r#"Upper bound of the "low" range (a valid floating-point number; default equal to `min`).

Values at or below this number are considered low."#),
    high(r#"Lower bound of the "high" range (a valid floating-point number; default equal to `max`).

Values at or above this number are considered high. Must be greater than `low`."#),
    optimum(r#"Optimal value of the gauge (a valid floating-point number).

Should lie within `min`..`max`. The user agent may style the gauge differently depending on whether the current value is below, within, or above the optimal range."#));
define_html_element!(HtmlFieldset, "fieldset", all,
    disabled(r#"Boolean attribute. When present, all form controls in the fieldset are disabled (matching the effect of setting `disabled` on each one).

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. The descendant controls are excluded from submission."#),
    form_attr(r#"ID of the `<form>` element with which the fieldset is associated."#),
    name(r#"Name of the fieldset.

Submitted with the form as part of the name/value pair only if the fieldset is the form's first `<fieldset>` ancestor of an element being submitted."#));
define_html_element!(HtmlLegend, "legend", all);

factory!(
    /// Create a new [`HtmlForm`] element (`<form>`).
    form, HtmlForm
);
factory!(
    /// Create a new [`HtmlLabel`] element (`<label>`).
    label, HtmlLabel
);
factory!(
    /// Create a new [`HtmlInput`] element (`<input>`).
    input, HtmlInput
);
factory!(
    /// Create a new [`HtmlButton`] element (`<button>`).
    button, HtmlButton
);
factory!(
    /// Create a new [`HtmlSelect`] element (`<select>`).
    select, HtmlSelect
);
factory!(
    /// Create a new [`HtmlDatalist`] element (`<datalist>`).
    datalist, HtmlDatalist
);
factory!(
    /// Create a new [`HtmlOptgroup`] element (`<optgroup>`).
    optgroup, HtmlOptgroup
);
factory!(
    /// Create a new [`HtmlOption`] element (`<option>`).
    option, HtmlOption
);
factory!(
    /// Create a new [`HtmlTextarea`] element (`<textarea>`).
    textarea, HtmlTextarea
);
factory!(
    /// Create a new [`HtmlOutput`] element (`<output>`).
    output, HtmlOutput
);
factory!(
    /// Create a new [`HtmlProgress`] element (`<progress>`).
    progress, HtmlProgress
);
factory!(
    /// Create a new [`HtmlMeter`] element (`<meter>`).
    meter, HtmlMeter
);
factory!(
    /// Create a new [`HtmlFieldset`] element (`<fieldset>`).
    fieldset, HtmlFieldset
);
factory!(
    /// Create a new [`HtmlLegend`] element (`<legend>`).
    legend, HtmlLegend
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_attrs() {
        assert_eq!(form().action("/submit").render(), r#"<form action="/submit"></form>"#);
        assert_eq!(form().method("post").render(), r#"<form method="post"></form>"#);
        assert_eq!(form().enctype("multipart/form-data").render(), r#"<form enctype="multipart/form-data"></form>"#);
        assert_eq!(form().target("_blank").render(), r#"<form target="_blank"></form>"#);
        assert_eq!(form().autocomplete("on").render(), r#"<form autocomplete="on"></form>"#);
        assert_eq!(form().novalidate("true").render(), r#"<form novalidate="true"></form>"#);
        assert_eq!(form().name("myform").render(), r#"<form name="myform"></form>"#);
        assert_eq!(form().accept_charset("UTF-8").render(), r#"<form accept-charset="UTF-8"></form>"#);
        assert_eq!(form().rel("stylesheet").render(), r#"<form rel="stylesheet"></form>"#);
    }

    #[test]
    fn label_attrs() {
        assert_eq!(label().for_attr("email").render(), r#"<label for="email"></label>"#);
        assert_eq!(label().form_attr("myform").render(), r#"<label form="myform"></label>"#);
    }

    #[test]
    fn input_attrs() {
        assert_eq!(input().type_attr("text").render(), r#"<input type="text">"#);
        assert_eq!(input().name("field").render(), r#"<input name="field">"#);
        assert_eq!(input().value("hello").render(), r#"<input value="hello">"#);
        assert_eq!(input().placeholder("Enter...").render(), r#"<input placeholder="Enter...">"#);
        assert_eq!(input().required("true").render(), r#"<input required="true">"#);
        assert_eq!(input().disabled("true").render(), r#"<input disabled="true">"#);
        assert_eq!(input().readonly("true").render(), r#"<input readonly="true">"#);
        assert_eq!(input().checked("true").render(), r#"<input checked="true">"#);
        assert_eq!(input().maxlength("100").render(), r#"<input maxlength="100">"#);
        assert_eq!(input().minlength("5").render(), r#"<input minlength="5">"#);
        assert_eq!(input().pattern("[a-z]+").render(), r#"<input pattern="[a-z]+">"#);
        assert_eq!(input().size("20").render(), r#"<input size="20">"#);
        assert_eq!(input().min("0").render(), r#"<input min="0">"#);
        assert_eq!(input().max("100").render(), r#"<input max="100">"#);
        assert_eq!(input().step("5").render(), r#"<input step="5">"#);
        assert_eq!(input().src("pic.jpg").render(), r#"<input src="pic.jpg">"#);
        assert_eq!(input().alt("Pic").render(), r#"<input alt="Pic">"#);
        assert_eq!(input().accept("image/*").render(), r#"<input accept="image/*">"#);
        assert_eq!(input().autocomplete("off").render(), r#"<input autocomplete="off">"#);
        assert_eq!(input().autofocus("true").render(), r#"<input autofocus="true">"#);
        assert_eq!(input().form_attr("myform").render(), r#"<input form="myform">"#);
        assert_eq!(input().formaction("/upload").render(), r#"<input formaction="/upload">"#);
        assert_eq!(input().formenctype("multipart/form-data").render(), r#"<input formenctype="multipart/form-data">"#);
        assert_eq!(input().formmethod("post").render(), r#"<input formmethod="post">"#);
        assert_eq!(input().formnovalidate("true").render(), r#"<input formnovalidate="true">"#);
        assert_eq!(input().formtarget("_blank").render(), r#"<input formtarget="_blank">"#);
        assert_eq!(input().height("300").render(), r#"<input height="300">"#);
        assert_eq!(input().width("200").render(), r#"<input width="200">"#);
        assert_eq!(input().list("opts").render(), r#"<input list="opts">"#);
        assert_eq!(input().multiple("true").render(), r#"<input multiple="true">"#);
        assert_eq!(input().spellcheck("true").render(), r#"<input spellcheck="true">"#);
        assert_eq!(input().datalist("opts").render(), r#"<input datalist="opts">"#);
        assert_eq!(input().popovertarget("menu").render(), r#"<input popovertarget="menu">"#);
        assert_eq!(input().popovertargetaction("toggle").render(), r#"<input popovertargetaction="toggle">"#);
    }

    #[test]
    fn button_attrs() {
        assert_eq!(button().type_attr("submit").render(), r#"<button type="submit"></button>"#);
        assert_eq!(button().disabled("true").render(), r#"<button disabled="true"></button>"#);
        assert_eq!(button().form_attr("myform").render(), r#"<button form="myform"></button>"#);
        assert_eq!(button().name("btn").render(), r#"<button name="btn"></button>"#);
        assert_eq!(button().value("ok").render(), r#"<button value="ok"></button>"#);
        assert_eq!(button().autofocus("true").render(), r#"<button autofocus="true"></button>"#);
        assert_eq!(button().popovertarget("menu").render(), r#"<button popovertarget="menu"></button>"#);
        assert_eq!(button().popovertargetaction("toggle").render(), r#"<button popovertargetaction="toggle"></button>"#);
    }

    #[test]
    fn select_attrs() {
        assert_eq!(select().name("choice").render(), r#"<select name="choice"></select>"#);
        assert_eq!(select().required("true").render(), r#"<select required="true"></select>"#);
        assert_eq!(select().disabled("true").render(), r#"<select disabled="true"></select>"#);
        assert_eq!(select().multiple("true").render(), r#"<select multiple="true"></select>"#);
        assert_eq!(select().size("5").render(), r#"<select size="5"></select>"#);
        assert_eq!(select().autofocus("true").render(), r#"<select autofocus="true"></select>"#);
        assert_eq!(select().form_attr("myform").render(), r#"<select form="myform"></select>"#);
    }

    #[test]
    fn datalist_element() {
        assert_eq!(datalist().render(), "<datalist></datalist>");
    }

    #[test]
    fn optgroup_attrs() {
        assert_eq!(optgroup().disabled("true").render(), r#"<optgroup disabled="true"></optgroup>"#);
        assert_eq!(optgroup().label("Group").render(), r#"<optgroup label="Group"></optgroup>"#);
        assert_eq!(optgroup().append_attrs(vec![crate::attributes::attr("disabled").value("true"), crate::attributes::attr("label").value("Group")]).render(), r#"<optgroup disabled="true" label="Group"></optgroup>"#);
    }

    #[test]
    fn option_attrs() {
        assert_eq!(option().disabled("true").render(), r#"<option disabled="true"></option>"#);
        assert_eq!(option().label("Opt").render(), r#"<option label="Opt"></option>"#);
        assert_eq!(option().selected("true").render(), r#"<option selected="true"></option>"#);
        assert_eq!(option().value("1").render(), r#"<option value="1"></option>"#);
    }

    #[test]
    fn textarea_attrs() {
        assert_eq!(textarea().name("bio").render(), r#"<textarea name="bio"></textarea>"#);
        assert_eq!(textarea().rows("10").render(), r#"<textarea rows="10"></textarea>"#);
        assert_eq!(textarea().cols("50").render(), r#"<textarea cols="50"></textarea>"#);
        assert_eq!(textarea().placeholder("Tell us...").render(), r#"<textarea placeholder="Tell us..."></textarea>"#);
        assert_eq!(textarea().required("true").render(), r#"<textarea required="true"></textarea>"#);
        assert_eq!(textarea().disabled("true").render(), r#"<textarea disabled="true"></textarea>"#);
        assert_eq!(textarea().readonly("true").render(), r#"<textarea readonly="true"></textarea>"#);
        assert_eq!(textarea().maxlength("500").render(), r#"<textarea maxlength="500"></textarea>"#);
        assert_eq!(textarea().minlength("10").render(), r#"<textarea minlength="10"></textarea>"#);
        assert_eq!(textarea().autocomplete("off").render(), r#"<textarea autocomplete="off"></textarea>"#);
        assert_eq!(textarea().autofocus("true").render(), r#"<textarea autofocus="true"></textarea>"#);
        assert_eq!(textarea().wrap("soft").render(), r#"<textarea wrap="soft"></textarea>"#);
        assert_eq!(textarea().form_attr("myform").render(), r#"<textarea form="myform"></textarea>"#);
        assert_eq!(textarea().spellcheck("true").render(), r#"<textarea spellcheck="true"></textarea>"#);
    }

    #[test]
    fn output_attrs() {
        assert_eq!(output().for_attr("a b").render(), r#"<output for="a b"></output>"#);
        assert_eq!(output().form_attr("myform").render(), r#"<output form="myform"></output>"#);
        assert_eq!(output().name("result").render(), r#"<output name="result"></output>"#);
    }

    #[test]
    fn progress_attrs() {
        assert_eq!(progress().value("50").render(), r#"<progress value="50"></progress>"#);
        assert_eq!(progress().max("100").render(), r#"<progress max="100"></progress>"#);
    }

    #[test]
    fn meter_attrs() {
        assert_eq!(meter().value("50").render(), r#"<meter value="50"></meter>"#);
        assert_eq!(meter().min("0").render(), r#"<meter min="0"></meter>"#);
        assert_eq!(meter().max("100").render(), r#"<meter max="100"></meter>"#);
        assert_eq!(meter().low("20").render(), r#"<meter low="20"></meter>"#);
        assert_eq!(meter().high("80").render(), r#"<meter high="80"></meter>"#);
        assert_eq!(meter().optimum("50").render(), r#"<meter optimum="50"></meter>"#);
    }

    #[test]
    fn fieldset_attrs() {
        assert_eq!(fieldset().disabled("true").render(), r#"<fieldset disabled="true"></fieldset>"#);
        assert_eq!(fieldset().form_attr("myform").render(), r#"<fieldset form="myform"></fieldset>"#);
        assert_eq!(fieldset().name("group").render(), r#"<fieldset name="group"></fieldset>"#);
    }

    #[test]
    fn legend_element() {
        assert_eq!(legend().render(), "<legend></legend>");
    }
}
