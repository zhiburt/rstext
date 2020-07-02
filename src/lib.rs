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

    pub fn domain<S: AsRef<str>>(&mut self, name: S) -> Result<DomainLocaled> {
        let locale = self.locale.clone();
        self.translator
            .domain(name)
            .and_then(|d| Ok(DomainLocaled { d, locale }))
    }
}

pub struct DomainLocaled<'a> {
    d: &'a mut Domain,
    locale: String,
}

impl DomainLocaled<'_> {
    pub fn get<S: AsRef<str>>(&mut self, id: S) -> Result<Option<&str>> {
        self.d
            .locale(&self.locale)?
            .get(id.as_ref())
            .map_or(Ok(None), |v| Ok(Some(v)))
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

    pub fn domain<S: AsRef<str>>(&mut self, name: S) -> Result<&mut Domain> {
        if self.domains.contains_key(name.as_ref()) {
            return Ok(self.domains.get_mut(name.as_ref()).unwrap());
        }

        let path = self.path.join(name.as_ref());
        match std::fs::metadata(&path) {
            Err(io) => return Err(TextError::from(io)),
            Ok(metadata) if metadata.is_file() => return Err(TextError::DomainNotFound),
            _ => (),
        };

        self.domains.insert(
            name.as_ref().to_owned(),
            Domain {
                path,
                locales: HashMap::new(),
            },
        );

        Ok(self.domains.get_mut(name.as_ref()).unwrap())
    }
}

pub struct Domain {
    locales: HashMap<String, po::Po>,
    path: PathBuf,
}

impl Domain {
    pub fn locale<S: AsRef<str>>(&mut self, locale: S) -> Result<&po::Po> {
        // @todo: resolve down casting of locale if it's not exists
        // ru_KZ -> ru
        let locale = locale.as_ref();

        if self.locales.contains_key(locale) {
            return Ok(&self.locales[locale]);
        }

        let path = self.path.join(locale.to_owned() + ".po");
        let file = std::fs::File::open(&path)?;

        let file = po::Po::parse(file)?;

        self.locales.insert(locale.to_owned(), file);

        Ok(&self.locales[locale])
    }
}
