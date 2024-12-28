pub fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {
        num * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_vec_map() {
        let v = vec![1, 2, 3];
        assert_eq!(vec_map(&v), vec![2, 4, 6]);
    }
    
    #[test]
    fn negative_vec_map() {
        let v = vec![1, 2, 3];
        assert_ne!(vec_map(&v), vec![-2, -4, -6]);
    }
}