use serde::de::{Deserialize, Deserializer};
use serde_json;

use error::*;
use query::Query;
use song;
use sunk::Sunk;
use util::*;

#[derive(Debug, Clone, Copy)]
pub enum ListType {
    AlphaByArtist,
    AlphaByName,
    Frequent,
    Highest,
    Newest,
    Random,
    Recent,
    Starred,
}

impl ::std::fmt::Display for ListType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::ListType::*;
        let fmt = match *self {
            AlphaByArtist => "alphabeticalByArtist",
            AlphaByName => "alphabeticalByName",
            Frequent => "frequent",
            Highest => "highest",
            Newest => "newest",
            Random => "random",
            Recent => "recent",
            Starred => "starred",
        };
        write!(f, "{}", fmt)
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub artist: Option<String>,
    artist_id: Option<u64>,
    cover_id: Option<String>,
    pub duration: u64,
    pub year: Option<u64>,
    pub genre: Option<String>,
    pub song_count: u64,
    songs: Vec<song::Song>,
}

/// Internal struct matching exactly what `serde` expects.
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct AlbumSerde {
    id: String,
    name: String,
    artist: Option<String>,
    artistId: Option<String>,
    coverArt: Option<String>,
    songCount: u64,
    duration: u64,
    created: String,
    year: Option<u64>,
    genre: Option<String>,
    song: Option<Vec<song::Song>>,
}

impl Album {
    pub fn songs(&self, sunk: &mut Sunk) -> Result<Vec<song::Song>> {
        if self.songs.len() as u64 != self.song_count {
            Ok(get_album(sunk, self.id)?.songs)
        } else {
            Ok(self.songs.clone())
        }
    }
}

impl<'de> Deserialize<'de> for Album {
    fn deserialize<D>(de: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = AlbumSerde::deserialize(de)?;

        Ok(Album {
            id: raw.id.parse().unwrap(),
            name: raw.name,
            artist: raw.artist,
            artist_id: raw.artistId.map(|i| i.parse().unwrap()),
            cover_id: raw.coverArt,
            duration: raw.duration,
            year: raw.year,
            genre: raw.genre,
            song_count: raw.songCount,
            songs: raw.song.unwrap_or_default(),
        })
    }
}

pub fn get_album(sunk: &mut Sunk, id: u64) -> Result<Album> {
    let res = sunk.get("getAlbum", Query::with("id", id))?;
    Ok(serde_json::from_value::<Album>(res)?)
}

pub fn get_albums(
    sunk: &mut Sunk,
    list_type: ListType,
    size: Option<u64>,
    offset: Option<u64>,
    folder_id: Option<u64>,
) -> Result<Vec<Album>> {
    let args = Query::new()
        .arg("type", list_type.to_string())
        .maybe_arg("size", map_str(size))
        .maybe_arg("offset", map_str(offset))
        .maybe_arg("musicFolderId", map_str(folder_id))
        .build();

    let res = sunk.get("getAlbumList2", args)?;

    let mut albums = vec![];
    if let Some(album_arr) = res["album"].as_array() {
        for album in album_arr.clone() {
            albums.push(serde_json::from_value::<Album>(album)?);
        }
    }
    Ok(albums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util;

    #[test]
    fn demo_get_albums() {
        let mut srv = test_util::demo_site().unwrap();
        let albums =
            get_albums(&mut srv, ListType::AlphaByArtist, None, None, None)
                .unwrap();

        println!("{:?}", albums);
        assert!(!albums.is_empty())
    }

    #[test]
    fn parse_album() {
        let parsed = serde_json::from_value::<Album>(raw()).unwrap();

        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, String::from("Bellevue"));
        assert_eq!(parsed.song_count, 9);
    }

    #[test]
    fn parse_album_deep() {
        let parsed = serde_json::from_value::<Album>(raw()).unwrap();

        assert_eq!(parsed.songs[0].id, 27);
        assert_eq!(parsed.songs[0].title, String::from("Bellevue Avenue"));
        assert_eq!(parsed.songs[0].duration, 198);
    }

    fn raw() -> serde_json::Value {
        json!({
         "id" : "1",
         "name" : "Bellevue",
         "artist" : "Misteur Valaire",
         "artistId" : "1",
         "coverArt" : "al-1",
         "songCount" : 9,
         "duration" : 1920,
         "playCount" : 2223,
         "created" : "2017-03-12T11:07:25.000Z",
         "genre" : "(255)",
         "song" : [ {
            "id" : "27",
            "parent" : "25",
            "isDir" : false,
            "title" : "Bellevue Avenue",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 1,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 5400185,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 198,
            "bitRate" : 216,
            "path" : "Misteur Valaire/Bellevue/01 - Misteur Valaire - Bellevue Avenue.mp3",
            "averageRating" : 3.0,
            "playCount" : 706,
            "created" : "2017-03-12T11:07:27.000Z",
            "starred" : "2017-06-01T19:48:25.635Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "31",
            "parent" : "25",
            "isDir" : false,
            "title" : "Don't Get Là",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 2,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 4866004,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 172,
            "bitRate" : 224,
            "path" : "Misteur Valaire/Bellevue/02 - Misteur Valaire - Don_t Get L.mp3",
            "playCount" : 310,
            "created" : "2017-03-12T11:07:28.000Z",
            "starred" : "2017-08-27T07:52:23.926Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "29",
            "parent" : "25",
            "isDir" : false,
            "title" : "Space Food",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 3,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 8954200,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 303,
            "bitRate" : 235,
            "path" : "Misteur Valaire/Bellevue/03 - Misteur Valaire - Space Food.mp3",
            "playCount" : 233,
            "created" : "2017-03-12T11:07:26.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "32",
            "parent" : "25",
            "isDir" : false,
            "title" : "Known By Sight (feat. Milk & Bone)",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 4,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 6219273,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 231,
            "bitRate" : 214,
            "path" : "Misteur Valaire/Bellevue/04 - Misteur Valaire - Known By Sight _feat. Milk _ Bone_.mp3",
            "playCount" : 216,
            "created" : "2017-03-12T11:07:27.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "33",
            "parent" : "25",
            "isDir" : false,
            "title" : "La Nature à Son Meilleur",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 5,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 5169929,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 187,
            "bitRate" : 220,
            "path" : "Misteur Valaire/Bellevue/05 - Misteur Valaire - La Nature  Son Meilleur.mp3",
            "playCount" : 190,
            "created" : "2017-03-12T11:07:26.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "34",
            "parent" : "25",
            "isDir" : false,
            "title" : "Interlude",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 6,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 2403983,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 99,
            "bitRate" : 191,
            "path" : "Misteur Valaire/Bellevue/06 - Misteur Valaire - Interlude.mp3",
            "playCount" : 149,
            "created" : "2017-03-12T11:07:28.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "28",
            "parent" : "25",
            "isDir" : false,
            "title" : "Old Orford",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 7,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 6403652,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 223,
            "bitRate" : 228,
            "path" : "Misteur Valaire/Bellevue/07 - Misteur Valaire - Old Orford.mp3",
            "playCount" : 160,
            "created" : "2017-03-12T11:07:25.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "30",
            "parent" : "25",
            "isDir" : false,
            "title" : "El Kid",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 8,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 6506923,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 234,
            "bitRate" : 221,
            "path" : "Misteur Valaire/Bellevue/08 - Misteur Valaire - El Kid.mp3",
            "playCount" : 134,
            "created" : "2017-03-12T11:07:28.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         }, {
            "id" : "26",
            "parent" : "25",
            "isDir" : false,
            "title" : "Banana Land",
            "album" : "Bellevue",
            "artist" : "Misteur Valaire",
            "track" : 9,
            "genre" : "(255)",
            "coverArt" : "25",
            "size" : 6870947,
            "contentType" : "audio/mpeg",
            "suffix" : "mp3",
            "duration" : 273,
            "bitRate" : 200,
            "path" : "Misteur Valaire/Bellevue/09 - Misteur Valaire - Banana Land.mp3",
            "playCount" : 125,
            "created" : "2017-03-12T11:07:25.000Z",
            "albumId" : "1",
            "artistId" : "1",
            "type" : "music"
         } ]
      })
    }
}
