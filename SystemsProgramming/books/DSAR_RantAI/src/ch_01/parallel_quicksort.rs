pub fn parallel_quicksort(arr: Vec<usize>) -> Vec<usize> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(
            parallel_quicksort(vec![8, 3, 7, 1, 9, 5, 2, 6, 4]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(
            parallel_quicksort(vec![4, 2, 4, 1, 3, 2]),
            vec![1, 2, 2, 3, 4, 4]
        );
    }
}
