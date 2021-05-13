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