use id3::{Tag, TagLike};
use std::path::Path;
use walkdir::WalkDir;

pub fn walk_additional_dir(apath: String) -> Vec<String> {
    let mut musicvec = Vec::new();

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.contains("Music") {
                if fname.ends_with(".mp3") {
                    musicvec.push(fname.clone());
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    musicvec.clone()
}

#[derive(Debug)]
pub struct RetagUtils {
    pub apath: String,
}

#[derive(Debug)]
pub struct TagData {
    pub artist: String,
    pub album: String,
    pub song: String,
    pub picture: Vec<u8>,
}

impl RetagUtils {
    pub fn dir_album(&self) -> String {
        let path = Path::new(&self.apath);
        let dpath = path.parent().unwrap().to_str().unwrap().to_string();
        let dsplit = dpath.split("/");
        let dvec: Vec<&str> = dsplit.collect();
        let album = dvec[dvec.len() - 1].to_string();
        // println!("dir_album: {}", album);

        album
    }

    pub fn dir_artist(&self) -> String {
        let path = Path::new(&self.apath);
        let dpath = path.parent().unwrap().to_str().unwrap().to_string();
        let path2 = Path::new(&dpath);
        let dpath2 = path2.parent().unwrap().to_str().unwrap().to_string();
        let dsplit = dpath2.split("/");
        let dvec: Vec<&str> = dsplit.collect();
        let artist = dvec[dvec.len() - 1].to_string();
        // println!("dir_artist: {}", artist);

        artist
    }

    pub fn filename_artist(&self) -> String {
        let path = Path::new(&self.apath);
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let fsplit = filename.split("_-_");
        let fvec: Vec<&str> = fsplit.collect();
        let filename_artist = fvec[1].to_string();
        // println!("filename_artist: {}", filename_artist);

        filename_artist
    }

    pub fn filename_album(&self) -> String {
        let path = Path::new(&self.apath);
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let fsplit = filename.split("_-_");
        let fvec: Vec<&str> = fsplit.collect();
        let filename_album = fvec[2].to_string();
        // println!("filename_album: {}", filename_album);

        filename_album
    }

    pub fn filename_song(&self) -> String {
        let path = Path::new(&self.apath);
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let fsplit = filename.split("_-_");
        let fvec: Vec<&str> = fsplit.collect();
        let fs = fvec[3].to_string();
        let fs2 = fs.split(".mp3");
        let filename_song = fs2.collect::<Vec<&str>>()[0].to_string();
        // println!("filename_song: {}", filename_song);

        filename_song
    }

    pub fn tag_artist(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect(&self.apath);
        // println!("tag_artist: {}", artist.clone());

        artist.to_string()
    }

    pub fn tag_album(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        // println!("tag_album: {}", album.clone());

        album.to_string()
    }

    pub fn tag_song(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let song = tag.title().expect(&self.apath);
        // println!("tag_song: {}", song.clone());

        song.to_string()
    }

    pub fn all_tags(&self) -> TagData {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let song = tag.title().expect(&self.apath);
        let first_picture = tag.pictures().next().unwrap().data.clone();
        let tagdata = TagData {
            artist: artist.to_string(),
            album: album.to_string(),
            song: song.to_string(),
            picture: first_picture,
        };


        tagdata
    }
}

#[derive(Debug)]
pub struct Mp3Checks {
    pub apath: String,
}

impl Mp3Checks {
    pub fn artist_check(&self) -> bool {
        let ck = RetagUtils { apath: self.apath.to_string() };
        let dir_artist = ck.dir_artist();
        let filename_artist = ck.filename_artist();
        let ta = ck.tag_artist();
        let tag_artist = ta.replace(" ", "_");
        let mut result = true;
        if dir_artist != filename_artist
            && filename_artist != tag_artist
            && dir_artist != tag_artist
        {
            println!("Artists don't match");
            println!("dir_artist: {}", dir_artist);
            println!("filename_artist: {}", filename_artist);
            println!("tag_artist: {}", tag_artist);
            result = false;
        }

        result
    }

    pub fn album_check(&self) -> bool {
        let ck = RetagUtils { apath: self.apath.to_string() };
        let dir_album = ck.dir_album();
        let filename_album = ck.filename_album();
        let tag_album = ck.tag_album();
        let tag_alb = tag_album.replace(" ", "_");
        let mut result = true;
        if dir_album != filename_album && filename_album != tag_alb && dir_album != tag_alb {
            println!("Albums don't match");
            println!("dir_album: {}", dir_album);
            println!("filename_album: {}", filename_album);
            println!("tag_album: {}", tag_alb);
            result = false;
        }

        result
    }

    pub fn song_check(&self) -> bool {
        let ck = RetagUtils { apath: self.apath.to_string() };
        let filename_song = ck.filename_song();
        let ts = ck.tag_song();
        let tag_songs = ts.replace(" ", "_");
        let mut result = true;
        if filename_song != tag_songs {
            println!("Songs don't match");
            println!("filename_song: {}", filename_song);
            println!("tag_song: {}", tag_songs);
            result = false;
        }

        result
    }
}
