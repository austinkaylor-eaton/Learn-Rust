﻿#[test]
fn use_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 2).collect();

    assert_eq!(v2, vec![3, 4, 5]);
}