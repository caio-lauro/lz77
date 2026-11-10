use std::cmp;

#[derive(Debug)]
pub struct Token {
    offset: usize,
    length: usize,
    chr: u8,
}

impl Token {
    pub fn new(offset: usize, length: usize, chr: u8) -> Token {
        Token {
            offset,
            length,
            chr,
        }
    }
}

pub fn compress(input: &str, window_size: usize, look_ahead: usize) -> Vec<Token> {
    let bytes = input.as_bytes();
    let size = bytes.len();

    let mut idx = 0;
    let mut tokens = vec![];

    while idx < size {
        let mut best_offset = 0;
        let mut best_length = 0;

        let search_start = cmp::max(0, idx as i32 - window_size as i32) as usize;
        for i in search_start..idx {
            let mut length = 0;
            while idx + length < size
                && length < look_ahead
                && bytes[i + length] == bytes[idx + length]
            {
                length += 1;
            }

            if length > best_length {
                best_length = length;
                best_offset = idx - i;
            }
        }

        let next_char = if idx + best_length < size {
            bytes[idx + best_length]
        } else {
            0
        };

        tokens.push(Token::new(best_offset, best_length, next_char));
        idx += best_length + 1;
    }

    tokens
}

pub fn decompress(tokens: &Vec<Token>) -> Vec<u8> {
    let mut decompressed = vec![];

    for token in tokens {
        if token.length > 0 {
            let start = decompressed.len() - token.offset;
            for i in 0..token.length {
                decompressed.push(decompressed[start + i]);
            }
        }

        if token.chr > 0 {
            decompressed.push(token.chr);
        }
    }

    decompressed
}
