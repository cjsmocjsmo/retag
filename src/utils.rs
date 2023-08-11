use walkdir::WalkDir;
use std::path::Path;
use id3::{Tag, TagLike};

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

pub fn dir_album(x: String) -> String {
    let path = Path::new(&x);
    let dpath = path.parent().unwrap().to_str().unwrap().to_string();
    let dsplit = dpath.split("/");
    let dvec: Vec<&str> = dsplit.collect();
    let album = dvec[dvec.len() - 1].to_string();
    // println!("dir_album: {}", album);

    album
}

pub fn dir_artist(x: String) -> String {
    let path = Path::new(&x);
    let dpath = path.parent().unwrap().to_str().unwrap().to_string();
    let path2 = Path::new(&dpath);
    let dpath2 = path2.parent().unwrap().to_str().unwrap().to_string();
    let dsplit = dpath2.split("/");
    let dvec: Vec<&str> = dsplit.collect();
    let artist = dvec[dvec.len() - 1].to_string();
    // println!("dir_artist: {}", artist);

    artist
}

pub fn filename_artist(x: String) -> String {
    let path = Path::new(&x);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let fsplit = filename.split("_-_");
    let fvec: Vec<&str> = fsplit.collect();
    let filename_artist = fvec[1].to_string();
    // println!("filename_artist: {}", filename_artist);

    filename_artist
}

pub fn filename_album(x: String) -> String {
    let path = Path::new(&x);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let fsplit = filename.split("_-_");
    let fvec: Vec<&str> = fsplit.collect();
    let filename_album = fvec[2].to_string();
    // println!("filename_album: {}", filename_album);

    filename_album
}

pub fn filename_song(x: String) -> String {
    let path = Path::new(&x);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let fsplit = filename.split("_-_");
    let fvec: Vec<&str> = fsplit.collect();
    let fs = fvec[3].to_string();
    let fs2 = fs.split(".mp3");
    let filename_song = fs2.collect::<Vec<&str>>()[0].to_string();
    // println!("filename_song: {}", filename_song);

    filename_song
}

pub fn tag_artist(x: String) -> String {
    let tag = Tag::read_from_path(&x).expect(&x);
    let artist = tag.artist().expect(&x);
    // println!("tag_artist: {}", artist.clone());

    artist.to_string()
}

pub fn tag_album(x: String) -> String {
    let tag = Tag::read_from_path(&x).expect(&x);
    let album = tag.album().expect(&x);
    // println!("tag_album: {}", album.clone());

    album.to_string()
}

pub fn tag_song(x: String) -> String {
    let tag = Tag::read_from_path(&x).expect(&x);
    let song = tag.title().expect(&x);
    // println!("tag_song: {}", song.clone());

    song.to_string()
}