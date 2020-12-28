use std::fs;
use std::collections::{BTreeMap,HashSet};

struct Food<'a> {
    allergens: HashSet<&'a str>,
    ingredients: HashSet<&'a str>
}

fn split2<'a>(s: &'a str, p: &str) -> (&'a str,&'a str) {
    let mut splitter = s.splitn(2,p);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    return (first,second);
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Food<'a> {
        let (mut head,mut tail) = split2(s, " ");
        let mut oldtail = s;
        let mut ingredients: HashSet<&str> = HashSet::new();
        while !oldtail.starts_with("(") {
            ingredients.insert(head);
            let (newhead,newtail) = split2(tail, " ");
            head = newhead;
            oldtail = tail;
            tail = newtail;
        }
        let allergen_string = &oldtail[10..oldtail.len()-1];
        let allergens = allergen_string.split(", ").collect::<HashSet<&str>>();
        return Food {allergens, ingredients};
    }
}

impl<'a> std::fmt::Display for Food<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ingredients: {:?} , Allergens: {:?}", 
        self.ingredients.iter().collect::<Vec<&&str>>().as_slice(),
        self.allergens.iter().collect::<Vec<&&str>>().as_slice())
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let foods: Vec<Food> = file.lines().map(Food::parse).collect();
    let all_ingredients: HashSet<&str> = foods.iter().map(|f| &f.ingredients).fold(HashSet::<&str>::new(),|s,i| i.union(&s).map(|s|*s).collect());
    let all_allergens: HashSet<&str> = foods.iter().map(|f| &f.allergens).fold(HashSet::<&str>::new(),|s,i| i.union(&s).map(|s|*s).collect());
    let mut allergen_candidates: BTreeMap<&str,HashSet<&str>> = all_allergens.iter().map(|a| (*a,all_ingredients.clone())).collect();
    for food in &foods {
        for allergen in &food.allergens {
            let remaining = allergen_candidates.get(allergen).unwrap().intersection(&food.ingredients).map(|s|*s).collect();
            allergen_candidates.insert(allergen, remaining);
        }
    }
    let possible_allergenic_ingredients = allergen_candidates.iter().map(|(_,hs)|hs).fold(HashSet::<&str>::new(),|s1,s2| s1.union(s2).map(|s|*s).collect::<HashSet<&str>>());
    let nonallergenic_ingredients: HashSet<&str> = all_ingredients.difference(&possible_allergenic_ingredients).map(|s|*s).collect();
    println!("There are {} ingredients that are definitely not allergenic.", nonallergenic_ingredients.len());
    let all_nonallergenic_occurrances = foods.iter().flat_map(|f| f.ingredients.iter().map(|s|*s).filter(|i| nonallergenic_ingredients.contains(i))).collect::<Vec<&str>>();
    println!("Across all food items, there are {} occurrances of ingredients that are definitely not allergenic.", all_nonallergenic_occurrances.len());
    while allergen_candidates.iter().map(|(_,s)|s.len()).max().unwrap() > 1 {
        let mut ingredients_to_remove: HashSet<&str> = HashSet::new();
        for (_,candidates) in &allergen_candidates {
            if candidates.len() == 1 {
                ingredients_to_remove.insert(candidates.iter().next().unwrap());
            }
        }
        for allergen in &all_allergens {
            if allergen_candidates.get(allergen).unwrap().len() == 1 {continue;}
            let update: HashSet<&str> = allergen_candidates.get(allergen).unwrap().difference(&ingredients_to_remove).map(|s|*s).collect();
            allergen_candidates.insert(allergen, update);
        }
    }
    println!("The allergens are: {:?}", allergen_candidates.keys());
    let dangerous_ingredient_list: String = allergen_candidates.keys().map(|k| allergen_candidates.get(k).unwrap()).fold(String::new(),|s,hs|s + *hs.iter().next().unwrap() + ",");
    println!("Translating these gives: {}", dangerous_ingredient_list.trim_end_matches(","));
}
