# vApp Submission: Word Puzzle game

## Verification
```yaml
github_username: "hallosayael"
discord_id: "715913148707635250"
timestamp: "2025-08-27"
```

## Developer
- **Name**: EL
- **GitHub**: @hallosayael
- **Discord**: EL#6666
- **Experience**: a private employee who likes the world of blockchain

## Project

### Name & Category
- **Project**: Word Puzzle game
- **Category**: gaming

### Description
I just gave an idea for additional games on the #game-arena channel besides the 8Queens game

### SL Integration  
Sorry in advance, but I am only self-taught and I don't really understand English, here I will explain the game I made, I hope you guys can understand it.

### Validation
Word validation logic in Word Puzzle game

### Short explanation:
- We'll accept input as a word and the available letters.
- We'll check if the word is valid (in the hash dictionary).
- We'll check if all the letters in the word are in letters.
- If valid, the main function will succeed; otherwise, the program asserts an error.

### Step 1: Create a new noir project folder
```
noir new puzzle_kata
cd puzzle_kata
```

### Step 2: Replace the contents of src/main.nr with the following code:
```
// Maximum word and letter size available
const WORD_LEN: usize = 7;
const LETTERS_LEN: usize = 7;
const DICTIONARY_SIZE: usize = 3;

// Simple hash function (e.g. adapt to Noir supported hashes)
fn simple_hash(arr: [u8; WORD_LEN]) -> u32 {
    let mut hash = 0u32;
    for i in 0..WORD_LEN {
        hash = hash + arr[i] as u32 * (i as u32 + 1);
    }
    hash
}

// Dictionary (dictionary) in the form of a word hash
const DICTIONARY: [u32; DICTIONARY_SIZE] = [
    751,  // hash("kata")
    664,  // hash("kita")
    535,  // hash("atik")
];

// Letter frequency counting function
fn count_letters(arr: [u8; LETTERS_LEN]) -> [u8; 26] {
    let mut counts = [0u8; 26];
    for i in 0..LETTERS_LEN {
        let idx = arr[i] - 97; // 'a' ASCII = 97
        counts[idx as usize] = counts[idx as usize] + 1;
    }
    counts
}

// The main function of word validation
fn validate_word(word: [u8; WORD_LEN], letters: [u8; LETTERS_LEN]) -> bool {
    // 1. Cek kata ada di kamus
    let h = simple_hash(word);
    let mut found = false;
    for i in 0..DICTIONARY_SIZE {
        if DICTIONARY[i] == h {
            found = true;
            break;
        }
    }
    assert(found, "Kata tidak ada di kamus");

    // 2. Check the letters are in letters
    let mut letter_counts = count_letters(letters);
    for i in 0..WORD_LEN {
        let c = word[i];
        if c == 0u8 { // ignore char kosong (misal kata lebih pendek dari 7)
            break;
        }
        let idx = (c - 97) as usize;
        assert(letter_counts[idx] > 0, "Huruf tidak tersedia");
        letter_counts[idx] = letter_counts[idx] - 1;
    }
    true
}

// Entry point function
fn main(word: [u8; WORD_LEN], letters: [u8; LETTERS_LEN]) -> bool {
    validate_word(word, letters)
}
```

### Explanation:
- The input words and letters are 7-letter byte arrays (lowercase ASCII).
- The dictionary is kept simple and uses a hash function.
- Validation fails if the word is not in the dictionary or if there are not enough letters.

### Step 3: Test the code
```
noir test
```

If there is an error in the assert, it means validation failed.

### Step 4: Next steps
- You can modify the dictionary and hash function.
- Integrate with the Soundness platform for proofing.
- Create a word input front-end.
