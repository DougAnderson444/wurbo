use nanoid::nanoid;

/// CSS Selector IDs need to start with a letter, so we can't use the default nanoid alphabet.
/// We'll use this custom one instead.
const ALPHABET: [char; 28] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '_', '-',
];

/// Generate a random ID for a CSS selector
pub fn rand_id() -> String {
    let id = nanoid!(16, &ALPHABET);
    id
}

/// Fill a 32 byte buffer with random data
pub fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}
