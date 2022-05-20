use rand::{Rng};
use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// We could optimize this to create PasteIds that
/// do not create potential collisions as the service scales.
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    /// Generates a new PasteId
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE[rng.gen::<usize>() % BASE.len()] as char)
        }

        PasteId(Cow::Owned(id))
    }

    /// Returns the path to the file linked to this PasteId
    /// from the local file system.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

/// Returns a `PasteId` if the user-generated path segment represents
/// a valid ID. Otherwise, return the ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    /// Todos:
    /// - Validate the length the PasteId (param)
    /// - Validate the existence of the paste file
    /// - Check the paste file against a list of blocklisted files
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}
