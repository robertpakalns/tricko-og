pub fn parse(raw_path: &str) -> (Option<&str>, Vec<(&str, &str)>) {
    if raw_path.is_empty() {
        return (None, vec![]);
    }

    let (path, query) = if let Some(i) = raw_path.find('?') {
        (&raw_path[..i], Some(&raw_path[i + 1..]))
    } else {
        (raw_path, None)
    };

    let path = if path.is_empty() { None } else { Some(path) };

    let query_params = query
        .map(|q| {
            q.split('&')
                .filter(|pair| !pair.is_empty())
                .filter_map(|pair| {
                    let mut split = pair.splitn(2, '=');
                    let key = split.next()?;
                    let value = split.next().unwrap_or("");
                    Some((key, value))
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);

    (path, query_params)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn no_query() {
        assert_eq!(parse("/home"), (Some("/home"), vec![]));
    }

    #[test]
    fn with_query() {
        assert_eq!(
            parse("/search?q=tricko"),
            (Some("/search"), vec![("q", "tricko")])
        );
    }

    #[test]
    fn multiple_query() {
        assert_eq!(
            parse("/search?q=tricko&type=redline"),
            (Some("/search"), vec![("q", "tricko"), ("type", "redline")])
        );
    }

    #[test]
    fn empty_path() {
        assert_eq!(parse(""), (None, vec![]));
    }

    #[test]
    fn only_query() {
        assert_eq!(parse("?param=value"), (None, vec![("param", "value")]));
    }

    #[test]
    fn question_at_end() {
        assert_eq!(parse("/home?"), (Some("/home"), vec![]));
    }

    #[test]
    fn query_with_empty_value() {
        assert_eq!(
            parse("/search?q=&sort=asc"),
            (Some("/search"), vec![("q", ""), ("sort", "asc")])
        );
    }

    #[test]
    fn query_without_equal_sign() {
        assert_eq!(parse("/search?q"), (Some("/search"), vec![("q", "")]));
    }
}
