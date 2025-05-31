/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
  let mut i = 1;
  while n != 1{
    i += 1;
    n = if n % 2 == 0{
        n / 2
    } else{
        n * 3 + 1
    };
  }
  i
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}