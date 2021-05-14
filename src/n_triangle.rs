use proconio::input;

pub fn exec() {
    input! {
        n : u32,
    }

    println!("{} is {}.", n, calc(n));
}


fn calc(n: u32) -> u64 {
    let mut sum: u64 = 0;

    if n <= 0 {return 0};

    // small triangles
    sum += (n * (n + 1) / 2) as u64;
    dbg!(sum);

    // small invertted triangles
    sum += ((n - 1) * n / 2) as u64;
    dbg!(sum);

    for i in 2..=n {
        // Triangles with height greater than or equal to 2
        for j in 1..=(i - 1) {
            sum += j as u64;
        }

        // Invertted triangles with height greater than or equal to 2
        if 4 <= i {
            for j in (4..=i).filter(|x| x % 2 == 0) {
                sum += (i - j) as u64 + 1;
            }
        }

        dbg!(sum);
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn be_valid() {
        assert_eq!(super::calc(0), 0);
        assert_eq!(super::calc(1), 1);
        assert_eq!(super::calc(2), 5);
        assert_eq!(super::calc(3), 13);
        assert_eq!(super::calc(4), 27);
        assert_eq!(super::calc(5), 48);
        assert_eq!(super::calc(6), 78);
        assert_eq!(super::calc(7), 118);
        assert_eq!(super::calc(8), 170);
        assert_eq!(super::calc(9), 235);
        assert_eq!(super::calc(10), 315);
    }
}