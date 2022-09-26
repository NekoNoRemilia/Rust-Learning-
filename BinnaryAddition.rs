fn add_binary(a: u64, b: u64) -> String {
    let sum = a + b;
    let s = format!("{:b}", sum);
    
    return s;
}
