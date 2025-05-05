use rspotify::{model, scopes, AuthCodeSpotify, Credentials, OAuth};
use rspotify::prelude::*;
use itertools::Itertools;
use rspotify::model::UserId;

const PREFIX: &str = "Autofy";

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub(crate) sources: Vec<Pipe>,
    // allow_duplicates: bool, // will be assumed false for now
    pub(crate) is_shadow: bool,
}

impl Playlist {
    fn get_songs(&self, client: &AuthCodeSpotify) -> Vec<PlayableId> {
        // if self.allow_duplicates {
        //     self.sources.iter().flat_map(|x| Pipe::get_songs(x, client))
        //     .collect()
        //
        // } else
        {
            self.sources.iter().flat_map(|x| Pipe::get_songs(x, client))
                .unique()
                .collect()
        }

    }
}

#[derive(Debug, Clone)]
pub struct Pipe {
    pub source: Source,
    pub filters: Vec<Filter>,
    pub name: String,
}

impl Pipe {
    fn get_songs(&self, client: &AuthCodeSpotify) -> Vec<PlayableId> {

        // // :(
        // self.filters.iter().fold(
        //     self.source.get_songs(client).into_iter().filter(|_| true),
        //     |songs, mut x| songs.filter(|song| x.matches(song, client))
        // ).collect()

        self.source.get_songs(client).into_iter().filter(
            |song| self.filters.iter().all(|x| x.matches(song, client))
        ).collect()
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    // this just seems like a playlist might be able to be shared
    Songs {raw_imports: Vec<model::PlayableId<'static>>, name: String},

    AutoPlaylist(Playlist),
    Playlist(model::PlaylistId<'static>),
    Album(model::AlbumId<'static>),
}

impl Source {
    fn get_songs(&self, client: &AuthCodeSpotify) -> Vec<PlayableId> {
        match self {
            Source::Songs {raw_imports, ..} => {raw_imports.clone()}
            Source::AutoPlaylist(x) => {x.get_songs(client)}
            Source::Playlist(id) => {
                client.playlist_items(id.clone(), None, None).filter_map(
                    |x| x.unwrap()
                        .track
                        .map(|y| y.id().unwrap().clone_static())
                ).collect()
            }
            Source::Album(id) => {
                client.album_track(id.clone(), None, ).map(
                    |x| PlayableId::Track(x.unwrap().id.unwrap())
                ).collect()
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum Filter {
}

impl Filter {

    pub(crate) fn name(&self) -> &str {
        todo!()
    }

    fn matches(&self, song: &PlayableId, client: &AuthCodeSpotify) -> bool {
        match self {
            _ => {todo!()}
        }
    }
}

pub fn update_spotify(playlists: &[Playlist], client: &AuthCodeSpotify, user_id: UserId) {

    let mut user_playlists = client.current_user_playlists();

    for playlist in playlists {

        if playlist.is_shadow {continue;}

        let songs = playlist.get_songs(client);

        // dbg!(&songs);

        let (id, preexisting_songs) = if let Some(Ok(precreated_playlist)) = user_playlists
            .find(|x|
                if let Ok(y) = x {
                    y.name == format!("{PREFIX} {}", playlist.name)
                } else {false}) {

            let mut correct = true;
            // precreated_playlist.id
            correct &= precreated_playlist.tracks.total as usize == songs.len();

            let current_items = client.playlist_items(precreated_playlist.id.clone(), None, None)
                .map(|x| x.unwrap().track.unwrap().id().unwrap().clone_static()).collect::<Vec<PlayableId>>();

            correct &= songs.iter().all(|x| current_items.contains(x));

            if correct {
                dbg!("skipped", playlist);
                continue;
            }  else {(precreated_playlist.id, current_items)}

        } else {
            (client.user_playlist_create(user_id.clone(),
                                        &format!("{PREFIX} {}", playlist.name),
                                        None,
                                        None,
                                        None)
                .unwrap().id, Vec::new())
        };

        let mut songs_to_add = songs.iter()
            .filter(|x| !preexisting_songs.contains(x)).map(PlayableId::clone)
            .fold(vec![Vec::new()], |mut array, x| {
                if array.last().unwrap().len() == 100 {
                    array.push(Vec::new());
                }
                array.last_mut().unwrap().push(x);
                array
            });

        for group in songs_to_add {
            client.playlist_add_items(id.clone(),
                                      group,
                                      None,
            ).unwrap();
        }

        assert_eq!(songs.len(),
            client.playlist_items(id, None, None).map(|x| x.unwrap()).collect::<Vec<_>>().len()
        )

    }



}

pub fn get_auth() -> color_eyre::Result<AuthCodeSpotify> {

    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!(
        "playlist-read-private",
        "playlist-modify-private",
        "playlist-modify-public"
    )).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false)?;

    spotify.prompt_for_token(&url)?;

    Ok(spotify)
}

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = get_auth()?;

    let playlist = client
        .current_user_playlists()
        .find(|x| x.as_ref().unwrap().name ==  "My Playlist #2")
        .unwrap()?;

    let sources = vec![
        Pipe {
            source: Source::Playlist(playlist.id.clone()),
            filters: vec![],
            name: playlist.name
        }];

    let x = dbg!(Playlist {
        name: String::from("it works"),
        sources,
        is_shadow: false,
    });

    update_spotify(&[x], &client, client.me()?.id);

    Ok(())
}