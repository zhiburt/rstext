mod error;
mod po;

pub use error::Result;

use error::TextError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Locale {
    path: PathBuf,
    domains: HashMap<String, po::Po>,
}

impl Locale {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(dir: P, locale: S) -> Result<Self> {
        let locale = locale.as_ref();
        let path = dir.as_ref().join(locale);
        match std::fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => Err(TextError::LocaleNotFound),
            Err(..) if locale.len() > 2 => Self::new(dir, &locale[..2]),
            _ => Ok(Self {
                path,
                domains: HashMap::new(),
            }),
        }
    }

    pub fn load<S: AsRef<str>>(&mut self, domain: S) -> Result<&po::Po> {
        let domain = domain.as_ref();
        let path = self.path.join(domain.to_owned() + ".po");
        let file = std::fs::File::open(&path)?;
        let po = po::Po::parse(file)?;
        self.domains.insert(domain.to_owned(), po);
        Ok(self.domain(domain).unwrap())
    }

    pub fn domain<S: AsRef<str>>(&self, domain: S) -> Option<&po::Po> {
        self.domains.get(domain.as_ref())
    }

    pub fn getd<S: AsRef<str>, S2: AsRef<str>>(&self, domain: S, id: S2) -> Option<&str> {
        self.domain(domain).and_then(|d| d.get(id.as_ref()))
    }

    pub fn code(&self) -> &std::ffi::OsStr {
        self.path.file_name().unwrap()
    }
}
