fn main() {
    println!("Hello, world!");

    let i = 10;
    let j = 20;
    let val = add(i, j);
    println!("{i} + {j} = {val}");

    let p = 100;
    let q = 40;
    let res = subt(p, q);
    println!("{p} - {q} = {res}");


    
}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

fn subt(i: i32, j: i32) -> i32 {
    return i - j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let i = 1;
        let j = 2;
        assert_eq!(i + j, add(i, j));
    	assert_eq!(add(j, i), add(i, j));
    }

    #[test]
    fn subt_test() {
    	let p = 10;
        let q = 7;
        assert_eq!(p - q, subt(p, q));
    	assert_ne!(subt(p, q), subt(q, p));
    }
    
    #[test]
    fn subt_test_neg() {
    	let p = 5;
	let q = 10;
        assert_eq!(p - q, subt(p, q));
        assert_ne!(subt(p, q), subt(q, p));
    }
}
