use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("can't find ingredients list")]
    NoIngredients,

    #[error("allergen {0} doesn't correspond to any ingredients")]
    NoIngredientsForAllergen(Allergen),

    #[error("allergen {0} is ambiguous - corresponds to one of {1} ingredients")]
    TooManyIngredientsForAllergen(Allergen, usize),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Ingredient {
    name: String,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, PartialOrd)]
pub struct Allergen {
    name: String,
}

#[derive(Clone, Debug)]
struct Recipe(HashSet<Ingredient>);

#[derive(Debug)]
pub struct Menu {
    recipes_containing_allergen: HashMap<Allergen, Vec<Recipe>>,
    ingredients_count: HashMap<Ingredient, usize>,
}

impl From<&str> for Allergen {
    fn from(s: &str) -> Self {
        Allergen {
            name: s.to_string(),
        }
    }
}

impl std::fmt::Display for Allergen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for Ingredient {
    fn from(s: &str) -> Self {
        Ingredient {
            name: s.to_string(),
        }
    }
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::str::FromStr for Menu {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut recipes_containing_allergen = HashMap::new();
        let mut ingredients_count = HashMap::new();
        s.trim()
            .lines()
            .map(|line| {
                let mut allergens_split = line.split(" (contains ");
                let ingredients_list = allergens_split.next().ok_or(Error::NoIngredients)?;
                let recipe = Recipe(
                    ingredients_list
                        .split(' ')
                        .map(|s| Ingredient {
                            name: s.to_string(),
                        })
                        .collect::<HashSet<_>>(),
                );
                for ingredient in &recipe.0 {
                    *ingredients_count.entry(ingredient.clone()).or_insert(0) += 1;
                }
                if let Some(list) = allergens_split.next() {
                    let allergens = list
                        .trim_end_matches(')')
                        .split(", ")
                        .map(|s| Allergen {
                            name: s.to_string(),
                        })
                        .collect::<Vec<Allergen>>();
                    for allergen in allergens {
                        recipes_containing_allergen
                            .entry(allergen)
                            .or_insert_with(Vec::new)
                            .push(recipe.clone());
                    }
                };
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Menu {
            recipes_containing_allergen,
            ingredients_count,
        })
    }
}

impl Menu {
    fn possible_allergens(&self) -> HashMap<Allergen, HashSet<Ingredient>> {
        let m = self
            .recipes_containing_allergen
            .iter()
            .map(|(allergen, recipes)| {
                (
                    allergen.clone(),
                    recipes.iter().fold(self.ingredients_set(), |acc, r| {
                        acc.intersection(&r.0).cloned().collect::<HashSet<_>>()
                    }),
                )
            })
            .collect::<HashMap<Allergen, HashSet<Ingredient>>>();
        m
    }

    fn deduce_allergens(&self) -> HashMap<Allergen, Ingredient> {
        let mut allergen_map = HashMap::new();
        let mut possible_allergen_map = self.possible_allergens();
        while allergen_map.len() < possible_allergen_map.len() {
            let deduced = possible_allergen_map
                .iter()
                .find(|(_a, ingredients)| ingredients.len() == 1)
                .unwrap();
            let ingredient = deduced.1.iter().next().unwrap().clone();
            allergen_map.insert(deduced.0.clone(), ingredient.clone());
            for ingredients in possible_allergen_map.values_mut() {
                ingredients.remove(&ingredient);
            }
        }
        allergen_map
    }

    pub fn deduce_hypoallergens(&self) -> HashSet<Ingredient> {
        self.possible_allergens()
            .iter()
            .fold(self.ingredients_set(), |acc, (_, i)| {
                acc.difference(i).cloned().collect::<HashSet<_>>()
            })
    }

    fn ingredients_set(&self) -> HashSet<Ingredient> {
        self.ingredients_count
            .keys()
            .cloned()
            .collect::<HashSet<Ingredient>>()
    }

    pub fn count_ingredients_usage(&self, ingredients: HashSet<Ingredient>) -> usize {
        ingredients
            .iter()
            .map(|i| self.ingredients_count.get(i).unwrap())
            .sum()
    }

    pub fn canonical_dangerous_ingredients(&self) -> String {
        let mut allergens = self.deduce_allergens().into_iter().collect::<Vec<_>>();
        allergens.sort_unstable_by(|i, j| i.0.partial_cmp(&j.0).unwrap());
        allergens
            .iter()
            .map(|c| c.1.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MENU: &str = &r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_parse_menu() {
        let menu = TEST_MENU.parse::<Menu>().unwrap();
        assert_eq!(
            2,
            menu.recipes_containing_allergen
                .get(&Allergen::from("dairy"))
                .unwrap()
                .len()
        );
        assert_eq!(
            2,
            menu.recipes_containing_allergen
                .get(&Allergen::from("fish"))
                .unwrap()
                .len()
        );
        assert_eq!(
            1,
            menu.recipes_containing_allergen
                .get(&Allergen::from("soy"))
                .unwrap()
                .len()
        );
        assert_eq!(
            Some(&3),
            menu.ingredients_count.get(&Ingredient::from("mxmxvkd"))
        );
        assert_eq!(
            Some(&1),
            menu.ingredients_count.get(&Ingredient::from("kfcds"))
        );
        assert_eq!(
            Some(&3),
            menu.ingredients_count.get(&Ingredient::from("sqjhc"))
        );
        assert_eq!(
            Some(&1),
            menu.ingredients_count.get(&Ingredient::from("nhms"))
        );
        assert_eq!(
            Some(&1),
            menu.ingredients_count.get(&Ingredient::from("trh"))
        );
        assert_eq!(
            Some(&2),
            menu.ingredients_count.get(&Ingredient::from("fvjkl"))
        );
        assert_eq!(
            Some(&2),
            menu.ingredients_count.get(&Ingredient::from("sbzzf"))
        );
    }

    #[test]
    fn test_menu_deduce_hypoallergens() {
        let menu = TEST_MENU.parse::<Menu>().unwrap();
        assert_eq!(5, menu.count_ingredients_usage(menu.deduce_hypoallergens()));
    }

    #[test]
    fn test_menu_deduce_allergens() {
        let menu = TEST_MENU.parse::<Menu>().unwrap();
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
            menu.canonical_dangerous_ingredients(),
        );
    }
}
