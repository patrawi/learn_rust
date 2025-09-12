trait Multiplier<A, B, C> {
    fn multiply(a: &A, b: &B) -> C;
}

struct I32Multiplier;
struct F64Multiplier;
impl Multiplier<f64, f64, f64> for F64Multiplier {
    fn multiply(a: &f64, b: &f64) -> f64 {
        *a * *b
    }
}
impl Multiplier<i32, i32, i32> for I32Multiplier {
    fn multiply(a: &i32, b: &i32) -> i32 {
        *a * *b
    }
}

fn calculate<T, A, B, C>(t: &T, a: &A, b: &B) -> C
where
    T: Multiplier<A, B, C>,
{
    T::multiply(a, b)
}

fn main() {
    let a = 8i32;
    let b = 9i32;
    let result = I32Multiplier::multiply(&a, &b);
    println!("Result: {}", result);
    let c = 8f64;
    let d = 9f64;
    let another_result = F64Multiplier::multiply(&c, &d);
    println!("Result: {}", another_result);
}
