pub fn levenshtein_dist_word(w1: &str, w2: &str) -> usize {
    levenshtein_dist_word_array(w1, w2)
}

pub fn levenshtein_dist_word_recursive(w1: &str, w2: &str) -> usize {
    // TODO this isn't the correct metric ?
    // maybe we should think about if we want the link to be one way?
    // if the query is the main thing be are trying to match and we should ignore the entry.
    if w1.len() == 0 {
        return w2.len();
    }

    if w2.len() == 0 {
        return w1.len();
    }

    let first_char_w1 = w1.chars().next().unwrap();
    let first_char_w2 = w2.chars().next().unwrap();
    if first_char_w1 == first_char_w2 {
        return levenshtein_dist_word_recursive(
            w1.split_once(first_char_w1).unwrap().1,
            w2.split_once(first_char_w2).unwrap().1,
        );
    }
    1 + *[
        levenshtein_dist_word_recursive(w1.split_once(first_char_w1).unwrap().1, w2),
        levenshtein_dist_word_recursive(w1, w2.split_once(first_char_w2).unwrap().1),
        levenshtein_dist_word_recursive(
            w1.split_once(first_char_w1).unwrap().1,
            w2.split_once(first_char_w2).unwrap().1,
        ),
    ]
    .iter()
    .min()
    .unwrap()
}

pub fn levenshtein_dist_word_array(w1: &str, w2: &str) -> usize {
    let w1_chars: Vec<char> = w1.chars().collect();
    let w2_chars: Vec<char> = w2.chars().collect();

    let width = w1_chars.len();
    let height = w2_chars.len();

    let mut array = vec![vec![0; width + 1]; height + 1];

    for i in 1..width + 1 {
        array[0][i] = i;
    }
    for i in 1..height + 1 {
        array[i][0] = i;
    }

    for i in 1..height + 1 {
        for j in 1..width + 1 {
            let mut sub_cost = 0;
            if w1_chars[j - 1] == w2_chars[i - 1] {
            } else {
                sub_cost = 1;
            }
            array[i][j] += *[array[i - 1][j - 1] + sub_cost,
                             array[i][j - 1] + 1,
                             array[i - 1][j] + 1]
                .iter()
                .min()
                .unwrap();
        }
    }
    return array[height][width];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_recursive() {
        assert!(levenshtein_dist_word_recursive("", "") == 0);
        assert!(levenshtein_dist_word_recursive("ab", "ab") == 0, "{}", levenshtein_dist_word_recursive("ab", "ab"));
        assert!(levenshtein_dist_word_recursive("cat", "cats") == 1,);
        assert!(levenshtein_dist_word_recursive("123", "abc") == 3);
        assert!(levenshtein_dist_word_recursive("qwyg", "dev!") == 4);
    }

    #[test]
    fn test_levenshtein_array() {
        assert!(levenshtein_dist_word_array("", "") == 0);
        assert!(levenshtein_dist_word_array("ab", "ab") == 0, "{}", levenshtein_dist_word_array("ab", "ab"));
        assert!(levenshtein_dist_word_array("cat", "cats") == 1);
        assert!(levenshtein_dist_word_array("123", "abc") == 3);
        assert!(levenshtein_dist_word_array("qwyg", "dev!") == 4);
        //assert!(levenshtein_dist_word_array("qwrpz", "I") == 5, "{}=/= {}", levenshtein_dist_word_array("qwrpz", "I"), 5);
        assert!(levenshtein_dist_word_array("I", "qwrpz") == 5, "{}=/= {}", levenshtein_dist_word_array("qwrpz", "I"), 5);
    }
}
