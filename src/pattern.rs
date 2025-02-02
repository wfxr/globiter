use anyhow::{bail, Result};
use itertools::Itertools;
use std::mem::take;

use super::token::Token;

#[derive(Debug, Clone)]
pub struct Pattern<'a> {
    original: &'a str,
    tokens: Vec<Token<'a>>,
}

#[derive(PartialEq)]
enum State<'a> {
    Plain,
    InSet(Vec<&'a str>),
}

impl<'a> Pattern<'a> {
    pub fn parse(s: &str) -> Result<Pattern> {
        let mut pattern = Pattern {
            original: s,
            tokens: Vec::new(),
        };
        let mut state = State::Plain;
        let (mut start, mut end) = (0, 0);
        for (idx, char) in s.char_indices() {
            let next_idx = idx + char.len_utf8();
            match char {
                '{' => match &mut state {
                    State::Plain => {
                        state = State::InSet(Vec::new());
                        pattern.tokens.push(Token::new_plain(&s[start..end]));
                        (start, end) = (next_idx, next_idx);
                    }
                    State::InSet(_) => bail!("unexpected character '{{' at pos {}", idx),
                },
                '}' => match &mut state {
                    State::InSet(v) => {
                        let mut set = take(v);
                        state = State::Plain;
                        set.push((&s[start..end]).trim());
                        pattern.tokens.push(Token::new_set(set));
                        (start, end) = (next_idx, next_idx);
                    }
                    State::Plain => bail!("unexpected character '}}' at pos {}", idx),
                },
                ',' => match &mut state {
                    State::Plain => end = next_idx,
                    State::InSet(set) => {
                        set.push((&s[start..end]).trim());
                        (start, end) = (next_idx, next_idx);
                    }
                },
                _ => end = next_idx,
            }
        }
        if end > start {
            pattern.tokens.push(Token::Plain((&s[start..end]).trim()));
        }

        Ok(pattern)
    }

    pub fn as_str(&self) -> &str {
        self.original
    }

    pub fn iter(&self) -> impl Iterator<Item = String> + '_ {
        self.tokens
            .iter()
            .map(|v| v.iter())
            .multi_cartesian_product()
            .map(|v| v.join(""))
    }
}

impl<'a> TryFrom<&'a str> for Pattern<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Pattern::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let cases = vec![
            (
                "normal",
                "Hello, World!",
                vec![Token::new_plain("Hello, World!")],
            ),
            (
                "one set",
                "https://example.com/{a,b,c}/file",
                vec![
                    Token::new_plain("https://example.com/"),
                    Token::new_set(vec!["a", "b", "c"]),
                    Token::new_plain("/file"),
                ],
            ),
            (
                "two set",
                "https://example.com/{a,b,c}/file/{x,y,z}",
                vec![
                    Token::new_plain("https://example.com/"),
                    Token::new_set(vec!["a", "b", "c"]),
                    Token::new_plain("/file/"),
                    Token::new_set(vec!["x", "y", "z"]),
                ],
            ),
            (
                "two set with spaces",
                "https://example.com/{a, b , c }/file/{foo bar, 你好, fizzbuzz, 世界}",
                vec![
                    Token::new_plain("https://example.com/"),
                    Token::new_set(vec!["a", "b", "c"]),
                    Token::new_plain("/file/"),
                    Token::new_set(vec!["foo bar", "你好", "fizzbuzz", "世界"]),
                ],
            ),
        ];

        for (name, input, expected) in cases {
            let p = Pattern::parse(input)?;

            assert_eq!(p.original, input, "case {name}");
            assert_eq!(p.tokens, expected, "case {name}");
        }

        Ok(())
    }

    #[test]
    fn test_parse_error() -> Result<()> {
        let cases = vec![
            (
                "bad pattern 1",
                "/{{a, b}",
                "unexpected character '{' at pos 2",
            ),
            (
                "bad pattern 2",
                "/{a}}",
                "unexpected character '}' at pos 4",
            ),
        ];

        for (name, input, expected) in cases {
            assert_eq!(
                Pattern::parse(input).unwrap_err().to_string(),
                expected,
                "case {name}"
            )
        }

        Ok(())
    }

    #[test]
    fn test_iter() -> Result<()> {
        let cases = vec![
            ("normal", "Hello, World!", vec!["Hello, World!"]),
            (
                "one set",
                "https://example.com/{a,b,c}/file",
                vec![
                    "https://example.com/a/file",
                    "https://example.com/b/file",
                    "https://example.com/c/file",
                ],
            ),
            (
                "two set",
                "https://example.com/{a,b,c}/file/{x,y,z}",
                vec![
                    "https://example.com/a/file/x",
                    "https://example.com/a/file/y",
                    "https://example.com/a/file/z",
                    "https://example.com/b/file/x",
                    "https://example.com/b/file/y",
                    "https://example.com/b/file/z",
                    "https://example.com/c/file/x",
                    "https://example.com/c/file/y",
                    "https://example.com/c/file/z",
                ],
            ),
        ];

        for (name, input, expected) in cases {
            let p = Pattern::parse(input)?;

            assert_eq!(p.iter().collect::<Vec<_>>(), expected, "case {name}");
        }

        Ok(())
    }
}
