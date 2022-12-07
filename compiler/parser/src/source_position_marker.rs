/// Represents the position of a token in a source file
/// as a range.
#[derive(Debug, Clone)]
pub struct SourceRange<'a> {
    pub uri: &'a str,
    pub begin: SourcePosition,
    pub end: SourcePosition,
}

impl SourceRange<'_> {
    pub fn combine(mut self, other: Self) -> Self {
        assert_eq!(self.uri, other.uri);
        self.end = other.end;
        self
    }
}

/// Represents a point in the source file.
/// This is zero indexed
#[derive(Debug, Clone, Copy)]
pub struct SourcePosition {
    pub line: usize,
    pub col: usize,
}

/// Represents a charachter in the original file
#[derive(Debug, Clone)]
pub struct CharWithSourcePosition<'a> {
    pub ch: char,
    pub range: SourceRange<'a>
}

pub fn from_source<'a>(src: &'a str, uri: &'a str) -> impl Iterator<Item = CharWithSourcePosition<'a>> + 'a {
    src.lines()
        .enumerate()
        // move forces the compiler to copy uri
        .flat_map(move |(line, line_src)| 
            line_src.chars()
                .enumerate()
                // move forces the compiler to copy line
                .map(move |(col, ch)| {
                    let pos = SourcePosition { line, col };
                    let range = SourceRange { uri, begin: pos, end: pos };
                    CharWithSourcePosition { ch, range }                   
                }))   
}

// This is too simple for tests ... Famous last words.