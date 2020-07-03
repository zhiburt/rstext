use rstext::{self, Translator};

fn main() -> rstext::Result<()> {
    let mut t = Translator::new("example_locales")?;
    let domain = t.load_and_get("domain1")?;
    domain.load("de")?;
    domain.load("en")?;
    let default_locale = domain.locale("en").unwrap();
    let locale = domain.locale("de").unwrap();
    let greeting = locale
        .get("greeting1")
        .or_else(|| default_locale.get("greeting"));

    println!("{:?}", greeting);

    Ok(())
}
