mod test_solutions;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use log::{debug, info};
use std::collections::{HashMap, HashSet, BTreeMap};
use itertools::{concat};

fn get_input_data(filename: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut ingredients_list: Vec<Vec<String>> = Vec::new();
    let mut allergens_list: Vec<Vec<String>> = Vec::new();

    let pattern = Regex::new(r"(.+)\(contains (.+)\)").unwrap();

    for line in f.lines() {

        let line = &line.unwrap();
        let ingredient_allergen_match = pattern.captures(line).unwrap();
        let ingredients = ingredient_allergen_match.get(1).unwrap()
            .as_str().split_whitespace();
        let allergens = ingredient_allergen_match.get(2).unwrap()
            .as_str().split(", ");
        ingredients_list.push(ingredients.map(|s| s.to_owned()).collect());
        allergens_list.push(allergens.map(|s| s.to_owned()).collect());
    }
    return (ingredients_list, allergens_list);
}

fn build_allergen_ingredients_list_dict(ingredients_list: &Vec<Vec<String>>,
                                        allergens_list: &Vec<Vec<String>>)
    -> HashMap<String, Vec<HashSet<String>>> {
    let mut dict: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    for (i, allergens) in allergens_list.iter().enumerate() {
        for allergen in allergens {
            let current_list = dict.get_mut(allergen);
            if current_list.is_some() {
                current_list.unwrap().push(ingredients_list[i].iter().cloned().collect());
            } else {

                dict.insert(allergen.to_owned(),
                            vec![ingredients_list[i].iter().cloned().collect()]);
            }
        }
    }
    return dict;
}

fn build_allergen_ingredients_list_flat_dict(ingredients_list: &Vec<Vec<String>>,
                                             allergens_list: &Vec<Vec<String>>)
    -> HashMap<String, HashSet<String>> {
    let allergen_ingredients_list_dict =
        build_allergen_ingredients_list_dict(&ingredients_list, &allergens_list);
    debug!("{:?}", allergen_ingredients_list_dict);
    let mut allergen_ingredients_list_flat_dict = HashMap::new();
    for (allergen, ingredients_list) in allergen_ingredients_list_dict {
        let mut first = ingredients_list.get(0).cloned().unwrap();
        for ingredients in &ingredients_list[1..] {
            first = first.intersection(ingredients).map(|s| s.to_owned()).
                collect();
        }
        allergen_ingredients_list_flat_dict.insert(allergen, first);
    }
    return allergen_ingredients_list_flat_dict;
}

fn solution_part_1(filename: &str) -> i32 {
    let (ingredients_list, allergens_list) = get_input_data(filename);
    debug!("{:?}", allergens_list);
    let all_ingredients: HashSet<String> = concat(ingredients_list.iter().cloned()).
        iter().cloned().collect();
    debug!("All Ingredients: {:?}", all_ingredients);
    let allergen_ingredients_list_flat_dict =
        build_allergen_ingredients_list_flat_dict(&ingredients_list, &allergens_list);
    debug!("Allergens ingredients flat list: {:?}", allergen_ingredients_list_flat_dict);
    let mut ingredients_with_allergen = HashSet::new();
    for (_allergen, ingredients_list) in &allergen_ingredients_list_flat_dict {
        ingredients_with_allergen = ingredients_with_allergen.union(ingredients_list).
            cloned().collect();
    }
    debug!("Ingredients with allergens list: {:?}", ingredients_with_allergen);
    let remaining_ingredients: HashSet<String> = all_ingredients.
        difference(&ingredients_with_allergen).cloned().collect();
    debug!("Remaining ingredients: {:?}", remaining_ingredients);
    let mut count = 0;
    for ingredients in &ingredients_list {
        for ingredient in ingredients {
            if remaining_ingredients.contains(ingredient) {
                count += 1;
            }
        }
    }
    return count;
}

fn find_allergen_with_one_ingredient(flat_dict: &HashMap<String, HashSet<String>>) -> String {
    for (allergen, ingredients) in flat_dict {
        if ingredients.len() == 1 {
            return allergen.to_owned();
        }
    }
    return "not_found".to_owned();
}

fn remove_element_from_list(flat_dict: &mut HashMap<String, HashSet<String>>, ingredient: String) {
    for (_allergen, ingredients) in flat_dict {
        ingredients.remove(&ingredient);
    }
}

fn solution_part_2(filename: &str) -> String {
    let (ingredients_list, allergens_list) = get_input_data(filename);
    let mut allergen_ingredients_list_flat_dict =
        build_allergen_ingredients_list_flat_dict(&ingredients_list, &allergens_list);
    let mut result_list: BTreeMap<String, String> = BTreeMap::new();
    while allergen_ingredients_list_flat_dict.len() > 0 {
        debug!("{:?}", allergen_ingredients_list_flat_dict);
        let allergen = find_allergen_with_one_ingredient(&allergen_ingredients_list_flat_dict);
        let ingredient = allergen_ingredients_list_flat_dict.
            remove(&allergen).unwrap().iter().nth(0).unwrap().to_owned();
        result_list.insert(allergen.to_owned(), ingredient.clone());
        remove_element_from_list(&mut allergen_ingredients_list_flat_dict, ingredient);
    }
    debug!("{:?}", result_list);
    let mut result = String::new();
    for (_allergen, ingredient) in result_list {
        result += &ingredient;
        result += ",";
    }
    result.remove(result.len() - 1);
    return result;
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
