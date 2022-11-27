use lazy_static::lazy_static;
use regex::Regex;

use crate::fenced_attributes::attributes::Attributes;

lazy_static! {
    pub static ref CODE_FENCED_CHUNK: Regex = Regex::new(r"^ *(`{3,}([^\n]*)|~{3,})").unwrap();
    pub static ref CODE_CHUNK_IN_BRACKETS: Regex =
        Regex::new(r"^ *(`{3,}|~{3,})([^\s{]*)\s*\{(.*?)\}").unwrap();
    pub static ref ONLY_LANG: Regex = Regex::new(r"^ *(`{3,}|~{3,})([^\s]+)").unwrap();
}

/// Code chunk information
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CodeChunk {
    /// code chunk language
    pub lang: Option<String>,

    /// code chunk attributes
    pub attributes: Attributes,
}

impl CodeChunk {
    /// build a CodeChunk from a string containing the fences line
    pub fn parse_fences(line: String) -> Option<Self> {
        if CODE_FENCED_CHUNK.is_match(&line) {
            let line = line.trim();
            let captures = if line.contains("{") {
                CODE_CHUNK_IN_BRACKETS.captures(line)
            } else {
                ONLY_LANG.captures(line)
            };
            let (language, attributes_string) = captures.map_or((None, None), |m| {
                let lang = m.get(2).map(|v| v.as_str());
                let attrs = m.get(3).map(|v| v.as_str());
                (lang, attrs)
            });
            language.map(|_l| {
                let attr = attributes_string.unwrap_or("");
                let attributes = Attributes::parse(attr).unwrap_or_default();
                CodeChunk {
                    lang: language.map(|l| String::from(l)),
                    attributes,
                }
            })
        } else {
            None
        }
    }
}

#[test]
fn indented_code_fences() {
    let fences = String::from("```");
    let chunk = CodeChunk::parse_fences(fences);
    assert_eq!(chunk.is_none(), true);

    let fences = String::from("```java");
    let chunk = CodeChunk::parse_fences(fences).unwrap();
    assert_eq!(chunk.lang, Some(String::from("java")));
    assert_eq!(chunk.attributes.is_empty(), true);

    let fences = String::from("```bash {id: test, sys=[macos], args=[test1, test2]}}");
    let chunk = CodeChunk::parse_fences(fences).unwrap();
    assert_eq!(chunk.lang, Some(String::from("bash")));
    assert_eq!(chunk.attributes.is_empty(), false);
}
