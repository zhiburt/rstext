use rstext;

#[test]
fn default_locale() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    let domain = locale.load("index").unwrap();
    assert_eq!(domain.get("greeting"), Some("Hello World"));
    assert_eq!(domain.get("timeline"), Some("2020-07-02"));
}

#[test]
fn locale_simplification() {
    let mut locale = rstext::Locale::new("example_locales", "en_UK").unwrap();
    assert_eq!(locale.code(), "en");
    let domain = locale.load("index").unwrap();
    assert_eq!(domain.get("greeting"), Some("Hello World"));
    assert_eq!(domain.get("timeline"), Some("2020-07-02"));
}

#[test]
fn locale_getd() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    locale.load("index").unwrap();
    assert_eq!(locale.getd("index", "greeting"), Some("Hello World"));
    assert_eq!(locale.getd("index", ""), None);
}
