use crate::{error::TextError, Result};
use std::collections::HashMap;
use std::io::{BufRead, Read};

#[derive(Default)]
pub struct Po {
    entities: HashMap<String, String>,
}

impl Po {
    pub fn parse<R: Read>(reader: R) -> Result<Self> {
        let mut reader = std::io::BufReader::new(reader);
        let mut entities = HashMap::new();
        let mut line = String::new();

        enum State {
            None,
            Msgid(String),
            Entity { msgid: String, msgstr: String },
        }

        let mut state = State::None;
        loop {
            line.clear();
            let eof = reader.read_line(&mut line)?;
            if eof == 0 {
                match state {
                    State::Msgid(..) => return Err(TextError::FormatError),
                    State::Entity { msgid, msgstr } => {
                        entities.insert(msgid, msgstr);
                    }
                    // eof
                    _ => (),
                }
                break;
            }

            let is_empty_line = line.is_empty() || line.trim().is_empty();
            let is_comment = line.starts_with("#");
            if is_empty_line || is_comment {
                continue;
            }

            if line.starts_with("msgid") {
                // state is changed save privious entity
                // we can't save state emidiately after creating in order to suppot multi lines
                if let State::Entity { msgid, msgstr } = state {
                    entities.insert(msgid, msgstr);
                }

                let s: &str = line[5..].trim();
                let id = unqoute(s).map(|s| s.to_owned())?;

                state = State::Msgid(id);
                continue;
            }

            match state {
                State::Msgid(msgid) if line.starts_with("msgstr") => {
                    let s: &str = line[6..].trim();
                    let msgstr = unqoute(s).map(|s| s.to_owned())?;

                    state = State::Entity {
                        msgid: msgid.clone(),
                        msgstr,
                    };
                    continue;
                }
                // handle multiline entity
                State::Entity { ref mut msgstr, .. } if unqoute(line.trim()).is_ok() => {
                    msgstr.push_str(unqoute(line.trim()).unwrap());
                    continue;
                }
                // format error
                _ => return Err(TextError::FormatError),
            }
        }

        Ok(Self { entities })
    }

    pub fn get(&self, id: &str) -> Option<&str> {
        self.entities.get(id).and_then(|s| Some(s.as_str()))
    }
}

fn unqoute<'a>(s: &'a str) -> Result<&'a str> {
    if !s.starts_with("\"") || !s.ends_with("\"") {
        return Err(TextError::FormatError);
    }

    Ok(&s[1..s.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_po_file() {
        let file = "msgid \"ask_location_menu.next_button\"\n\
                          msgstr \"Next\"\n";
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("ask_location_menu.next_button"), Some("Next"));
    }

    #[test]
    fn parse_po_file_messy() {
        let file = "msgid     \"ask_location_menu.next_button\"   \n\
                          msgstr    \"Next\"   \n   ";
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("ask_location_menu.next_button"), Some("Next"));
    }

    #[test]
    fn parse_po_file_emptylines() {
        let file = r#"
msgid "ask_location_menu.next_button"
    
msgstr "Next""#;
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("ask_location_menu.next_button"), Some("Next"));
    }

    #[test]
    fn parse_po_file_comments() {
        let file = r#"
#  translator-comments
#. extracted-comments
#: reference…
#, flag…
msgid "ask_location_menu.next_button"
msgstr "Next""#;
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("ask_location_menu.next_button"), Some("Next"));
    }

    #[test]
    fn parse_po_file_empty() {
        let file = "";
        let po = Po::parse(file.as_bytes());
        assert!(po.is_ok());
    }

    #[test]
    fn parse_po_file_multi_entities() {
        let file = "msgid \"id1\"\n\
                          msgstr \"v1\"\n\
                          msgid \"id2\"\n\
                          msgstr \"v2\"\n";
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("id1"), Some("v1"));
        assert_eq!(po.get("id2"), Some("v2"));

        let file = "msgid \"id1\"\n\
                          msgstr \"v1\"\n\
                          \n\
                          msgid \"id2\"\n\
                          msgstr \"v2\"\n";
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("id1"), Some("v1"));
        assert_eq!(po.get("id2"), Some("v2"));
    }

    #[test]
    fn parse_po_file_multiline() {
        let file = "msgid \"id\"\n\
                          msgstr \"1\"\n\
                          \"2\"\n\
                          \"3\"\n";
        let po = Po::parse(file.as_bytes()).unwrap();
        assert_eq!(po.get("id"), Some("123"));
    }
}
