mod collatz;
mod ipsum;
mod pn;

pub fn test_cases() -> Vec<(Vec<u8>, Vec<Vec<u8>>, Vec<Vec<usize>>)> {

    let mut result = vec![];

    result.push((
        vec![97, 99, 97, 97, 98, 99, 102, 101, 100, 99, 124, 98, 98, 106],
        vec![
            vec![97, 97, 98],
            vec![99, 97],
            vec![97, 98, 98],
            vec![97],
            vec![99],
            vec![105, 124, 99, 98, 106, 108],
            vec![98, 106],
        ],
        vec![
            vec![2],
            vec![1],
            vec![],
            vec![0, 2, 3],
            vec![1, 5, 9],
            vec![],
            vec![12]
        ]
    ));

    result.push((
        vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
        vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0],
            vec![1],
            vec![1, 1]
        ],
        vec![
            vec![0, 8],
            vec![0, 7, 8],
            vec![3, 6, 11, 14],
            vec![]
        ]
    ));

    result.push((
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![
            vec![4, 6],
            vec![4, 5, 6],
            vec![7],
            vec![8]
        ],
        vec![
            vec![],
            vec![3],
            vec![6],
            vec![7]
        ]
    ));

    result.push((
        vec![1, 2, 3, 0, 1, 2, 3, 4, 1, 2, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 4, 5, 1, 6, 2, 4],
        vec![
            vec![1, 2, 3, 4, 1, 2, 4],
            vec![4, 5, 6],
            vec![1, 2, 3],
            vec![2, 4, 5],
            vec![2, 4]
        ],
        vec![
            vec![15],
            vec![],
            vec![0, 4, 11, 15],
            vec![20],
            vec![20, 25]
        ]
    ));

    result.push((
        vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        vec![
            vec![1, 1, 0, 1],
            vec![1, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 1],
        ],
        vec![
            vec![0],
            vec![0],
            vec![10],
            vec![7, 12],
        ]
    ));

    result.push((
        vec![1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1, 1]
        ],
        vec![
            vec![0, 1, 2, 3, 5, 6, 7, 9, 10],
            vec![0, 1, 2, 5, 6, 9],
            vec![0, 1, 5],
            vec![3, 7],
            vec![2, 6]
        ]
    ));

    result.push((
        vec![2, 3, 4, 2, 1, 4, 4, 6, 5, 3, 5, 6, 5, 3, 6, 5, 3, 5, 4, 2, 2, 5, 2, 3, 2, 4, 1, 4, 3, 4, 5, 5, 3, 1, 6, 1, 5, 2, 4, 6, 4, 6, 2, 2, 5, 3, 2, 6, 5, 3, 4, 4, 3, 4, 2, 3, 4, 4, 3, 5, 3, 1, 5, 1, 6, 4, 3, 3, 4, 4, 1, 2, 5, 4, 3, 2, 4, 2, 1, 5, 5, 4, 2, 5, 1, 1, 1, 6, 2, 6, 4, 2, 5, 2, 3, 3, 5, 6, 1, 5, 1, 4, 2, 5, 4, 6, 4, 1, 4, 5, 5, 2, 3, 2, 3, 6, 6, 4, 2, 3, 1, 5, 6, 2, 5, 2, 3, 2, 2, 1, 6, 1, 6, 2, 2, 6, 5, 6, 5, 2, 6, 3, 1, 2, 2, 6, 5, 5, 5, 3, 2, 3, 5, 1, 4, 3, 1, 5, 1, 3, 5, 2, 5, 3, 1, 3, 1, 2, 2, 1, 4, 4, 2, 4, 2, 4, 5, 5, 1, 4, 5, 5, 1, 5, 2, 4, 1, 4, 2, 5, 3, 1, 5, 2, 6, 4, 2, 2, 1, 3, 6, 6, 3, 1, 2, 4, 5, 6, 1, 5, 4, 5, 3, 6, 2, 6, 4, 6, 5, 2, 1, 1, 1, 3, 5, 1, 6, 6, 2, 4, 3, 1, 3, 5, 2, 5, 2, 5, 2, 1, 2, 2, 1, 2, 1, 1, 3, 1, 1, 3, 1, 4, 4, 5, 5, 1, 3, 1, 6, 1, 4, 5, 1, 5, 5, 4, 1, 6, 5, 2, 5, 6, 2, 3, 1, 1, 3, 5, 3, 3, 6, 2, 1, 3, 1, 5, 6, 2, 6, 2, 1, 4, 2, 2, 2, 6, 3, 6, 5, 4, 2, 1, 3, 4, 2, 4, 2, 2, 5, 6, 1, 3, 4, 6, 1, 4, 2, 2, 1, 6, 5, 4, 2, 1, 2, 6, 3, 3, 4, 3, 5, 1, 1, 1, 1, 1, 3, 2, 4, 4, 4, 6, 2, 4, 2, 1, 6, 3, 1, 4, 3, 5, 1, 6, 5, 3, 2, 1, 3, 6, 4, 2, 6, 5, 3, 5, 6, 4, 4, 4, 4, 5, 2, 4, 6, 6, 1, 6, 4, 4, 2, 6, 1, 3, 4, 3, 1, 6, 6, 5, 3, 1, 1, 3, 4, 6, 6, 1, 4, 1, 6, 2, 6, 2, 2, 5, 1, 3, 4, 5, 4, 1, 4, 1, 2, 4, 1, 5, 6, 2, 3, 4, 3, 3, 3, 1, 5, 3, 5, 5, 2, 5, 2, 5, 2, 1, 3, 1, 5, 3, 5, 1, 1, 4, 1, 3, 5, 4, 6, 5, 3, 1, 3, 5, 4, 2, 2, 4, 1, 2, 6, 3, 6, 3, 5, 5, 5, 3, 3, 2, 2, 2, 4, 5, 3, 5, 5, 6, 2, 1, 5, 4, 2, 6, 3, 3, 2, 1, 4, 1, 2, 4, 2, 2, 4, 1, 5, 5, 1, 4, 4, 1, 2, 1, 6, 2, 5, 1, 5, 1, 3, 6],
        vec![
            vec![6, 6, 5, 6],
            vec![1, 2, 1],
            vec![1, 1, 1],
            vec![2, 3, 4],
        ],
        vec![
            vec![],
            vec![242, 501],
            vec![84, 220, 331, 332, 333],
            vec![0, 54, 419]
        ]
    ));

    result.push((
        vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1, 0, 5, 8, 2, 0, 9, 7, 4, 9, 4, 4, 5, 9, 2, 3, 0, 7, 8, 1, 6, 4, 0, 6, 2, 8, 6, 2, 0, 8, 9, 9, 8, 6, 2, 8, 0, 3, 4, 8, 2, 5, 3, 4, 2, 1, 1, 7, 0, 6, 7, 9, 8, 2, 1, 4, 8, 0, 8, 6, 5, 1, 3, 2, 8, 2],
        vec![
            vec![1, 2, 3],
            vec![1, 1],
            vec![2, 4],
            vec![3, 3],
            vec![9, 8],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![8, 9, 7, 9, 3, 2, 3, 8],
            vec![8, 6, 5, 1, 3, 2, 8, 2]
        ],
        vec![
            vec![],
            vec![94],
            vec![],
            vec![24],
            vec![80, 100],
            vec![0],
            vec![11],
            vec![107]
        ]
    ));

    let size = 360;

    result.push((
        vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; size].concat(),
        vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![7, 8, 9, 1, 2, 3]
        ],
        vec![
            vec![],
            (0..size - 1).map(|n| 6 + n * 9).collect()
        ]
    ));

    result
}
