#[cfg(test)]
mod tests {

    use categories_macro::Category;
    use std::collections::HashSet;
    #[derive(Category, Hash, Eq, PartialEq, Debug)]
    enum Direction {
        Est,
        West,
        North,
        South,
    }

    #[test]
    fn simple_expansion() {
        let wanted = [
            Direction::Est,
            Direction::West,
            Direction::North,
            Direction::South,
        ]
        .iter()
        .collect::<HashSet<_>>();

        assert_eq!(
            Direction::categories().iter().collect::<HashSet<_>>(),
            wanted
        )
    }
}
