use rspotify::model;
use rspotify::AuthCodeSpotify;

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub sources: Vec<(Source, Vec<model::PlayableItem>)>,
    pub allow_duplicates: bool,
    pub is_shadow: bool,
}

#[derive(Debug, Clone)]
pub enum Source {
    // this just seems like a playlist might be able to be shared
    Songs{raw_imports: Vec<model::PlayableItem>, name: String},

    Playlist(model::playlist),
    Album(model::album)
}

fn update_spotify(playlists: Vec<Playlist>, client: &mut u32) {}