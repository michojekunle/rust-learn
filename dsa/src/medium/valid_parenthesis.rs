impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut balance = 0;
        let mut indToBeCut: Vec<usize> = Vec::new();
    
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
               balance += 1; 
            } else if c == ')' {
                balance -= 1;
            }  

            if (c == '(' || c == ')') && balance < 0 {
                balance = 0;
                indToBeCut.push(i);
            }
        }

        for (i, c) in s.char_indices().rev() {
            if balance == 0 {
                break;
            }

            if c == '(' {
                indToBeCut.push(i);
                balance -= 1;
            }
        }

        fn remove_at_indexes(s: &str, indexes: &[usize]) -> String {
            s.chars()
                .enumerate()
                .filter(|(i, _)| !indexes.contains(i))
                .map(|(_, c)| c)
                .collect()
        }

        remove_at_indexes(&s, &indToBeCut)
    }
}