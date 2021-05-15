//! # 高さがNの時の三角形には何個三角形が含まれるのか？
//! ## この問題を解くために必要な知識
//! * 等差数列: 隣り合う2項の差が常に一定の数列のこと。
//!   - 例1: 1, 2, 3, 4, 5, 6
//!   - 例2: 1, 3, 5, 7, 9, 11
//!   - このような数列の差を「公差」と呼ぶ
//! * 等差数列の一般項の公式
//!   - 数列an の第n項an が n の式で表されるとき，これを数列an の一般項と呼ぶ
//!     + 初項 a，公差 d の等差数列 an の一般項: an=a+(n−1)d
//! * 等差数列の和の公式
//!   - 初項から第 n 項までの等差数列の和を Sn とする。
//!     + 初項 a，末項 l，項数 n のとき: Sn=n(a+l)/2
//!     * 初項 a，公差 d，項数 n のとき: Sn=n{2a+(n−1)d}/2


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
    sum = sum_arith_progression(1, height, height);
    dbg!(sum);

    // small invertted triangles
    sum += sum_arith_progression(1, height - 1, height - 1);
    dbg!(sum);

    // Triangles with height greater than or equal to 2
    if 2 <= height {
        let fixd = height + 1 - 2;
        sum += sum_arith_progression(1, fixd, fixd) * (fixd + 2) / 3;
        dbg!(sum);
    }

    // Invertted triangles with height greater than or equal to 2
    for i in (4..=height).filter(|x| x % 2 == 0) {
        let fixd = height + 1 - i;
        sum += sum_arith_progression(1, fixd, fixd);
        dbg!(sum);
    }
    sum
}


/// 等差数列の和の公式 (sum_arithmetic_progression)
///   初項から第 n 項までの等差数列の和を Sn とする。
///   初項 a，末項 l，項数 n のとき
///   Sn = n(a + l) / 2
/// 初項 a: first
/// 末項 l: last
/// 項数 n: length
fn sum_arith_progression(first: u64, last: u64, length: u64) -> u64 {
    length * (first + last) / 2
}

// 等差数列 を求める関数
// 初項 a，公差 d の等差数列 an の一般項は
// an = a + (n − 1)d
// fn arith_progression(first: u64, diff: u64, n: u64) -> u64 {
//     first + (n - 1) * diff
// }

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