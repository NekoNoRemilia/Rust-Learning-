fn solution(num: i32) -> i32 {
    let mut total: i32;
    let mut i: i32;
    i = 1;
    total = 0;
    while num > i {
        if i%3 == 0 || i%5 == 0{
            total += i;
        } 
        i+=1;
    }
    return total;
}
