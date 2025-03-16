use rspotify::model;

pub struct Playlist {
    name: String,
    sources: Vec<(Source, Vec<model::PlayableItem>)>,
    allow_duplicates: bool,
}

pub enum Source {
    Songs(Vec<model::PlayableItem>),
    Playlist(model::playlist),
    Album(model::album)
}