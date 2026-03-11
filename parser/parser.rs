use pulldown_cmark::{Parser, Options, html};

/// Converts Markdown bytes to HTML bytes.
pub fn markdown_to_html(markdown_text: &[u8]) -> Vec<u8> {
    // 1. Convert bytes to string (Markdown is typically UTF-8)
    let markdown_str = std::str::from_utf8(markdown_text).unwrap_or("");

    // 2. Set up extensions (similar to parser.CommonExtensions)
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // 3. Initialize the parser
    let parser = Parser::new_ext(markdown_str, options);

    // 4. Render the AST directly to an HTML string
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 5. Convert back to bytes
    html_output.into_bytes()
}