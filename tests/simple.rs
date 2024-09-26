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

    #[derive(Category, Eq, PartialEq, Debug)]
    enum Ingredient {
        Butter,
        Eggs(usize),
        Flour,
        Sugar,
    }

    #[test]
    fn complexe_expansion() {
        let ingredients = Ingredient::categories();
        assert_eq!(ingredients.len(), 4);
        assert!(ingredients.contains(&Ingredient::Eggs(usize::default())));
        assert!(ingredients.contains(&Ingredient::Butter));
        assert!(ingredients.contains(&Ingredient::Flour));
        assert!(ingredients.contains(&Ingredient::Sugar));
    }
}
