fn main() {
    let longest;
    let str1 = String::from("Aysuhmaan");
    {
        let str2 = String::from("Aysuhmaan Singh");
        longest = longer(&str1, &str2).to_string();
        println!("{longest}");
    }
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
