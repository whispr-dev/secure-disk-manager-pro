use secure_diskmanager_ops::{compression, disk_management, entropy, rng, search, SdmError};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_demo_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("secure_diskmanager_ops_demo_{stamp}"))
}

fn main() -> secure_diskmanager_ops::Result<()> {
    rng::initialize()?;

    let demo_dir = unique_demo_dir();
    fs::create_dir_all(&demo_dir)?;

    let input = demo_dir.join("demo_input.txt");
    let compressed = demo_dir.join("demo_input.txt.z");
    let roundtrip = demo_dir.join("demo_roundtrip.txt");
    let keyfile = demo_dir.join("demo.key");

    fs::write(
        &input,
        b"Secure DiskManager Rust demo data.\nCompression roundtrip check.\n",
    )?;

    let immediate_bytes = disk_management::get_disk_usage(&demo_dir)?;
    println!("demo dir immediate file bytes before compression: {immediate_bytes}");

    let entropy_score = entropy::calculate_file_entropy(&input)?;
    println!("input entropy: {entropy_score:.6}");

    let hits = search::find_matching_files(&demo_dir, r"\.txt$", true)?;
    println!("search hits:\n{}", search::format_search_results(&hits));

    compression::compress(&input, &compressed)?;
    compression::decompress(&compressed, &roundtrip)?;

    let original = fs::read(&input)?;
    let recovered = fs::read(&roundtrip)?;
    if original != recovered {
        return Err(SdmError::Compression(
            "demo compression roundtrip mismatch".to_string(),
        ));
    }

    rng::write_keyfile(&keyfile, 32)?;
    println!("wrote demo keyfile: {}", keyfile.display());
    println!("compression roundtrip OK: {} -> {}", input.display(), roundtrip.display());

    let _ = fs::remove_dir_all(&demo_dir);
    Ok(())
}
