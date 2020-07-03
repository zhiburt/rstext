use rstext::{self, Locale};

fn main() -> rstext::Result<()> {
    let mut locale = Locale::new("example_locales", "en")?;
    locale.load("domain1")?;
    let domain = locale.domain("domain1").unwrap();
    let greeting = domain.get("greeting").unwrap();

    println!("{:?}", greeting);

    Ok(())
}

