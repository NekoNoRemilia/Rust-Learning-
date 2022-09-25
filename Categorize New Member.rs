fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut member = Vec::new();
    for (a, h) in data {
        if a >= 55 && h > 7{
            member.push(String::from("Senior"));
        }
        else{
            member.push(String::from("Open"));
        }
    }
    member
}
