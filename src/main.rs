use std::io;

fn main() {
    let max = read_max().expect("failed to read");
    if max > 1000 {
        println!("too big number = {}", max);
        return;
    }
    let arr = 1..=max;
    for x in arr {
        let s = fiz_buzz(x);
        println!("{}", s);
    }
}

fn read_line() -> Result<String, String> {
    let mut max = String::new();
    match io::stdin().read_line(&mut max) {
        Ok(_) => Ok(max.trim().to_string()),
        Err(_) => Err("Failed to readline".to_owned()),
    }
}

fn read_max() -> Result<i32, String> {
    let max_str = r#try!(read_line());
    match max_str.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err("Failed parse".to_owned()),
    }
}

fn fiz_buzz(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_owned()
    } else if n % 5 == 0 {
        "Buzz".to_owned()
    } else if n % 3 == 0 {
        "Fizz".to_owned()
    } else {
        n.to_string()
    }
}
