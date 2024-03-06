#[cfg(test)]
mod tests {
    use crate::lr_6::substring;

    // test_boyer_moore_search

    #[test]
    fn test_boyer_moore_search_each_letter_matches() {
        let index = substring::boyer_moore_search("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn test_boyer_moore_search_a_few_separate_matches() {
        let index = substring::boyer_moore_search("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn test_boyer_moore_search_one_match() {
        let index = substring::boyer_moore_search("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn test_boyer_moore_search_lots_of_matches() {
        let index = substring::boyer_moore_search("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn test_boyer_moore_search_lots_of_intricate_matches() {
        let index = substring::boyer_moore_search("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_boyer_moore_search_not_found0() {
        let index = substring::boyer_moore_search("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_boyer_moore_search_not_found1() {
        let index = substring::boyer_moore_search("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_boyer_moore_search_not_found2() {
        let index = substring::boyer_moore_search("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_boyer_moore_search_empty_string() {
        let index = substring::boyer_moore_search("", "abcdef");
        assert_eq!(index, vec![]);
    }

    // test_rabin_karp_search

    #[test]
    fn test_rabin_karp_search_each_letter_matches() {
        let index = substring::rabin_karp_search("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn test_rabin_karp_search_a_few_separate_matches() {
        let index = substring::rabin_karp_search("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn test_rabin_karp_search_one_match() {
        let index = substring::rabin_karp_search("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn test_rabin_karp_search_lots_of_matches() {
        let index = substring::rabin_karp_search("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn test_rabin_karp_search_lots_of_intricate_matches() {
        let index = substring::rabin_karp_search("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_rabin_karp_search_not_found0() {
        let index = substring::rabin_karp_search("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_rabin_karp_search_not_found1() {
        let index = substring::rabin_karp_search("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_rabin_karp_search_not_found2() {
        let index = substring::rabin_karp_search("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_rabin_karp_search_empty_string() {
        let index = substring::rabin_karp_search("", "abcdef");
        assert_eq!(index, vec![]);
    }

    // test_knuth_morris_pratt_search

    #[test]
    fn test_knuth_morris_pratt_search_each_letter_matches() {
        let index = substring::knuth_morris_pratt_search("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_a_few_separate_matches() {
        let index = substring::knuth_morris_pratt_search("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_one_match() {
        let index = substring::knuth_morris_pratt_search("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_lots_of_matches() {
        let index = substring::knuth_morris_pratt_search("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_lots_of_intricate_matches() {
        let index = substring::knuth_morris_pratt_search("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_not_found0() {
        let index = substring::knuth_morris_pratt_search("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_not_found1() {
        let index = substring::knuth_morris_pratt_search("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_not_found2() {
        let index = substring::knuth_morris_pratt_search("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn test_knuth_morris_pratt_search_empty_string() {
        let index = substring::knuth_morris_pratt_search("", "abcdef");
        assert_eq!(index, vec![]);
    }
}
