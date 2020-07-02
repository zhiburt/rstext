use rstext;

#[test]
fn default() {
    let mut locale = rstext::Translator::new("example_locales").unwrap();
    let domain = locale.load_and_get("domain1").unwrap();
    domain.load("de").unwrap();
    assert_eq!(
        domain.locale("de").unwrap().get("greeting"),
        Some("Hallo Welt")
    );

    domain.load("en").unwrap();
    let locale = domain.locale("en").unwrap();
    assert_eq!(locale.get("greeting"), Some("Hello World"));
    assert_eq!(locale.get("timeline"), Some("2020-07-02"));
}

#[test]
fn default_locale() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    locale.load("domain1").unwrap();
    let mut domain = locale.domain("domain1").unwrap();
    assert_eq!(domain.get("greeting"), Some("Hello World"));
    assert_eq!(domain.get("timeline"), Some("2020-07-02"));
}
