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

// TODO think about this
pub fn levenshtein_dist_word_simd(w1: &str, w2: &str) -> usize {
    if !w1.is_ascii() || !w2.is_ascii() {
        return levenshtein_dist_word_array(w1, w2);
    }

    let w1_ascii = w1.as_bytes();
    let w2_ascii = w2.as_bytes();

    let width = w1_ascii.len();
    let height = w2_ascii.len();

    let mut array = vec![vec![0u8; width + 1]; height + 1];
    for i in 1..width + 1 {
        array[0][i] = i as u8;
    }
    for i in 1..height + 1 {
        array[i][0] = i as u8;
    }

    use core::arch::x86_64::*;

    let mut temp = [0u8; 16];
    unsafe {
        assert!(w2_ascii.len() < 16, "Function doesn't handle words longer than 16 chars.");
        temp.copy_from_slice(&w1_ascii);
        let _a =_mm_lddqu_si128(temp.as_ptr() as *const __m128i); 

        temp = [0u8; 16];
        temp.copy_from_slice(&vec![u8::MAX; width]);
        let _mask =_mm_lddqu_si128(temp.as_ptr() as *const __m128i); 
        for it in w2_ascii.iter() {


            let mut _b = _mm_set1_epi8(*it as i8);
            _b = _mm_and_si128(_b, _mask);

            let mut _r = _mm_cmpeq_epi8(_a, _b);

            _r = _mm_andnot_si128(_r, _mm_set1_epi8(1));

            let result: &mut [u8; 16] = std::mem::transmute(&mut _r);
            for jt in result {
            }


        }
    }
    
    todo!()
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
