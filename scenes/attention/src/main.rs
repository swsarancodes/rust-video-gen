use std::{fs, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from("target/attention");
    fs::create_dir_all(&out_dir)?;

    // 1. Build and write the master explainer reel (with audio SFX & dipped transitions)
    let reel = psychopomp_attention::build_reel(true)?;
    let reel_path = out_dir.join("attention.reel.json");
    fs::write(&reel_path, serde_json::to_string_pretty(&reel)? + "\n")?;
    eprintln!(
        "Wrote master reel to {} ({:.1}s, {} segments)",
        reel_path.display(),
        reel.duration_nanos() as f64 / 1e9,
        reel.segments.len()
    );

    // 2. Write individual segment scene plans
    for segment in &reel.segments {
        let seg_path = out_dir.join(format!("{}.json", segment.plan.id));
        segment.plan.write_or_print(Some(seg_path))?;
    }

    // 3. Build and write the interactive presentation deck (for `psychopomp plan present`)
    let deck = psychopomp_attention::build_deck()?;
    let deck_path = out_dir.join("deck.json");
    deck.write_with_slides(&deck_path)?;
    eprintln!(
        "Wrote interactive presentation deck to {} ({} slides)",
        deck_path.display(),
        deck.slides.len()
    );

    println!("\n=== Psychopomp Attention Explainer Generated ===");
    println!("Interactive presentation:");
    println!("  cargo run --release -- plan present target/attention/deck.json --theme neutral");
    println!("Render 60fps shutter-sampled video:");
    println!(
        "  cargo run --release -- plan render target/attention/attention.reel.json output/attention.mp4 --theme neutral"
    );

    Ok(())
}
