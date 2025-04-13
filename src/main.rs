mod backend;

use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use rspotify::prelude::OAuthClient;
use crate::backend::{Playlist, Pipe, Source, update_spotify};

mod frontend;

use color_eyre::Result;

fn main() -> Result<()> {
    // backend::main()
    frontend::main()
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

