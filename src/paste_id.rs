use crate::constants::ID_SIZE;
use rand::Rng;
use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// We could optimize this to create PasteIds that
/// do not create potential collisions as the service scales.
#[derive(UriDisplayPath)]
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
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "pastes");
        Path::new(root).join(self.0.as_ref())
    }
}

/// Returns a `PasteId` if the user-generated path segment represents
/// a valid ID. Otherwise, return the ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let satisifes_len = param.len() >= ID_SIZE;
        let satisfies_type = param.chars().all(|c| c.is_ascii_alphanumeric());
        let satisfies_identity =
            Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/", "pastes")).exists();

        match satisifes_len && satisfies_type && satisfies_identity {
            true => Ok(PasteId(param.into())),
            false => Err(param),
        }
    }
}
