use rstext;

#[test]
fn default() {
    let mut locale = rstext::Translator::new("example_locales").unwrap();
    let domain = locale.domain("domain1").unwrap();
    assert_eq!(
        domain.locale("de").unwrap().get("greeting"),
        Some("Hallo Welt")
    );

    let locale = domain.locale("en").unwrap();
    assert_eq!(locale.get("greeting"), Some("Hello World"));
    assert_eq!(locale.get("timeline"), Some("2020-07-02"));
}

#[test]
fn default_locale() {
    let mut locale = rstext::Locale::new("example_locales", "en").unwrap();
    let mut domain = locale.domain("domain1").unwrap();
    assert_eq!(domain.get("greeting"), Ok(Some("Hello World")));
    assert_eq!(domain.get("timeline"), Ok(Some("2020-07-02")));
}
