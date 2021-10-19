use preon_engine::types::PreonVector;

// Just because I can

fn main() {
    let original = PreonVector::new(23f32, 532f32);
    let normalized = original.normalized();
    let restored = normalized * 532f32;

    println!(
        "
        Original: {}
        Normalized: {}
        Restored: {}
        ",
        original, normalized, restored
    );
}
