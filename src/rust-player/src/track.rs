use id3::{self, Tag, TagLike};

#[derive(Clone, Debug)]
pub struct Track {
    pub fpath: String,
    pub title: String,
    pub artist: String,
    pub album: String,
}

impl Track {
    pub fn new(fpath: String) -> Self {
        let meta = match Tag::read_from_path(&fpath) {
            Ok(m) => m,
            Err(_) => return Self {
                fpath,
                title: "Unknown".to_string(),
                artist: "Unknown".to_string(),
                album: "Unknown".to_string()
            }
        };

        let title = match meta.title() {
            Some(t) => t.to_string(),
            None => "Unknown".to_string()
        };
        let artist = match meta.artist() {
            Some(t) => t.to_string(),
            None => "Unknown".to_string()
        };
        let album = match meta.album() {
            Some(t) => t.to_string(),
            None => "Unknown".to_string()
        };

        return Self {fpath, title, artist, album}
    }
}
