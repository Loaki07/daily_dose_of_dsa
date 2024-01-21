pub fn length_of_last_word(s: String) -> i32 {
    s.split(" ")
        .filter(|el| *el != " ")
        .filter(|el| !el.is_empty())
        .into_iter()
        .last()
        .map(|w| w.len() as i32)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex2() {
        let res = length_of_last_word(
            "   fly me   to   the moon  ".to_owned(),
        );
        let expected = 4_i32;
        assert_eq!(res, expected);
    }
}
