use std::iter;

const DOT: i32 = -1;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mut fs = input
        .as_ref()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold(Vec::new(), |mut v, (i, c)| {
            v.extend(iter::repeat(if i % 2 == 0 { i as i32 / 2 } else { DOT }).take(c as usize));
            v
        });

    let mut l = 0;
    let mut r = fs.len() - 1;
    while l < r {
        while l < r && fs[l] != DOT {
            l += 1;
        }

        while l < r && fs[r] == DOT {
            r -= 1;
        }

        if l < r {
            fs.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    let checksum = fs.iter().enumerate().fold(0_i64, |acc, (i, &v)| {
        if v < 0 {
            return acc;
        }

        acc + i as i64 * v as i64
    });

    checksum
}

pub fn part_two(_input: impl AsRef<str>) -> i64 {
    // let mut fs = get_test_input(9)
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .enumerate()
    //     .fold(Vec::new(), |mut v, (i, c)| {
    //         v.extend(iter::repeat(if i % 2 == 0 { i as i32 / 2 } else { DOT }).take(c as usize));
    //         v
    //     });

    // let mut l = 0;
    // let mut r = fs.len() - 1;
    // 'outer: loop {
    //     while l < r && fs[r] == DOT {
    //         r -= 1;
    //     }

    //     if l >= r {
    //         break;
    //     }

    //     let num = fs[r];
    //     let mut num_count = 0;
    //     while l < r && fs[r] == num {
    //         r -= 1;
    //         num_count += 1;
    //     }

    //     loop {
    //         while l < r && fs[l] != DOT {
    //             l += 1;
    //         }

    //         if l >= r {
    //             break 'outer;
    //         }

    //         let mut dot_count = 0;
    //         while l < r && fs[l] == DOT {
    //             l += 1;
    //             dot_count += 1;

    //             if dot_count == num_count {
    //                 for i in 0..num_count {
    //                     fs.swap(l - num_count + i, r + i);
    //                 }
    //                 continue 'outer;
    //             }
    //         }
    //     }
    // }

    // println!("{:?}", fs);

    // let checksum = fs.iter().enumerate().fold(0, |acc, (i, &v)| {
    //     if v < 0 {
    //         return acc;
    //     }

    //     acc + i * v as usize
    // });

    // println!("{}", checksum);
    0
}
