pub mod utils;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    usbdrive: String,
}

fn main() -> std::io::Result<()> {
    println!("Hello Retagger!");
    let args = Cli::parse();
    let usb = args.usbdrive;
    let media = utils::walk_additional_dir(usb);
    let mut count = 0;
    for m in media{
        println!("{}", m.clone());
        count += 1;
        let dir_artist = utils::dir_artist(m.clone());
        let filename_artist = utils::filename_artist(m.clone());
        let ta = utils::tag_artist(m.clone());
        let tag_artist = ta.replace(" ", "_");
        if dir_artist != filename_artist && filename_artist != tag_artist && dir_artist != tag_artist {
            println!("Artists don't match");
            println!("dir_artist: {}", dir_artist);
            println!("filename_artist: {}", filename_artist);
            println!("tag_artist: {}", tag_artist);
            println!("{}", m.clone());
        }


        let dir_album = utils::dir_album(m.clone());
        let filename_album = utils::filename_album(m.clone());
        let tag_album = utils::tag_album(m.clone());
        let tag_alb = tag_album.replace(" ", "_");
        if dir_album != filename_album && filename_album != tag_alb && dir_album != tag_alb {
            println!("Albums don't match");
            println!("dir_album: {}", dir_album);
            println!("filename_album: {}", filename_album);
            println!("tag_album: {}", tag_alb);
            println!("{}", m.clone());
        }

        let filename_song = utils::filename_song(m.clone());
        let ts = utils::tag_song(m.clone());
        let tag_songs = ts.replace(" ", "_");
        if filename_song != tag_songs {
            println!("Songs don't match");
            println!("filename_song: {}", filename_song);
            println!("tag_song: {}", tag_songs);
            println!("{}", m.clone());
        }

    }

    println!("Processed {} files", count);


    Ok(())
}
