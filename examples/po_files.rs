use rstext::{self, Translator};

fn main() -> rstext::Result<()> {
    let mut locale = Translator::new("example_locales")?;
    let domain = locale.domain("domain1")?;
    let locale = domain.locale("de")?;
    let greeting = locale.get("greeting");

    println!("{:?}", greeting);

    Ok(())
}
