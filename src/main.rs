pub mod utils;

use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    usbdrive: String,
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    println!("Hello from retag!\n\nScanning files...");
    let args = Cli::parse();
    let usb = args.usbdrive;
    let media = utils::walk_additional_dir(usb);
    println!("Found {} files", media.len());
    let mut count = 0;
    for m in media.clone() {
        count += 1;

        let ck = utils::Mp3Checks { apath: m.clone() };
        let artcheck = ck.artist_check();
        let albcheck = ck.album_check();
        let songcheck = ck.song_check();
        if artcheck == true && albcheck == true && songcheck == true {
            println!("All tags match");
        } else {
            println!("artcheck: {}", artcheck);
            println!("albcheck: {}", albcheck);
            println!("songcheck: {}", songcheck);
            panic!("Tags don't match\nHalting program until tags are fixed")
        }
    }
    for x in media.clone() {
        let atags = utils::RetagUtils { apath: x.clone() };
        let alltags = atags.all_tags();
        println!("alltags: {:?}", alltags);
    }

        // println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n{}", count)


    let duration = start.elapsed();
    println!("Processed {} files in {} seconds", count, duration.as_secs());


    Ok(())
}
