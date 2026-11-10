use std::cmp;

type Token = (usize, usize, u8);

fn compress(input: &[u8], n: usize, window_size: i32, look_ahead: usize) -> Vec<Token> {
    let mut idx = 0;
    let mut tokens = vec![];
    while idx < n {
        let mut best_offset = 0;
        let mut best_length = 0;

        let search_start = cmp::max(0, idx as i32 - window_size) as usize;
        for i in search_start..idx {
            let mut length = 0;
            while idx + length < n
                && length < look_ahead
                && input[i + length] == input[idx + length]
            {
                length += 1;
            }

            if length > best_length {
                best_length = length;
                best_offset = idx - i;
            }
        }

        let next_char = if idx + best_length < n {
            input[idx + best_length]
        } else {
            0
        };

        tokens.push((best_offset, best_length, next_char));
        idx += best_length + 1;
    }

    tokens
}

fn decompress(tokens: &Vec<Token>) -> Vec<u8> {
    let mut decompressed = vec![];

    for (offset, length, next_char) in tokens {
        if *length > 0 {
            let start = decompressed.len() - offset;
            for i in 0..*length {
                decompressed.push(decompressed[start + i]);
            }
        }

        if *next_char > 0 {
            decompressed.push(*next_char);
        }
    }

    decompressed
}

fn main() {
    let input = "abracadabra pé de cabra";
    println!("Texto original: {input}");

    let input = input.as_bytes();

    let n = input.len();
    let window_size = 10;
    let look_ahead = 5;

    let compressed = compress(input, n, window_size, look_ahead);
    println!("Tokens comprimidos:");
    for (o, l, n) in &compressed {
        println!("{o} {l} {}", *n as char);
    }

    let decompressed = decompress(&compressed);
    println!(
        "Texto restaurado: {}",
        String::from_utf8(decompressed).unwrap()
    );
}
