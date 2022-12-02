mod bags_loader;
mod bag;

//fn contains_gold(bag: (&String, &Vec<String>)) -> bool {
    //true
//}

fn main() {
    let bags = bags_loader::get_bags();
    let mut contains_gold: Vec<String> = vec![String::from("shiny gold")];
    for (key, value) in &bags {
        for i in 0..contains_gold.len() {
            if value.contains(&contains_gold[i]) {
                contains_gold.push(key.clone());
            }
        }
    }
    println!("Count: {}", contains_gold.len()-1);
}
