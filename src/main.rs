mod backend;

use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use rspotify::prelude::OAuthClient;
use crate::backend::{Playlist, Pipe, Source, update_spotify};

mod frontend;

use color_eyre::Result;

fn get_auth() -> Result<AuthCodeSpotify> {

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

fn main() -> Result<()> {

    color_eyre::install()?;

    let client = get_auth()?;

    let playlist = client
        .current_user_playlists()
        .find(|x| x.as_ref().unwrap().name ==  "My Playlist #2")
        .unwrap()?;

    let sources = vec![Pipe{ source: Source::Playlist(playlist.id.clone()), filters: vec![] }];

    let x = dbg!(Playlist {
        name: String::from("it works"),
        sources,
        is_shadow: false,
    });

    update_spotify(&[x], &client, client.me()?.id);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use rspotify::AuthCodeSpotify;
//     use super::*;
//
//     #[test]
//     fn it_works() {
//     }
// }

