pub fn hi() -> String {
    "Hi.".to_string()
}

pub fn yo() -> String {
    "Yo.".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hi() {
        assert_eq!(hi(), "Hi.".to_string());
        assert_eq!(hi(), "Hi.");
    }

    #[test]
    fn test_yo() {
        assert_eq!(yo(), "Yo.".to_string());
        assert_eq!(yo(), "Yo.");
    }
}
