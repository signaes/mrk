//! Embedded content elements (`<img>`, `<video>`, `<audio>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlPicture, "picture");
define_html_element!(HtmlSource, "source",
    type_attr("MIME type of the resource."),
    media("Target media query."),
    src("URL of the resource."),
    srcset("Image sources for responsive images."),
    sizes("Image sizes for responsive images."),
    width("Display width in pixels."),
    height("Display height in pixels."));
define_html_element!(HtmlImg, "img",
    src("URL of the image."),
    alt("Alternative text description."),
    width("Display width in pixels."),
    height("Display height in pixels."),
    loading("Loading behavior (lazy or eager)."),
    decoding("Image decoding hint (async, sync, or auto)."),
    fetchpriority("Fetch priority hint (high, low, or auto)."),
    referrerpolicy("Referrer policy for the request."),
    sizes("Image sizes for responsive images."),
    srcset("Image sources for responsive images."),
    crossorigin("CORS setting (anonymous or use-credentials)."),
    usemap("Name of the image map to use."),
    ismap("Whether the image is a server-side image map."),
    longdesc("URL of a long description of the image."));
define_html_element!(HtmlIframe, "iframe",
    src("URL of the embedded page."),
    srcdoc("Inline HTML to display."),
    name("Frame name for targeting."),
    width("Display width in pixels."),
    height("Display height in pixels."),
    loading("Loading behavior (lazy or eager)."),
    referrerpolicy("Referrer policy for the request."),
    sandbox("Permissions policy for the iframe."),
    allow("Feature policy for the iframe."),
    allowfullscreen("Whether fullscreen is allowed."),
    allowpaymentrequest("Whether payment API is allowed."),
    credentialless("Whether to send credentials."),
    csp("Content Security Policy for the iframe."));
define_html_element!(HtmlEmbed, "embed",
    src("URL of the embedded content."),
    type_attr("MIME type of the content."),
    width("Display width in pixels."),
    height("Display height in pixels."));
define_html_element!(HtmlObject, "object",
    data("URL of the resource."),
    type_attr("MIME type of the resource."),
    name("Object name for form submission."),
    form("Associated form ID."),
    width("Display width in pixels."),
    height("Display height in pixels."),
    typemustmatch("Whether the type must match the resource."));
define_html_element!(HtmlParam, "param",
    name("Parameter name."),
    value("Parameter value."));
define_html_element!(HtmlVideo, "video",
    src("URL of the video source."),
    poster("URL of the poster image."),
    controls("Whether to show media controls."),
    autoplay("Whether to play automatically."),
    loop_attr("Whether to loop."),
    muted("Whether to mute audio."),
    preload("Preload behavior (auto, metadata, or none)."),
    width("Display width in pixels."),
    height("Display height in pixels."),
    playsinline("Whether to play inline on mobile."),
    crossorigin("CORS setting."));
define_html_element!(HtmlAudio, "audio",
    src("URL of the audio source."),
    controls("Whether to show media controls."),
    autoplay("Whether to play automatically."),
    loop_attr("Whether to loop."),
    muted("Whether to mute audio."),
    preload("Preload behavior (auto, metadata, or none)."),
    crossorigin("CORS setting."));
define_html_element!(HtmlTrack, "track",
    src("URL of the subtitle file."),
    kind("Kind of track (subtitles, captions, etc.)."),
    srclang("Language of the track."),
    label("Track label for user display."),
    default_attr("Whether the track is enabled by default."));
define_html_element!(HtmlMap, "map", name("Name of the image map."));
define_html_element!(HtmlArea, "area",
    alt("Alternative text for the area."),
    coords("Coordinates of the area."),
    shape("Shape of the area (rect, circle, poly)."),
    href("URL of the area."),
    target("Frame target for the link."),
    rel("Relationship to the linked resource."),
    download("Filename for downloading the link."),
    ping("URLs to ping when the link is clicked."),
    referrerpolicy("Referrer policy for the request."),
    type_attr("MIME type of the linked resource."),
    media("Target media query."));
define_html_element!(HtmlPortal, "portal");

// Create a new [`HtmlPicture`] element (`<picture>`).
factory!(picture, HtmlPicture);
// Create a new [`HtmlSource`] element (`<source>`).
factory!(source, HtmlSource);
// Create a new [`HtmlImg`] element (`<img>`).
factory!(img, HtmlImg);
// Create a new [`HtmlIframe`] element (`<iframe>`).
factory!(iframe, HtmlIframe);
// Create a new [`HtmlEmbed`] element (`<embed>`).
factory!(embed, HtmlEmbed);
// Create a new [`HtmlObject`] element (`<object>`).
factory!(object, HtmlObject);
// Create a new [`HtmlParam`] element (`<param>`).
factory!(param, HtmlParam);
// Create a new [`HtmlVideo`] element (`<video>`).
factory!(video, HtmlVideo);
// Create a new [`HtmlAudio`] element (`<audio>`).
factory!(audio, HtmlAudio);
// Create a new [`HtmlTrack`] element (`<track>`).
factory!(track, HtmlTrack);
// Create a new [`HtmlMap`] element (`<map>`).
factory!(map, HtmlMap);
// Create a new [`HtmlArea`] element (`<area>`).
factory!(area, HtmlArea);
// Create a new [`HtmlPortal`] element (`<portal>`).
factory!(portal, HtmlPortal);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picture_element() {
        assert_eq!(picture().render(), "<picture></picture>");
    }

    #[test]
    fn source_attrs() {
        assert_eq!(source().type_attr("image/webp").render(), r#"<source type="image/webp">"#);
        assert_eq!(source().media("(min-width: 800px)").render(), r#"<source media="(min-width: 800px)">"#);
        assert_eq!(source().src("img.jpg").render(), r#"<source src="img.jpg">"#);
        assert_eq!(source().srcset("img.webp").render(), r#"<source srcset="img.webp">"#);
        assert_eq!(source().sizes("100vw").render(), r#"<source sizes="100vw">"#);
        assert_eq!(source().width("640").render(), r#"<source width="640">"#);
        assert_eq!(source().height("480").render(), r#"<source height="480">"#);
    }

    #[test]
    fn img_attrs() {
        assert_eq!(img().src("photo.jpg").render(), r#"<img src="photo.jpg">"#);
        assert_eq!(img().alt("Photo").render(), r#"<img alt="Photo">"#);
        assert_eq!(img().width("100").render(), r#"<img width="100">"#);
        assert_eq!(img().height("200").render(), r#"<img height="200">"#);
        assert_eq!(img().loading("lazy").render(), r#"<img loading="lazy">"#);
        assert_eq!(img().decoding("async").render(), r#"<img decoding="async">"#);
        assert_eq!(img().fetchpriority("high").render(), r#"<img fetchpriority="high">"#);
        assert_eq!(img().referrerpolicy("no-referrer").render(), r#"<img referrerpolicy="no-referrer">"#);
        assert_eq!(img().sizes("100vw").render(), r#"<img sizes="100vw">"#);
        assert_eq!(img().srcset("img.webp").render(), r#"<img srcset="img.webp">"#);
        assert_eq!(img().crossorigin("anonymous").render(), r#"<img crossorigin="anonymous">"#);
        assert_eq!(img().usemap("#map").render(), r##"<img usemap="#map">"##);
        assert_eq!(img().ismap("true").render(), r#"<img ismap="true">"#);
        assert_eq!(img().longdesc("desc.html").render(), r#"<img longdesc="desc.html">"#);
    }

    #[test]
    fn iframe_attrs() {
        assert_eq!(iframe().src("page.html").render(), r#"<iframe src="page.html"></iframe>"#);
        assert_eq!(iframe().srcdoc("hello").render(), r#"<iframe srcdoc="hello"></iframe>"#);
        assert_eq!(iframe().name("frame").render(), r#"<iframe name="frame"></iframe>"#);
        assert_eq!(iframe().width("300").render(), r#"<iframe width="300"></iframe>"#);
        assert_eq!(iframe().height("200").render(), r#"<iframe height="200"></iframe>"#);
        assert_eq!(iframe().loading("lazy").render(), r#"<iframe loading="lazy"></iframe>"#);
        assert_eq!(iframe().referrerpolicy("no-referrer").render(), r#"<iframe referrerpolicy="no-referrer"></iframe>"#);
        assert_eq!(iframe().sandbox("allow-scripts").render(), r#"<iframe sandbox="allow-scripts"></iframe>"#);
        assert_eq!(iframe().allow("camera").render(), r#"<iframe allow="camera"></iframe>"#);
        assert_eq!(iframe().allowfullscreen("true").render(), r#"<iframe allowfullscreen="true"></iframe>"#);
        assert_eq!(iframe().allowpaymentrequest("true").render(), r#"<iframe allowpaymentrequest="true"></iframe>"#);
        assert_eq!(iframe().credentialless("true").render(), r#"<iframe credentialless="true"></iframe>"#);
        assert_eq!(iframe().csp("default-src 'self'").render(), r#"<iframe csp="default-src 'self'"></iframe>"#);
    }

    #[test]
    fn embed_attrs() {
        assert_eq!(embed().src("plugin.swf").render(), r#"<embed src="plugin.swf">"#);
        assert_eq!(embed().type_attr("application/pdf").render(), r#"<embed type="application/pdf">"#);
        assert_eq!(embed().width("400").render(), r#"<embed width="400">"#);
        assert_eq!(embed().height("300").render(), r#"<embed height="300">"#);
    }

    #[test]
    fn object_attrs() {
        assert_eq!(object().data("file.swf").render(), r#"<object data="file.swf"></object>"#);
        assert_eq!(object().type_attr("application/pdf").render(), r#"<object type="application/pdf"></object>"#);
        assert_eq!(object().name("obj").render(), r#"<object name="obj"></object>"#);
        assert_eq!(object().form("myform").render(), r#"<object form="myform"></object>"#);
        assert_eq!(object().width("400").render(), r#"<object width="400"></object>"#);
        assert_eq!(object().height("300").render(), r#"<object height="300"></object>"#);
        assert_eq!(object().typemustmatch("true").render(), r#"<object typemustmatch="true"></object>"#);
    }

    #[test]
    fn param_attrs() {
        assert_eq!(param().name("movie").render(), r#"<param name="movie">"#);
        assert_eq!(param().value("film.swf").render(), r#"<param value="film.swf">"#);
    }

    #[test]
    fn video_attrs() {
        assert_eq!(video().src("vid.mp4").render(), r#"<video src="vid.mp4"></video>"#);
        assert_eq!(video().poster("thumb.jpg").render(), r#"<video poster="thumb.jpg"></video>"#);
        assert_eq!(video().controls("true").render(), r#"<video controls="true"></video>"#);
        assert_eq!(video().autoplay("true").render(), r#"<video autoplay="true"></video>"#);
        assert_eq!(video().loop_attr("true").render(), r#"<video loop="true"></video>"#);
        assert_eq!(video().muted("true").render(), r#"<video muted="true"></video>"#);
        assert_eq!(video().preload("auto").render(), r#"<video preload="auto"></video>"#);
        assert_eq!(video().width("640").render(), r#"<video width="640"></video>"#);
        assert_eq!(video().height("480").render(), r#"<video height="480"></video>"#);
        assert_eq!(video().playsinline("true").render(), r#"<video playsinline="true"></video>"#);
        assert_eq!(video().crossorigin("anonymous").render(), r#"<video crossorigin="anonymous"></video>"#);
    }

    #[test]
    fn audio_attrs() {
        assert_eq!(audio().src("sound.mp3").render(), r#"<audio src="sound.mp3"></audio>"#);
        assert_eq!(audio().controls("true").render(), r#"<audio controls="true"></audio>"#);
        assert_eq!(audio().autoplay("true").render(), r#"<audio autoplay="true"></audio>"#);
        assert_eq!(audio().loop_attr("true").render(), r#"<audio loop="true"></audio>"#);
        assert_eq!(audio().muted("true").render(), r#"<audio muted="true"></audio>"#);
        assert_eq!(audio().preload("auto").render(), r#"<audio preload="auto"></audio>"#);
        assert_eq!(audio().crossorigin("anonymous").render(), r#"<audio crossorigin="anonymous"></audio>"#);
    }

    #[test]
    fn track_attrs() {
        assert_eq!(track().src("subs.vtt").render(), r#"<track src="subs.vtt">"#);
        assert_eq!(track().kind("subtitles").render(), r#"<track kind="subtitles">"#);
        assert_eq!(track().srclang("en").render(), r#"<track srclang="en">"#);
        assert_eq!(track().label("English").render(), r#"<track label="English">"#);
        assert_eq!(track().default_attr("true").render(), r#"<track default="true">"#);
    }

    #[test]
    fn map_attrs() {
        assert_eq!(map().name("mymap").render(), r#"<map name="mymap"></map>"#);
    }

    #[test]
    fn area_attrs() {
        assert_eq!(area().alt("Click").render(), r#"<area alt="Click">"#);
        assert_eq!(area().coords("0,0,100,100").render(), r#"<area coords="0,0,100,100">"#);
        assert_eq!(area().shape("rect").render(), r#"<area shape="rect">"#);
        assert_eq!(area().href("/link").render(), r#"<area href="/link">"#);
        assert_eq!(area().target("_blank").render(), r#"<area target="_blank">"#);
        assert_eq!(area().rel("noopener").render(), r#"<area rel="noopener">"#);
        assert_eq!(area().download("file.txt").render(), r#"<area download="file.txt">"#);
        assert_eq!(area().ping("/track").render(), r#"<area ping="/track">"#);
        assert_eq!(area().referrerpolicy("no-referrer").render(), r#"<area referrerpolicy="no-referrer">"#);
        assert_eq!(area().type_attr("text/html").render(), r#"<area type="text/html">"#);
        assert_eq!(area().media("screen").render(), r#"<area media="screen">"#);
    }

    #[test]
    fn portal_element() {
        assert_eq!(portal().render(), "<portal></portal>");
    }
}
