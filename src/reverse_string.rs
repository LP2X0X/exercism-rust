// Reversing strings (reading them from right to left, rather than from left to right)
// is a surprisingly common task in programming.

// Your task is to reverse a given string.

// Some examples:
//     Turn "stressed" into "desserts".
//     Turn "strops" into "sports".
//     Turn "racecar" into "racecar".

// Bonus
// Test your function on this string: uüu and see what happens.

// ---

// SUMMARY
// Use input string length as index to gradually add char in reverse order to an array
// then recreate the reversed string with iter().collect().
#[expect(unused)]
pub fn reverse(input: &str) -> String {
    let input_len = input.chars().count(); // This helps accurately count even the grapheme characters.
    let mut temp_vec = vec!['_'; input_len];

    let mut _input_len = input_len;

    for c in input.chars() {
        _input_len -= 1;
        temp_vec[_input_len] = c;
    }

    let reverse_input = temp_vec.iter().collect();
    return reverse_input;
}
