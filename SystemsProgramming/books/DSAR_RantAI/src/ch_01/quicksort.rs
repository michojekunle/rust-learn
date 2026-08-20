pub fn quick_sort(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() < 2 {
        return arr;
    }

    let pivot = arr[arr.len() - 1_usize];
    let mut left = vec![];
    let mut middle = vec![];
    let mut right = vec![];

    for e in arr {
        if e < pivot {
            left.push(e);
        } else if e == pivot {
            middle.push(e)
        } else if e > pivot {
            right.push(e);
        }
    }

    quick_sort(left)
        .into_iter()
        .chain(middle)
        .chain(quick_sort(right))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(
            quick_sort(vec![8, 3, 7, 1, 9, 5, 2, 6, 4]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(quick_sort(vec![4, 2, 4, 1, 3, 2]), vec![1, 2, 2, 3, 4, 4]);
    }
}
