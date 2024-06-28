use std::collections::HashMap;

pub fn get_mime_types() -> HashMap<&'static str, &'static str> {
    let mut mime_types = HashMap::new();

    // text
    mime_types.insert("css", "text/css");
    mime_types.insert("csv", "text/csv");
    mime_types.insert("html", "text/html");
    mime_types.insert("htm", "text/html");
    mime_types.insert("ics", "text/calendar");
    mime_types.insert("js", "text/javascript");
    mime_types.insert("mjs", "text/javascript");

    // fonts
    mime_types.insert("woff", "font/woff");
    mime_types.insert("woff2", "font/woff2");
    mime_types.insert("otf", "font/otf");
    mime_types.insert("ttf", "font/ttf");


    // audio
    mime_types.insert("mid","audio/midi");
    mime_types.insert("midi", "audio/x-midi");
    mime_types.insert("aac", "audio/aac");

    // images
    mime_types.insert("apng", "image/apng");
    mime_types.insert("avif", "image/avif");
    mime_types.insert("gif", "image/gif");
    mime_types.insert("bmp", "image/bmp");
    mime_types.insert("ico", "image/vnd.microsoft.icon");
    mime_types.insert("jpeg", "image/jpeg");
    mime_types.insert("jpg", "image/jpeg");

    // video
    mime_types.insert("avi", "video/x-msvideo");
    mime_types.insert("mp4", "video/mp4");
    mime_types.insert("mpeg","video/mpeg");
    mime_types.insert("ogv", "video/gg");
    mime_types.insert("webm", "video/webm");

    // application
    mime_types.insert("bin", "application/octet-stream");
    mime_types.insert("json", "application/json");
    mime_types.insert("jsonld", "application/ld+json");
    mime_types.insert("jar", "application/java-archive");

    mime_types
}
