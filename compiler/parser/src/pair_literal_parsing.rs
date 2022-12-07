use crate::source_position_marker::{SourceRange, CharWithSourcePosition};

#[derive(Debug)]
pub struct PairLiteralToken<'a> {
    pub range: SourceRange<'a>,
    pub token: PairLiteralInner,
}

#[derive(Debug)]
pub enum PairLiteralInner {
    String(String),
    Char(String),
    Unmatched(char),
}

#[derive(Debug)]
pub enum ParseError<'a> {
    UnmatchedString(SourceRange<'a>),
    UnmatchedChar(SourceRange<'a>),
}

pub fn parse_pairs<'a>(src: impl Iterator<Item = CharWithSourcePosition<'a>>) -> impl Iterator<Item = Result<PairLiteralToken<'a>, ParseError<'a>>> {
    struct PairLiteralIterator<I> {
        inner: I,
    }
    
    impl<'a, I> Iterator for PairLiteralIterator<I> where I: Iterator<Item = CharWithSourcePosition<'a>> {
        type Item = Result<PairLiteralToken<'a>, ParseError<'a>>;
    
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.inner.next()?;
            Some(self.inner_next(next))
        }
    }

    impl<'a, I> PairLiteralIterator<I> where I: Iterator<Item = CharWithSourcePosition<'a>> {
        fn inner_next(&mut self, next: CharWithSourcePosition<'a>) -> <Self as Iterator>::Item {
            let (inner, range) = match next.ch {
                '"' => self.find_pair(next.range, '"').map(|(s, e)| (PairLiteralInner::String(s), e))
                .map_err(|e| ParseError::UnmatchedString(e))?,
                '\'' => self.find_pair(next.range, '\'').map(|(s, e)| (PairLiteralInner::Char(s), e))
                    .map_err(|e| ParseError::UnmatchedChar(e))?,
                ch => (PairLiteralInner::Unmatched(ch), next.range),
            };
            let token = PairLiteralToken { range: range, token: inner };
            Ok(token)
        }

        fn find_pair(&mut self, begin: SourceRange<'a>, br: char) -> Result<(String, SourceRange<'a>), SourceRange<'a>> {
            let mut acc = String::new();
            let mut last_is_escape = false;
            let end = loop {
                match self.inner.next() {
                    Some(token) => {
                        if token.ch == br && !last_is_escape {
                            break token.range;
                        }
                        else {
                            if token.ch == '\\' && !last_is_escape {
                                last_is_escape = true;
                            }
                            else {
                                last_is_escape = false;
                                acc.push(token.ch);
                            }
                        }
                    },
                    None => return Err(begin)
                }
            };
            Ok((acc, begin.combine(end)))
        }
    }

    PairLiteralIterator { inner: src }
}


#[cfg(test)]
mod test {
    use crate::source_position_marker::from_source;

    use super::*;
    
    #[test]
    fn parse_no_pairs() {
        let src = "gerghgoog ernhgowngg ggrwe gwjg wejogwejog\n\nrngergu gergorjweg erg erg";
        let chars = from_source(src, "test.agr");
        let pairs = parse_pairs(chars);
        pairs.for_each(|t| {
            assert!(t.is_ok());
            let t = t.unwrap();
            match t.token {
                PairLiteralInner::String(_) => assert!(false),
                PairLiteralInner::Char(_) => assert!(false),
                PairLiteralInner::Unmatched(_) => assert!(true),
            }
        })
    }

    #[test]
    fn parse_string() {
        let src = "\"teststring\"";
        let chars = from_source(src, "test.agr");
        let mut pairs = parse_pairs(chars);
        let str = pairs.next().unwrap().unwrap().token;
        assert_eq!(pairs.count(), 0);
        match str {
            PairLiteralInner::String(s) => assert_eq!(s, "teststring"),
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_unmatched_string() {
        let src = "\"teststring";
        let chars = from_source(src, "test.agr");
        let mut pairs = parse_pairs(chars);
        let str = pairs.next().unwrap();
        assert_eq!(pairs.count(), 0);
        assert!(str.is_err());
    }

    #[test]
    fn parse_char() {
        let src = "'t'";
        let chars = from_source(src, "test.agr");
        let mut pairs = parse_pairs(chars);
        let ch = pairs.next().unwrap().unwrap().token;
        assert_eq!(pairs.count(), 0);
        match ch {
            PairLiteralInner::Char(s) => assert_eq!(s, "t"),
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_unmatched_char() {
        let src = "'teststring";
        let chars = from_source(src, "test.agr");
        let mut pairs = parse_pairs(chars);
        let ch = pairs.next().unwrap();
        assert_eq!(pairs.count(), 0);
        assert!(ch.is_err());
    }

    #[test]
    fn combined_test() {
        let src = "let s = \"str\" + 'a' + c";
        let chars = from_source(src, "test.agr");
        let _pairs = parse_pairs(chars);
        // TODO: Finish this test
    }
}