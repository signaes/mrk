//! Form elements (`<form>`, `<input>`, `<button>`, `<select>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlForm, "form",
    action("URL to submit the form to."),
    method("HTTP method (get, post, dialog)."),
    enctype("Encoding type for form data."),
    target("Frame target for the response."),
    autocomplete("Whether autocomplete is enabled."),
    novalidate("Whether to skip validation."),
    name("Form name."),
    accept_charset("Accepted character encodings."),
    rel("Relationship to the linked resource."));
define_html_element!(HtmlLabel, "label",
    for_attr("ID of the associated form control."),
    form_attr("Associated form ID."));
define_html_element!(HtmlInput, "input",
    type_attr("Input type (text, email, password, etc.)."),
    name("Field name."),
    value("Current value."),
    placeholder("Placeholder text."),
    required("Whether the field is required."),
    disabled("Whether the field is disabled."),
    readonly("Whether the field is read-only."),
    checked("Whether the field is checked."),
    maxlength("Maximum length of the value."),
    minlength("Minimum length of the value."),
    pattern("Regular expression for validation."),
    size("Display width in characters."),
    min("Minimum value."),
    max("Maximum value."),
    step("Step increment for numeric inputs."),
    src("URL of the image for image inputs."),
    alt("Alternative text for image inputs."),
    accept("Accepted file types for file inputs."),
    autocomplete("Autocomplete hint."),
    autofocus("Whether to focus on page load."),
    form_attr("Associated form ID."),
    formaction("URL to submit the form to (overrides form action)."),
    formenctype("Encoding type for form data (overrides form)."),
    formmethod("HTTP method (overrides form method)."),
    formnovalidate("Whether to skip validation (overrides form)."),
    formtarget("Frame target (overrides form target)."),
    height("Display height in pixels."),
    width("Display width in pixels."),
    list("ID of the associated datalist."),
    multiple("Whether multiple values are allowed."),
    spellcheck("Whether spell checking is enabled."),
    datalist("ID of the associated datalist."),
    popovertarget("ID of the popover element to toggle."),
    popovertargetaction("Popover action (toggle, show, or hide)."));
define_html_element!(HtmlButton, "button",
    type_attr("Button type (submit, reset, button)."),
    disabled("Whether the button is disabled."),
    form_attr("Associated form ID."),
    name("Button name."),
    value("Button value."),
    autofocus("Whether to focus on page load."),
    popovertarget("ID of the popover element to toggle."),
    popovertargetaction("Popover action (toggle, show, or hide)."));
define_html_element!(HtmlSelect, "select",
    name("Select name."),
    required("Whether selection is required."),
    disabled("Whether the select is disabled."),
    multiple("Whether multiple selection is allowed."),
    size("Number of visible options."),
    autofocus("Whether to focus on page load."),
    form_attr("Associated form ID."));
define_html_element!(HtmlDatalist, "datalist");
define_html_element!(HtmlOptgroup, "optgroup",
    disabled("Whether the group is disabled."),
    label("Group label text."));
define_html_element!(HtmlOption, "option",
    disabled("Whether the option is disabled."),
    label("Option label text."),
    selected("Whether the option is selected."),
    value("Value submitted when selected."));
define_html_element!(HtmlTextarea, "textarea",
    name("Textarea name."),
    rows("Number of visible text rows."),
    cols("Number of visible text columns."),
    placeholder("Placeholder text."),
    required("Whether the field is required."),
    disabled("Whether the field is disabled."),
    readonly("Whether the field is read-only."),
    maxlength("Maximum length of the value."),
    minlength("Minimum length of the value."),
    autocomplete("Autocomplete hint."),
    autofocus("Whether to focus on page load."),
    wrap("Line wrapping mode (soft or hard)."),
    form_attr("Associated form ID."),
    spellcheck("Whether spell checking is enabled."));
define_html_element!(HtmlOutput, "output",
    for_attr("Space-separated IDs of input elements."),
    form_attr("Associated form ID."),
    name("Output name."));
define_html_element!(HtmlProgress, "progress",
    value("Current value."),
    max("Maximum value."));
define_html_element!(HtmlMeter, "meter",
    value("Current value."),
    min("Minimum value."),
    max("Maximum value."),
    low("Low threshold."),
    high("High threshold."),
    optimum("Optimal value."));
define_html_element!(HtmlFieldset, "fieldset",
    disabled("Whether the fieldset is disabled."),
    form_attr("Associated form ID."),
    name("Fieldset name."));
define_html_element!(HtmlLegend, "legend");

// Create a new [`HtmlForm`] element (`<form>`).
factory!(form, HtmlForm);
// Create a new [`HtmlLabel`] element (`<label>`).
factory!(label, HtmlLabel);
// Create a new [`HtmlInput`] element (`<input>`).
factory!(input, HtmlInput);
// Create a new [`HtmlButton`] element (`<button>`).
factory!(button, HtmlButton);
// Create a new [`HtmlSelect`] element (`<select>`).
factory!(select, HtmlSelect);
// Create a new [`HtmlDatalist`] element (`<datalist>`).
factory!(datalist, HtmlDatalist);
// Create a new [`HtmlOptgroup`] element (`<optgroup>`).
factory!(optgroup, HtmlOptgroup);
// Create a new [`HtmlOption`] element (`<option>`).
factory!(option, HtmlOption);
// Create a new [`HtmlTextarea`] element (`<textarea>`).
factory!(textarea, HtmlTextarea);
// Create a new [`HtmlOutput`] element (`<output>`).
factory!(output, HtmlOutput);
// Create a new [`HtmlProgress`] element (`<progress>`).
factory!(progress, HtmlProgress);
// Create a new [`HtmlMeter`] element (`<meter>`).
factory!(meter, HtmlMeter);
// Create a new [`HtmlFieldset`] element (`<fieldset>`).
factory!(fieldset, HtmlFieldset);
// Create a new [`HtmlLegend`] element (`<legend>`).
factory!(legend, HtmlLegend);

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
        assert_eq!(optgroup().attrs(vec![crate::attributes::attr("disabled").value("true"), crate::attributes::attr("label").value("Group")]).render(), r#"<optgroup disabled="true" label="Group"></optgroup>"#);
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
