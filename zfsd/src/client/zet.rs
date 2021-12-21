use clap::{App, Arg};
use zfs::{DownloadDigest, zfs_download_digest_dir};

fn write_download_digest(digest: DownloadDigest) -> std::io::Result<()> {
    let uid = uuid::Uuid::new_v4();
    let fname = format!("{}/{}", zfs_download_digest_dir(), uid.to_string());
    if let Ok(bs) = serde_json::to_vec(&digest) {
        std::fs::write(&fname, &bs)?;
    } else {
        println!("Failed to serialise DownloadDigest -- aborting.")
    }
    Ok(())
}
fn parse_args() -> (String, String) {
    let args = App::new("zet: zfs utility to download files.")
        .arg(
            Arg::from_usage("-p, --path[PATH]...  'The path to download the file to.'")
                .required(true),
        )
        .arg(
            Arg::from_usage(
                "-k, --key=[KEY]...  'The key of the file to download.'",
            )
            .required(true),
        )
        .get_matches();

    (
        args.value_of("path").unwrap().to_string(),
        args.value_of("key").unwrap().to_string(),
    )
}

fn main() {
    let (path, key) = parse_args();
    let digest = DownloadDigest { path, key, pace: 0 };
    write_download_digest(digest).unwrap();
}
