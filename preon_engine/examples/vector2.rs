use preon_engine::types::PreonVector;

// Just because I can

fn main() {
    let original = PreonVector::new(23.0, 532.0);
    let normalized = original.normalized();
    let restored = normalized * 532.0;

    println!("Original: {}", original);
    println!("Normalized: {}", normalized);
    println!("Restored: {}", restored);
}
