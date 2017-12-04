use std::collections::hash_set::HashSet;

pub fn is_valid(passphrase: &str) -> bool {
    let mut words = HashSet::new();
    let mut count = 0;
    for w in passphrase.split_whitespace() {
        if words.contains(w) {
            return false;
        }
        words.insert(w);
        count += 1;
    }
    count > 1
}

pub fn is_supervalid(passphrase: &str) -> bool {
    let mut words = HashSet::new();
    let mut count = 0;
    for w in passphrase.split_whitespace() {
        let mut v = w.bytes().collect::<Vec<u8>>();
        v.sort();
        let w = String::from_utf8(v).unwrap();
        if words.contains(&w) {
            return false;
        }
        words.insert(w);
        count += 1;
    }
    count > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let tests = vec![
            ("aa bb cc dd ee", true),
            ("aa bb cc dd aa", false),
            ("aa bb cc dd aaa", true),
        ];

        for t in tests {
            assert_eq!(is_valid(t.0), t.1);
        }
    }

    #[test]
    fn test_is_supervalid() {
        let tests = vec![
            ("abcde fghij", true),
            ("abcde xyz ecdab", false),
            ("a ab abc abd abf abj", true),
            ("iiii oiii ooii oooi oooo", true),
            ("oiii ioii iioi iiio", false),
        ];

        for t in tests {
            assert_eq!(is_supervalid(t.0), t.1);
        }
    }
}
