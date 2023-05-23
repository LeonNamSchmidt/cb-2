use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // TODO: Capture link definitions
    //catch link with url
    #[regex("<a([^>]*)href=\"([^\"]*)\"([^>]*)>([^<]*)</a[ \r\t\n]*>", extract_link_info)]
    Link((LinkUrl, LinkText)),

    // TODO: Ignore all characters that do not belong to a link definition    
    #[regex(".", logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // TODO: Implement extraction from link definition 
    let slice = lex.slice();
    //get url out of Token
    let url_start = slice.find("href=\"").unwrap() + 6;
    let url_end = slice[url_start..].find('"').unwrap();
    let url = &slice[url_start..url_end];
    //get linktext out of Token
    let text_start = slice.find('>').unwrap() + 1;
    let text_end = slice[text_start..].find('<').unwrap();
    let text = &slice[text_start..text_end];
    (LinkUrl(url.to_string()), LinkText(text.to_string()))
}
