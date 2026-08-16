//! Embedded content elements (`<img>`, `<video>`, `<audio>`, etc.).

use super::macros::{add_bool_methods, define_html_element, factory};

define_html_element!(HtmlPicture, "picture", aria_hidden_only);
define_html_element!(HtmlSource, "source", no_aria,
    type_attr(r#"MIME type of the resource.

For `<source>` inside `<picture>`: a MIME type or `image/*` wildcard, used to filter matching images.
For `<source>` inside `<video>` or `<audio>`: a MIME type with optional codecs (e.g. `video/mp4; codecs="avc1.42E01E"`)."#),
    media(r#"Media query list (e.g. `screen`, `(min-width: 800px)`).

For `<picture>` sources: matches the user's environment. The first matching source is selected.
For media sources: informational; not all browsers apply it.

Accepts any valid media query list."#),
    src(r#"URL of the resource.

For `<picture>`: typically combined with `srcset` and `sizes` via the `srcset`/`sizes` attributes rather than `src`.
For `<video>` / `<audio>`: the source media URL."#),
    srcset(r#"Source set for responsive images.

Comma-separated list of `<url> <descriptor>` pairs. Each entry may use width descriptors (`small.webp 480w`) or pixel-density descriptors (`small.webp 1x, large.webp 2x`).

Used with `sizes` to select a source for the current viewport."#),
    sizes(r#"Sizes for the responsive image source set.

Comma-separated list of media-condition / source-size pairs (e.g. `(max-width: 600px) 100vw, 50vw`). Describes the intended display width of the image for the user agent to pick a source from `srcset`."#),
    width(r#"Display width of the image in CSS pixels (a valid non-negative integer).

For `<picture>` and `<source>`, the `width` attribute is a hint."#),
    height(r#"Display height of the image in CSS pixels (a valid non-negative integer).

For `<picture>` and `<source>`, the `height` attribute is a hint."#));
define_html_element!(HtmlImg, "img", all,
    src(r#"URL of the image to embed.

Required for image fetches. A data URL or blob URL is also allowed."#),
    alt(r#"Alternative text describing the image, used by screen readers, search engines, and shown when the image cannot be loaded.

Must be a non-empty string for `<img>` (except in a small set of decorative cases where `alt=""` is intentional). For images that contain text, the text should be included."#),
    width(r#"Rendered width of the image in CSS pixels (a valid non-negative integer).

Affects the image's intrinsic size and the aspect ratio used to reserve layout space before the image loads."#),
    height(r#"Rendered height of the image in CSS pixels (a valid non-negative integer).

Affects the image's intrinsic size and the aspect ratio used to reserve layout space before the image loads."#),
    loading(r#"Hint for when the user agent should begin loading the image.

One of:
- `eager` (default; load immediately)
- `lazy` (defer until the image is near the viewport)"#),
    decoding(r#"Hint for how the user agent should decode the image.

One of:
- `sync` (decode synchronously, blocking other work)
- `async` (decode asynchronously)
- `auto` (no preference; user-agent decides)"#),
    fetchpriority(r#"Hint for the relative fetch priority of the image request.

One of:
- `high`
- `low`
- `auto` (default)"#),
    referrerpolicy(r#"Referrer policy for the request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#),
    sizes(r#"Sizes for the responsive image source set.

Comma-separated list of media-condition / source-size pairs (e.g. `(max-width: 600px) 100vw, 50vw`)."#),
    srcset(r#"Source set for responsive images.

Comma-separated list of `<url> <descriptor>` pairs (e.g. `small.webp 480w, large.webp 1080w`)."#),
    crossorigin(r#"CORS setting for the image request.

One of:
- `anonymous`
- `use-credentials`

Required for canvas pixel access to images from foreign origins."#),
    usemap(r#"Name of the `<map>` element to associate with this image, prefixed with `#` (e.g. `#nav-map`).

The referenced `<map>` defines clickable regions via `<area>` children."#));
add_bool_methods!(HtmlImg,
    ismap(r#"Boolean attribute. When present, the image is a server-side image map: clicks submit the click coordinates as query parameters on the parent `<a>`'s href.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#));
define_html_element!(HtmlIframe, "iframe", all,
    src(r#"URL of the page to embed.

Used to load a navigable into the frame. Combine with `sandbox` and `referrerpolicy` to control the embedded content's permissions."#),
    srcdoc(r#"Inline HTML to display as the frame's content.

Takes precedence over `src` when present. Authoring the document means authoring a complete document (with `<html>`, `<head>`, `<body>`). Use `%22` and `%23` to escape embedded quotes for safe embedding."#),
    name(r#"Name of the frame, used as a target for hyperlinks and forms (e.g. `<a target="myframe">`).

Must be a valid browsing-context name (a string of 1+ ASCII letters, digits, or hyphens, not starting with a digit; or one of the reserved keywords `_blank`, `_self`, `_parent`, `_top`)."#),
    width(r#"Rendered width of the frame in CSS pixels (a valid non-negative integer; default `300`)."#),
    height(r#"Rendered height of the frame in CSS pixels (a valid non-negative integer; default `150`)."#),
    loading(r#"Hint for when the user agent should begin loading the frame.

One of:
- `eager` (default; load immediately)
- `lazy` (defer until near the viewport)"#),
    referrerpolicy(r#"Referrer policy for the iframe request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#),
    sandbox(r#"Permissions policy for the frame's content.

A space-separated list of sandbox tokens. Each token re-enables a feature that is otherwise restricted in the sandbox:
- `allow-downloads`
- `allow-forms`
- `allow-modals`
- `allow-orientation-lock`
- `allow-pointer-lock`
- `allow-popups`
- `allow-popups-to-escape-sandbox`
- `allow-presentation`
- `allow-same-origin`
- `allow-scripts`
- `allow-top-navigation`
- `allow-top-navigation-by-user-activation`
- `allow-top-navigation-to-custom-protocols`

If the attribute is absent (or empty), the frame is fully sandboxed: same-origin is denied, scripts cannot run, and forms cannot be submitted."#),
    allow(r#"Permissions Policy feature allowlist for the embedded document.

A semicolon-separated list of `feature-name 'src'` (or `feature-name 'src' 'src'` for multiple origins) entries:
- `accelerometer`
- `ambient-light-sensor`
- `autoplay`
- `battery`
- `camera`
- `display-capture`
- `document-domain`
- `encrypted-media`
- `execution-while-not-rendered`
- `execution-while-out-of-viewport`
- `fullscreen`
- `gamepad`
- `geolocation`
- `gyroscope`
- `hid`
- `identity-credentials-get`
- `idle-detection`
- `local-fonts`
- `magnetometer`
- `microphone`
- `midi`
- `otp-credentials`
- `payment`
- `picture-in-picture`
- `publickey-credentials-create`
- `publickey-credentials-get`
- `screen-wake-lock`
- `serial`
- `speaker-selection`
- `storage-access`
- `usb`
- `web-share`
- `window-management`
- `xr-spatial-tracking`

Example: `camera 'self'; microphone https://other.example`."#),
    csp(r#"Content Security Policy applied to the embedded document.

A Content Security Policy string (the same syntax as an HTTP `Content-Security-Policy` header). The policy is enforced in addition to any other policy the document is delivered with."#));
add_bool_methods!(HtmlIframe,
    allowfullscreen(r#"Boolean attribute. When present, allows the embedded document to call `requestFullscreen()`.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. The Permissions Policy `allow="fullscreen"` attribute is the modern replacement."#),
    credentialless(r#"Boolean attribute. When present, the iframe loads without any user credentials, cookies, or storage sent with requests. The origin is anonymized so it cannot be used to access cross-origin partitioned storage.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#));
define_html_element!(HtmlEmbed, "embed", all,
    src(r#"URL of the embedded resource.

The type of the resource is determined by the optional `type` attribute."#),
    type_attr(r#"MIME type of the embedded resource (e.g. `application/pdf`, `image/svg+xml`).

Used to select a plugin; the resource at `src` is rendered with a plugin matching this type."#),
    width(r#"Display width in CSS pixels (a valid non-negative integer)."#),
    height(r#"Display height in CSS pixels (a valid non-negative integer)."#));
define_html_element!(HtmlObject, "object", all,
    data(r#"URL of the resource.

Specify a `type` attribute when the type cannot be inferred reliably from the URL, to help the user agent pick a plugin before fetching."#),
    type_attr(r#"MIME type of the resource referenced by `data`.

Used to help the user agent select a plugin without downloading the resource first."#),
    name(r#"Name of the object, submitted with the form as part of the name/value pair.

Used for form-associated objects."#),
    form(r#"ID of the `<form>` element to associate this object with.

Allows `<object>` to participate in form submission even when not nested inside the form."#),
    width(r#"Display width in CSS pixels (a valid non-negative integer)."#),
    height(r#"Display height in CSS pixels (a valid non-negative integer)."#));
define_html_element!(HtmlVideo, "video", all,
    src(r#"URL of the video to play.

An alternative to nesting `<source>` elements, which is preferred for serving multiple formats or fallbacks."#),
    poster(r#"URL of an image to display before the video starts playing.

A typical use is a thumbnail or "play" overlay."#),
    preload(r#"Hint for how aggressively the user agent should preload the media.

One of:
- `none` (do not preload; the user is not expected to need it)
- `metadata` (preload only metadata, e.g. duration and dimensions)
- `auto` (the user agent may preload the whole media)

The attribute is ignored when `autoplay` is present."#),
    width(r#"Display width in CSS pixels (a valid non-negative integer)."#),
    height(r#"Display height in CSS pixels (a valid non-negative integer)."#),
    crossorigin(r#"CORS setting for the video request.

One of:
- `anonymous`
- `use-credentials`

Required to use the video with `<canvas>` in a non-CORS-disabled way."#));
add_bool_methods!(HtmlVideo,
    controls(r#"Boolean attribute. When present, the user agent displays its default media controls (play, pause, volume, fullscreen, etc.).

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    autoplay(r#"Boolean attribute. When present, the media begins playback as soon as it can do so without stopping.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Many browsers restrict autoplay to muted media or require user activation."#),
    loop_attr(r#"Boolean attribute. When present, the media automatically seeks back to the start after reaching the end.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    muted(r#"Boolean attribute. When present, the audio output is silenced by default.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Required for autoplay in many browsers."#),
    playsinline(r#"Boolean attribute. When present, hints that the video should play inline (within the page) rather than entering the platform's native fullscreen player on iOS.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#));
define_html_element!(HtmlAudio, "audio", all,
    src(r#"URL of the audio to play.

An alternative to nesting `<source>` elements."#),
    preload(r#"Hint for how aggressively the user agent should preload the media.

One of:
- `none` (do not preload)
- `metadata` (preload only metadata)
- `auto` (the user agent may preload the whole media)

Ignored when `autoplay` is present."#),
    crossorigin(r#"CORS setting for the audio request.

One of:
- `anonymous`
- `use-credentials`"#));
add_bool_methods!(HtmlAudio,
    controls(r#"Boolean attribute. When present, the user agent displays its default media controls.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    autoplay(r#"Boolean attribute. When present, audio playback begins as soon as it can without stopping.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Many browsers restrict autoplay to muted media or require user activation."#),
    loop_attr(r#"Boolean attribute. When present, audio automatically seeks back to the start after reaching the end.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#),
    muted(r#"Boolean attribute. When present, the audio output is silenced by default.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Required for autoplay in many browsers."#));
define_html_element!(HtmlTrack, "track", aria_hidden_only,
    src(r#"URL of the track file (e.g. a WebVTT `.vtt` file for subtitles or captions)."#),
    kind(r#"Type of text track.

One of:
- `subtitles` (translation; default)
- `captions` (transcription of dialog and important sounds)
- `descriptions` (audio description of visual content)
- `chapters` (chapter titles, navigable)
- `metadata` (script-only data, not shown to the user)"#),
    srclang(r#"Language of the track text as a BCP 47 language tag (e.g. `en`, `fr`).

Required when `kind` is `subtitles`."#),
    label(r#"User-visible title for the track; shown in the user agent's caption picker."#));
add_bool_methods!(HtmlTrack,
    default_attr(r#"Boolean attribute. When present, the track is enabled by default if the user has not chosen another.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Only one `<track>` per media element may have this attribute."#));
define_html_element!(HtmlMap, "map", no_aria, name(r##"Name of the image map.

Referenced by `usemap` on an `<img>` or `<object>` with a leading `#` (e.g. `usemap="#nav-map"`). The name must be unique and not empty."##));
define_html_element!(HtmlArea, "area", all,
    alt(r#"Alternative text for the area.

Required when `href` is present; describes the link's destination for screen readers and is shown when the image cannot be loaded."#),
    coords(r#"Coordinates of the area, expressed as a comma-separated list of integers.

The number and meaning of the values depends on the `shape` attribute:
- `rect`: `x1, y1, x2, y2` (in CSS pixels from the image origin)
- `circle`: `x, y, radius`
- `poly`: `x1, y1, x2, y2, ..., xn, yn` (at least 6 values)
- `default`: the entire image; no coordinates"#),
    shape(r#"Shape of the clickable area.

One of:
- `rect` (rectangle; default)
- `circle`
- `poly` (polygon)
- `default` (the entire image beyond any other `<area>`)"#),
    href(r#"URL the area links to.

If absent, the area is "dead" (no link)."#),
    target(r#"Browsing context for the link.

One of:
- `_self` (default)
- `_blank`
- `_parent`
- `_top`
- a navigable target name"#),
    rel(r#"Relationship between the current document and the linked resource.

A space-separated list of link types. Common values:
- `alternate`
- `author`
- `bookmark`
- `external`
- `help`
- `license`
- `next`
- `nofollow`
- `noopener`
- `noreferrer`
- `prev`
- `search`
- `tag`

`noopener` and `noreferrer` are recommended for `target="_blank"`."#),
    ping(r#"Space-separated list of URLs to ping with a `POST` request when the link is followed.

Used for click-through tracking. The pings are sent in the background, do not block navigation, and are subject to referrer policy."#),
    referrerpolicy(r#"Referrer policy for the request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#),
    media(r#"Media query for which the link applies (e.g. `screen`, `print`).

Accepts any valid media query list."#));
add_bool_methods!(HtmlArea,
    download(r#"If present, the linked resource is downloaded instead of being navigated to. The value, if provided, is the suggested file name.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. When a non-empty value is provided, it suggests a default filename for the download."#));

factory!(
    /// Create a new [`HtmlPicture`] element (`<picture>`).
    picture, HtmlPicture
);
factory!(
    /// Create a new [`HtmlSource`] element (`<source>`).
    source, HtmlSource
);
factory!(
    /// Create a new [`HtmlImg`] element (`<img>`).
    img, HtmlImg
);
factory!(
    /// Create a new [`HtmlIframe`] element (`<iframe>`).
    iframe, HtmlIframe
);
factory!(
    /// Create a new [`HtmlEmbed`] element (`<embed>`).
    embed, HtmlEmbed
);
factory!(
    /// Create a new [`HtmlObject`] element (`<object>`).
    object, HtmlObject
);
factory!(
    /// Create a new [`HtmlVideo`] element (`<video>`).
    video, HtmlVideo
);
factory!(
    /// Create a new [`HtmlAudio`] element (`<audio>`).
    audio, HtmlAudio
);
factory!(
    /// Create a new [`HtmlTrack`] element (`<track>`).
    track, HtmlTrack
);
factory!(
    /// Create a new [`HtmlMap`] element (`<map>`).
    map, HtmlMap
);
factory!(
    /// Create a new [`HtmlArea`] element (`<area>`).
    area, HtmlArea
);

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
    }

    #[test]
    fn img_boolean_attrs_table() {
        let cases = [("ismap", img().ismap().render(), r#"<img ismap>"#)];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
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
        assert_eq!(iframe().csp("default-src 'self'").render(), r#"<iframe csp="default-src 'self'"></iframe>"#);
    }

    #[test]
    fn iframe_boolean_attrs_table() {
        let cases = [
            ("allowfullscreen", iframe().allowfullscreen().render(), r#"<iframe allowfullscreen></iframe>"#),
            ("credentialless", iframe().credentialless().render(), r#"<iframe credentialless></iframe>"#),
        ];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
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
    }

    #[test]
    fn video_attrs() {
        assert_eq!(video().src("vid.mp4").render(), r#"<video src="vid.mp4"></video>"#);
        assert_eq!(video().poster("thumb.jpg").render(), r#"<video poster="thumb.jpg"></video>"#);
        assert_eq!(video().preload("auto").render(), r#"<video preload="auto"></video>"#);
        assert_eq!(video().width("640").render(), r#"<video width="640"></video>"#);
        assert_eq!(video().height("480").render(), r#"<video height="480"></video>"#);
        assert_eq!(video().crossorigin("anonymous").render(), r#"<video crossorigin="anonymous"></video>"#);
    }

    #[test]
    fn video_boolean_attrs_table() {
        let cases = [
            ("controls", video().controls().render(), r#"<video controls></video>"#),
            ("autoplay", video().autoplay().render(), r#"<video autoplay></video>"#),
            ("loop", video().loop_attr().render(), r#"<video loop></video>"#),
            ("muted", video().muted().render(), r#"<video muted></video>"#),
            ("playsinline", video().playsinline().render(), r#"<video playsinline></video>"#),
        ];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }

    #[test]
    fn audio_attrs() {
        assert_eq!(audio().src("sound.mp3").render(), r#"<audio src="sound.mp3"></audio>"#);
        assert_eq!(audio().preload("auto").render(), r#"<audio preload="auto"></audio>"#);
        assert_eq!(audio().crossorigin("anonymous").render(), r#"<audio crossorigin="anonymous"></audio>"#);
    }

    #[test]
    fn audio_boolean_attrs_table() {
        let cases = [
            ("controls", audio().controls().render(), r#"<audio controls></audio>"#),
            ("autoplay", audio().autoplay().render(), r#"<audio autoplay></audio>"#),
            ("loop", audio().loop_attr().render(), r#"<audio loop></audio>"#),
            ("muted", audio().muted().render(), r#"<audio muted></audio>"#),
        ];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }

    #[test]
    fn track_attrs() {
        assert_eq!(track().src("subs.vtt").render(), r#"<track src="subs.vtt">"#);
        assert_eq!(track().kind("subtitles").render(), r#"<track kind="subtitles">"#);
        assert_eq!(track().srclang("en").render(), r#"<track srclang="en">"#);
        assert_eq!(track().label("English").render(), r#"<track label="English">"#);
    }

    #[test]
    fn track_boolean_attrs_table() {
        let cases = [("default", track().default_attr().render(), r#"<track default>"#)];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
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
        assert_eq!(area().ping("/track").render(), r#"<area ping="/track">"#);
        assert_eq!(area().referrerpolicy("no-referrer").render(), r#"<area referrerpolicy="no-referrer">"#);
        assert_eq!(area().media("screen").render(), r#"<area media="screen">"#);
    }

    #[test]
    fn area_boolean_attrs_table() {
        let cases = [("download", area().download().render(), r#"<area download>"#)];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }
}
