use rstext::{self, Locale};

fn main() -> rstext::Result<()> {
    let mut locale = Locale::new("example_locales", "en")?;
    let domain = locale.load("domain1")?;
    let greeting = domain.get("greeting").unwrap();

    println!("{:?}", greeting);

    Ok(())
}

