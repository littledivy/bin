use rocket::{form::Form, response::Redirect};
use rocket_basicauth::BasicAuth;

use std::{fs, path::Path};

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;
use super::authenicate;

#[derive(FromForm)]
pub struct PasteIdForm {
    content: String,
    ext: String,
    name: Option<String>,
    edit: bool,
}

#[post("/submit", data = "<paste>")]
pub async fn submit(auth: BasicAuth, paste: Form<PasteIdForm>) -> Redirect {
    if !authenicate(auth) {
        return Redirect::to("/401");
    }

    let id = PasteId::new(paste.edit, paste.name.as_deref(), 6);

    let filepath = Path::new(&get_upload_dir()).join(format!("{id}", id = id));
    let content = &paste.content;
    let ext = &paste.ext;

    fs::write(filepath, content).expect("Unable to write to the file");

    Redirect::to(format!("/p/{id}.{ext}", id = id, ext = ext))
}
