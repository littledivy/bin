use std::borrow::Cow;
use std::fmt;

use rocket::request::FromParam;

use rand::{self, distributions::Alphanumeric, Rng};

pub struct PasteId<'a>(Cow<'a, str>);

fn valid_id(id: &str) -> bool {
    let mut parts = id.split('-');
    // Last part must be alphanumeric
    let id = parts.last().unwrap();
    id.chars().all(char::is_alphanumeric)
}

impl<'a> PasteId<'a> {
    pub fn new(name: Option<&str>, size: usize) -> PasteId<'static> {
        if let Some(name) = name {
            if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
                panic!("Invalid name");
            }
        }

        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();

        PasteId(Cow::Owned(match name {
            Some(name) => format!("{}-{}", name, id),
            None => id,
        }))
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match valid_id(param) {
            true => Ok(PasteId(Cow::Borrowed(param))),
            false => Err(param),
        }
    }
}

impl<'a> fmt::Display for PasteId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
