use statrs::distribution::{ContinuousCDF, Normal};
fn main() {
    let mut result = Normal::new(0.0, 1.0);
    println!("{:?}", result.unwrap().cdf(2.));
}
