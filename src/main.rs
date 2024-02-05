fn main() {
    println!("Hello, world!");

    let i = 10;
    let j = 20;
    let val = add(i, j);
    println!("{i} + {j} = {val}");
}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let i = 1;
        let j = 2;
        assert_eq!(i + j, add(i, j));
    }
}