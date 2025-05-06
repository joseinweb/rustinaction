use rand;

pub fn gen_lucky_num() -> u64 {
    rand::random::<u64>() % 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
