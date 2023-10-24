#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // if let Some(ref mut remainder) = self.remainder { // same as line below
        if let Some(remainder) = &mut self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn take_until<'haystack>(source: &'haystack str, c: &'_ char) -> &'haystack str {
    // this string is going to be dealocated when this function returns
    // therefore we need to be able to distinguish between lifetime of the
    // 's' and the lifetime of the delimiter
    let delim = &format!("{c}");
    StrSplit::new(source, delim)
        .next()
        .expect("StrSplit always returns a value")
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn test_take_until() {
    let haystack = "a b c d ";
    assert_eq!(take_until(haystack, &'c'), "a b ");
}
