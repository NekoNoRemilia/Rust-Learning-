fn is_square(n: i64) -> bool {
    let square = (n as f64).sqrt() as i64;
    if n == square*square{
        return true;
    } else {
        return false;
    }
}
