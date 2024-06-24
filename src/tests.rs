#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    fn setup() -> Config {
        let defaultconfig = Config {
            version: false,
            queries: vec![String::from("L")],
            fpath: String::from("test50.txt"),
            ignorecase: false,
            count: false,
            invert: false,
            pipe: false,
        };
        defaultconfig
    }

    #[test]
    fn onequery() {
        let defaultcfg = setup();
        let query = "gmp";
        let contents = "\
            srt
            gmp
            kmod";
        assert_eq!(
            vec!["            gmp"],
            search(defaultcfg.invert, query, contents)
        );
    }

    #[test]
    fn onequery_t50() {
        let defaultcfg = setup();
        let query = "qt";
        let contents = fs::read_to_string("test50.txt").unwrap();
        let expected = vec![
            "qt6-webview 6.7.1-1",
            "qt6-shadertools 6.7.1-1",
            "qt6-positioning 6.7.1-1",
            "appstream-qt 1.0.3-1",
            "qt6-websockets 6.7.1-1",
        ];
        assert_eq!(search(defaultcfg.invert, query, &contents[..]), expected);
    }

    #[test]
    fn onequery_t50_caps() {
        let defaultcfg = setup();
        let query = "QT";
        let contents = fs::read_to_string("test50.txt").unwrap();
        let expected: Vec<&str> = vec![];
        assert_eq!(search(defaultcfg.invert, query, &contents[..]), expected);
    }

    #[test]
    fn onequery_t50_inv() {
        let mut defaultcfg = setup();
        defaultcfg.invert = true;
        let query = "1";
        let contents = fs::read_to_string("test50.txt").unwrap();
        let expected = vec![
            "pcre 8.45-4",
            "dbus-broker-units 36-2",
            "uchardet 0.0.8-2",
            "gmp 6.3.0-2",
            "a52dec 0.8.0-2",
        ];
        assert_eq!(search(defaultcfg.invert, query, &contents[..]), expected);
    }

    #[test]
    fn onequery_t50_ign() {
        let mut defaultcfg = setup();
        defaultcfg.ignorecase = true;
        let query = "-QT";
        let contents = fs::read_to_string("test50.txt").unwrap();
        let expected = vec!["appstream-qt 1.0.3-1"];
        assert_eq!(ignsearch(defaultcfg.invert, query, &contents[..]), expected);
    }

    #[test]
    fn onequery_t50_count() {
        let mut defaultcfg = setup();
        defaultcfg.count = true;
        let query = "qt";
        let contents = fs::read_to_string("test50.txt").unwrap();
        let expected = 5;
        assert_eq!(
            search(defaultcfg.invert, query, &contents[..]).len(),
            expected
        );
    }
}
