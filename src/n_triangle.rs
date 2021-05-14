use proconio::input;

pub fn exec() {
    input! {
        n : u32,
    }

    println!("{} is {}.", n, calc(n));
}


fn calc(n: u32) -> u64 {
    if n <= 0 {return 0};

    let n: u64 = n as u64;
    let mut sum: u64;

    // small triangles
    sum = n * (n + 1) / 2;
    dbg!(sum);

    // small invertted triangles
    sum += (n - 1) * n / 2;
    dbg!(sum);

    // Triangles with height greater than or equal to 2
    if 2 <= n {
        sum += ((n - 1) * n / 2) * ((n - 1) + 2) / 3;
        dbg!(sum);
    }

    for i in 4..=n {
        // Invertted triangles with height greater than or equal to 2
        for j in (4..=i).filter(|x| x % 2 == 0) {
            sum += i - j + 1;
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