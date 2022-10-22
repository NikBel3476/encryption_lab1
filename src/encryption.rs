use log;

const ENCRYPTION_CHARS_AMOUNT: usize = 6;
const SECRET: [char; 6] = ['s', 'e', 'c', 'r', 'e', 't'];

const ENCRYPTION_CHARS: [char; ENCRYPTION_CHARS_AMOUNT] = ['A', 'B', 'C', 'D', 'E', 'F'];

const ENCRYPTION_TABLE: [char; ENCRYPTION_CHARS_AMOUNT * ENCRYPTION_CHARS_AMOUNT] = [
    'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9'
];

pub fn encrypt(message: &str) -> String {
    // Этап 1. Замена
    let replaced_message = message.to_ascii_uppercase().chars().into_iter().fold(String::new(), |accum, c| {
        let index = ENCRYPTION_TABLE.iter().position(|&x| { &x == &c }).unwrap();
        let row = index / ENCRYPTION_CHARS_AMOUNT;
        let column = index % ENCRYPTION_CHARS_AMOUNT;

        accum + &ENCRYPTION_CHARS[row].to_string() + &ENCRYPTION_CHARS[column].to_string()
    });

    // Этап 2. Перестановка
    let mut vec_to_permutation: [Vec<char>; ENCRYPTION_CHARS_AMOUNT] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for (i, c) in SECRET.into_iter().enumerate() {
        vec_to_permutation[i % ENCRYPTION_CHARS_AMOUNT].push(c);
    }
    for (i, c) in replaced_message.chars().into_iter().enumerate() {
        vec_to_permutation[i % ENCRYPTION_CHARS_AMOUNT].push(c);
    }
    vec_to_permutation.sort_by(|a, b| a[0].cmp(&b[0]));
    for i in 0..ENCRYPTION_CHARS_AMOUNT {
        vec_to_permutation[i].remove(0);
    }

    let rows = replaced_message.len() / ENCRYPTION_CHARS_AMOUNT + 1;
    let mut hash = String::new();
    for i in 0..rows {
        for j in 0..vec_to_permutation.len() {
            let letter = vec_to_permutation.get(j);
            if let Some(letter) = letter {
                let letter = letter.get(i);
                if let Some(letter) = letter {
                    hash.push(*letter);
                }
            }
        }
    }

    hash
}
