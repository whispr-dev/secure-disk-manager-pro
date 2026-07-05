use secure_diskmanager_ops::{compression, disk_management, rng, search};

fn main() -> secure_diskmanager_ops::Result<()> {
    rng::initialize()?;
    println!("immediate file bytes: {}", disk_management::get_disk_usage(".")?);

    let hits = search::find_matching_files(".", r"\.rs$", false)?;
    println!("{}", search::format_search_results(&hits));

    // compression::compress("input.bin", "input.bin.z")?;
    // compression::decompress("input.bin.z", "input.roundtrip.bin")?;
    let _ = &compression::compress;
    Ok(())
}
