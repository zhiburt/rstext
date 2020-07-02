use rstext::{self, Translator};

fn main() -> rstext::Result<()> {
    let mut t = Translator::new("example_locales")?;
    let domain = t.load_and_get("domain1")?;
    domain.load("de")?;
    let locale = domain.locale("de").unwrap();
    let greeting = locale.get("greeting");

    println!("{:?}", greeting);

    Ok(())
}
