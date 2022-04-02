use std::error::Error;
use std::fs::DirEntry;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use sha1::{Digest, Sha1};
use metadata::MediaFileMetadata;

fn file_sha(file: impl AsRef<Path>) -> Option<[u8; 20]> {
    let mut sha = sha1::Sha1::new();

    let mut file = std::fs::File::open(
        file,
    ).ok()?;

    let mut reader = BufReader::new(file);

    let mut buf: Vec<u8> = Vec::with_capacity(8192);

    unsafe { buf.set_len(8192); }

    buf.fill(0);

    let mut i = 0;

    loop {
        if let Ok(bytes) = reader.read(&mut buf) {
            if bytes == 0 {
                break;
            }

            sha.update(&buf[0..bytes]);

            i += 1;

            if i == 100 {
                reader.seek(SeekFrom::End(8192 * 100));

                continue;
            }

            continue;
        }

        break;
    }

    Some(sha.digest().bytes())
}


fn process_file(path: impl AsRef<Path>) -> Option<(MediaFileMetadata, String)> {
    let meta = metadata::MediaFileMetadata::new(&path)
        .ok()?
        .decode_frames(true)
        .ok()?
        .clone();

    meta.height?;
    meta.width?;

    let hash = hex::encode(
        file_sha(&path)
            .unwrap(),
    );

    let old_path = &path.as_ref().to_str()?.to_string();

    let new_path = &format!(
        "./{}-{}.{}",
        meta.height?,
        hash,
        path.as_ref().extension()?.to_str()?,
    );

    std::fs::rename(
        &path,
        Path::new(
            new_path,
        ),
    );

    println!(
        "{} -> {}",
        old_path,
        new_path,
    );

    None
}

fn runner() -> Result<(), Box<dyn Error>> {
    std::fs::read_dir(".")?
        .filter_map(|entry|
            if !entry.as_ref().ok()?.file_type().as_ref().ok()?.is_dir() {
                Some(entry.unwrap())
            } else {
                None
            }
        )
        .filter_map(|item| process_file(item.path()))
        .for_each(|item| {
            dbg!(item);
        });

    Ok(())
}

fn main() {
    runner();
}
