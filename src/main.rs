mod lz77;

fn main() {
    let input = "abracadabra pé de cabra";
    println!("Texto original: {input}");

    let window_size = 10;
    let look_ahead = 5;

    let compressed = lz77::compress(input, window_size, look_ahead);
    println!("Tokens comprimidos:");
    for token in &compressed {
        println!("{token:?}");
    }

    let decompressed = lz77::decompress(&compressed);
    println!(
        "Texto restaurado: {}",
        String::from_utf8(decompressed).unwrap()
    );
}
