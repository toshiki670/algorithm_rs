use proconio::input;

pub fn exec() {
    println!("Enter a height:");
    input! {
        height : u32,
    }
    println!("{} is {}.", height, calc(height));
}


fn calc(height: u32) -> u64 {
    if height <= 0 {return 0};

    let height: u64 = height as u64;
    let mut sum: u64;

    // small triangles
    sum = height * (height + 1) / 2;
    dbg!(sum);

    // small invertted triangles
    sum += (height - 1) * height / 2;
    dbg!(sum);

    // Triangles with height greater than or equal to 2
    if 2 <= height {
        let fixd_nheight = height + 1 - 2;
        sum += (fixd_nheight * (fixd_nheight + 1) / 2) * (fixd_nheight + 2) / 3;
        dbg!(sum);
    }

    // Invertted triangles with height greater than or equal to 2
    for i in (4..=height).filter(|x| x % 2 == 0) {
        let fixd_nheight = height + 1 - i;
        sum += fixd_nheight * (fixd_nheight + 1) / 2;
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