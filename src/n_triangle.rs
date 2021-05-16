//! # 高さがNの時の三角形には何個三角形が含まれるのか？
//! ## この問題を解くために必要な知識
//! ### 等差数列
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
//! ### 階差数列
//! * 階差数列: 数列 {an} の隣り合う2つの項の差（bn = an+1–an）を項とする数列 {bn} を，数列 {an} の 階差数列 と呼ぶ
//!   - 例1: 1, 3, 6, 10, 15, 21
//!   - 例2: 1, 6, 15, 28, 45, 66
//! * 階差数列の一般項の公式
//!   - 数列 {an} の階差数列を {bn} とすると
//!     n≧2 のとき: an = a1 + ∑(k=1, n-1, bk)
//! * 階差数列の和
//!   - 項数 n のとき: Sn = ∑(k=1, n, an)
//! * 階差数列の求め方
//!   - まず、等差数列の一般項の公式を用いて {bn} を解く
//!   - 階差数列の一般項の b に代入
//! * 階差数列の和の求め方
//!   - 階差数列の和の公式に 階差数列の一般項 を代入


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
    sum = sum_arith_progression(1, 1, height);
    dbg!(sum);

    if 2 <= height {
        let fixd = height + 1 - 2;

        // small invertted triangles
        sum += sum_arith_progression(1, 1, fixd);
        dbg!(sum);

        // Triangles with height greater than or equal to 2
        sum += triangles_with_height_greater_than_or_equal_to_2(fixd);
        dbg!(sum);
    }

    // Invertted triangles with height greater than or equal to 2
    for i in (4..=height).filter(|x| x % 2 == 0) {
        let fixd = height + 1 - i;
        sum += sum_arith_progression(1, 1, fixd);
        dbg!(sum);
    }
    sum
}


/// 等差数列の和の公式 (sum_arithmetic_progression)
///   初項から第 n 項までの等差数列の和を Sn とする。
///   初項 a，公差 d，項数 n のとき
///   Sn = n{2a+(n−1)d} / 2
/// 初項 a: first
/// 公差 d: diff
/// 項数 n: length
fn sum_arith_progression(first: u64, diff: u64, length: u64) -> u64 {
    length * (2 * first + (length - 1) * diff) / 2
}


/// 高さが2以上の三角形
///
/// 高さが2の時、高さが2の三角形が 1
/// 高さが3の時、高さが2の三角形が 3, 高さが3の三角形が 1
/// 高さが4の時、高さが2の三角形が 6, 高さが3の三角形が 3, 高さが4の三角形が1
/// 高さが5の時、高さが2の三角形が10, 高さが3の三角形が 6, 高さが4の三角形が3, 高さが5の三角形が1
/// 高さが6の時、高さが2の三角形が15, 高さが3の三角形が10, 高さが4の三角形が6, 高さが5の三角形が3, 高さが6の三角形が1
///
/// このように各三角形の合計は、[1, 3, 6, 10, 15](以降{an}) となる。
/// {an} の階差数列 {bn} は、[2, 3, 4, 5]。
///
/// 等差数列の一般項の公式を用いて {bn} を解く
///   初項 2, 公差 1 であるから、
///     bn = a + (n−1) * d
///     bn = 2 + (n-1) * 1
///        = 2 + (n-1)
///        = 2 + n - 1
///        = n + 1
///
///   階差数列の一般項の b に代入
///     an = a1 + ∑(k=1, n-1, bk)
///        = a1 + ∑(k=1, n-1, k+1)
///        = 1 + ∑(k=1, n-1, k+1)
///        = 1 + ∑(k=1, n-1, k) + ∑(k=1, n-1, 1)
///        = 1 + ((n-1)n)/2 + n-1
///        = ((n-1)n)/2 + n
///        = ((n-1)n)/2 + 2n/2
///        = ((n-1)n+2n)/2
///        = (n^2-n+2n)/2
///        = (n^2-n+2n)/2
///        = (n^2+n)/2
///
///   階差数列の和の公式に 階差数列の一般項 を代入
///     Sn = ∑(k=1, n, an)
///        = ∑(k=1, n, (k^2+k)/2)
///        = ∑(k=1, n, (k^2+k)) / 2
///        = (∑(k=1, n, (k^2+k))) / 2
///        = (∑(k=1, n, k^2) + ∑(k=1, n, k)) / 2
///        = ((n(n+1)(2n+1))/6 + n(n+1)/2) / 2
///        = (((n^2+n)(2n+1))/6 + (n^2+n)/2) / 2
///        = (((n^2+n)(2n+1))/6 + (3n^2+3n)/6) / 2
///        = (((n^2+n)(2n+1) + 3n^2 + 3n) / 6) / 2
///        = ((n^2+n)(2n+1) + 3n^2 + 3n) / 12
///        = (2n^3 + 3n^2 + n + 3n^2 + 3n) / 12
///        = (2n^3 + 6n^2 + 4n) / 12
///        = (n^3 + 3n^2 + 2n) / 6
///
/// 以上の計算結果から、 (n^3 + 3n^2 + 2n) / 6 で高さが2以上の三角形が求められる。
fn triangles_with_height_greater_than_or_equal_to_2(n: u64) -> u64 {
    (n.pow(3) + 3 * n.pow(2) + 2 * n) / 6
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