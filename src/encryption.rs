use log;

const SECRET: [char; 6] = ['S', 'E', 'C', 'R', 'E', 'T'];
const ENCRYPTION_CHARS_AMOUNT: usize = SECRET.len();

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
    let replaced_message = message.trim().to_ascii_uppercase().chars().into_iter().fold(String::new(), |accum, c| {
        let index = ENCRYPTION_TABLE.iter().position(|&x| { &x == &c }).unwrap();
        let row = index / ENCRYPTION_CHARS_AMOUNT;
        let column = index % ENCRYPTION_CHARS_AMOUNT;

        accum + &ENCRYPTION_CHARS[row].to_string() + &ENCRYPTION_CHARS[column].to_string()
    });
    log::info!("{}", &replaced_message);

    // Этап 2. Перестановка
    let mut vectors_to_permutation: [Vec<char>; ENCRYPTION_CHARS_AMOUNT] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for (i, c) in SECRET.into_iter().enumerate() {
        vectors_to_permutation[i % ENCRYPTION_CHARS_AMOUNT].push(c);
    }
    for (i, c) in replaced_message.chars().into_iter().enumerate() {
        vectors_to_permutation[i % ENCRYPTION_CHARS_AMOUNT].push(c);
    }
    vectors_to_permutation.sort_by(|a, b| a[0].cmp(&b[0]));
    for i in 0..ENCRYPTION_CHARS_AMOUNT {
        vectors_to_permutation[i].remove(0);
    }

    let rows = replaced_message.len() / ENCRYPTION_CHARS_AMOUNT + 1;
    let mut hash = String::new();
    for i in 0..rows {
        for j in 0..vectors_to_permutation.len() {
            let letters = vectors_to_permutation.get(j);
            if let Some(letters) = letters {
                let letter = letters.get(i);
                match letter {
                    Some(letter) => hash.push(*letter),
                    None => hash.push(' ')
                }
            }
        }
    }
    log::info!("{}", &hash);

    hash.trim().to_string()
}

pub fn decrypt(hash: &str) -> String {
    let mut vectors_to_permutation = vec![];
    let sorted_secret = {
        let mut secret = SECRET.to_vec();
        secret.sort_by(|a, b| a.cmp(b));
        secret
    };
    for (i, c) in sorted_secret.into_iter().enumerate() {
        vectors_to_permutation.push(vec![c]);
    }
    for (i, c) in hash.trim().chars().into_iter().enumerate() {
        vectors_to_permutation[i % ENCRYPTION_CHARS_AMOUNT].push(c);
    }

    let mut permutated_vectors: Vec<Vec<char>> = vec![];
    for c in SECRET.into_iter() {
        let index = vectors_to_permutation.iter().position(|x| { x[0] == c }).unwrap();
        permutated_vectors.push(vectors_to_permutation.remove(index).iter()
            .filter_map(|x| match *x != ' ' {
                true => Some(*x),
                false => None
            }).collect()
        );
    }
    for v in permutated_vectors.iter_mut() {
        v.remove(0);
    }

    log::info!("{:#?}", &permutated_vectors);

    let mut message = String::new();
    let rows = hash.len() / ENCRYPTION_CHARS_AMOUNT + 1;
    for i in 0..rows {
        for j in 0..permutated_vectors.len() {
            let letter = permutated_vectors.get(j);
            if let Some(letter) = letter {
                let letter = letter.get(i);
                match letter {
                    Some(letter) => message.push(*letter),
                    None => {}
                }
            }
        }
    }
    message = message.to_ascii_uppercase();
    log::info!("{}", &message);

    let mut decrypted_message = String::new();
    for i in 0..message.len() / 2 {
        let row = match ENCRYPTION_CHARS.iter().position(|&x| { &x == &message.chars().nth(i * 2).unwrap() }) {
            Some(row) => row,
            None => return String::from("Не удалось расшифровать сообщение 1")
        };
        let column = match ENCRYPTION_CHARS.iter().position(|&x| { &x == &message.chars().nth(i * 2 + 1).unwrap() }) {
            Some(column) => column,
            None => return String::from("Не удалось расшифровать сообщение 2")
        };
        let index = row * ENCRYPTION_CHARS_AMOUNT + column;
        match ENCRYPTION_TABLE.get(index) {
            Some(letter) => decrypted_message.push(*letter),
            None => return String::from("Не удалось расшифровать сообщение 3")
        }
    }

    decrypted_message.to_ascii_lowercase()
}
