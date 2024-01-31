use nanoid::nanoid;

/// CSS Selector IDs need to start with a letter, so we can't use the default nanoid alphabet.
/// We'll use this custom one instead.
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Generate a random ID for a CSS selector
pub fn rand_id() -> String {
    let id = nanoid!(16, &ALPHABET);
    id
}
