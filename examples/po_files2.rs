use rstext::{self, Locale};

fn main() -> rstext::Result<()> {
    let mut locale = Locale::new("example_locales", "en")?;
    let mut domain = locale.domain("domain1")?;
    let greeting = domain.get("greeting")?;

    println!("{:?}", greeting);

    Ok(())
}

