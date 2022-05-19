use rand::{Rng};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// We should probably optimize this to create
/// PasteIds that do not create collisions as
/// the service scales.
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
