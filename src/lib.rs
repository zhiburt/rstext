mod error;
mod po;

pub use error::Result;

use error::TextError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Locale {
    locale: String,
    translator: Translator,
}

impl Locale {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(path: P, locale: S) -> Result<Self> {
        Translator::new(path).and_then(|translator| {
            Ok(Self {
                locale: locale.as_ref().to_owned(),
                translator,
            })
        })
    }

    pub fn load<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
        let locale = &self.locale;
        self.translator.load(name.as_ref())?;
        self.translator
            .domain_mut(name.as_ref())
            .unwrap()
            .load(locale)
    }

    pub fn domain<S: AsRef<str>>(&self, name: S) -> Option<DomainLocaled> {
        self.translator.domain(name.as_ref()).and_then(|d| {
            Some(DomainLocaled {
                d,
                locale: &self.locale,
            })
        })
    }

    pub fn getd<S: AsRef<str>, S2: AsRef<str>>(&self, domain: S, id: S2) -> Option<String> {
        self.domain(domain)
            .and_then(|d| d.get(id.as_ref()).map(|s| s.to_owned()))
    }
}

pub struct DomainLocaled<'a> {
    d: &'a Domain,
    locale: &'a str,
}

impl DomainLocaled<'_> {
    pub fn get<S: AsRef<str>>(&self, id: S) -> Option<&str> {
        self.d
            .locale(self.locale)
            .and_then(|po| po.get(id.as_ref()))
    }
}

pub struct Translator {
    path: PathBuf,
    domains: HashMap<String, Domain>,
}

impl Translator {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            path: path.as_ref().into(),
            domains: HashMap::new(),
        })
    }

    pub fn load<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
        let name = name.as_ref();
        let path = self.path.join(name);
        match std::fs::metadata(&path) {
            Err(io) => return Err(TextError::from(io)),
            Ok(metadata) if metadata.is_file() => return Err(TextError::DomainNotFound),
            _ => (),
        };

        self.domains.insert(
            name.to_owned(),
            Domain {
                path,
                locales: HashMap::new(),
            },
        );
        Ok(())
    }

    pub fn load_and_get<S: AsRef<str>>(&mut self, domain: S) -> Result<&mut Domain> {
        self.load(domain.as_ref())?;
        match self.domain_mut(domain.as_ref()) {
            Some(domain) => Ok(domain),
            None => unreachable!(),
        }
    }

    pub fn domain<S: AsRef<str>>(&self, name: S) -> Option<&Domain> {
        self.domains.get(name.as_ref())
    }

    pub fn domain_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut Domain> {
        self.domains.get_mut(name.as_ref())
    }
}

pub struct Domain {
    locales: HashMap<String, po::Po>,
    path: PathBuf,
}

impl Domain {
    pub fn locale<S: AsRef<str>>(&self, locale: S) -> Option<&po::Po> {
        // @todo: resolve down casting of locale if it's not exists
        // ru_KZ -> ru
        self.locales.get(locale.as_ref())
    }

    pub fn load<S: AsRef<str>>(&mut self, locale: S) -> Result<()> {
        let locale = locale.as_ref();
        let path = self.path.join(locale.to_owned() + ".po");
        let file = std::fs::File::open(&path)?;

        let file = po::Po::parse(file)?;

        self.locales.insert(locale.to_owned(), file);
        Ok(())
    }
}
