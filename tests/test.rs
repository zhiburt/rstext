use rstext;

#[test]
fn locale() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    let domain = locale.load("index").unwrap();
    assert_eq!(domain.get("greeting"), Some("Hello World"));
    assert_eq!(domain.get("timeline"), Some("2020-07-02"));
}

#[test]
fn locale_not_found() {
    let locale = rstext::Locale::new("example_locales", "bl");
    assert!(locale.is_err());
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
fn locale_simplification_not_found() {
    let locale = rstext::Locale::new("example_locales", "bl_00");
    assert!(locale.is_err());
}

#[test]
fn locale_getd() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    locale.load("index").unwrap();
    assert_eq!(locale.getd("index", "greeting"), Some("Hello World"));
    assert_eq!(locale.getd("index", ""), None);
}

#[test]
fn locale_get_message_from_context() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    let domain = locale.load("index").unwrap();
    assert_eq!(domain.getc("menu", "timeline"), Some("2020-07-02"));
}
