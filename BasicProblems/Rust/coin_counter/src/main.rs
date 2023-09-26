use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("This is my coin counter project!");

    let mut input: i32 = helper::get_number_from_user();
    
    let mut count_map: HashMap<i32, usize> = HashMap::new();

    while input > 0 {
        let highest_value = get_highest_subtractor(input);
        *count_map.entry(*highest_value).or_insert(0) += 1;
        input -= highest_value;
    }

    // Print the counts
    for (element, count) in &count_map {
        println!("Element {}: Count {}", element, count);
    }

    print_in_descending_order(count_map);
}

fn print_in_descending_order(count_map: HashMap<i32, usize>) {
    let mut sorted_entries: Vec<_> = count_map.iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(a.1));

    for (key, value) in sorted_entries {
        println!("Element: {}, Count: {}", key, value);
    }
}

fn get_highest_subtractor(input: i32) -> &'static i32 {
    const LIST: [i32; 8] = [200, 100, 50, 20, 10, 5, 2, 1];
    LIST.iter()
        .filter(|&&x| x <= input)
        .max()
        .unwrap_or(&0)
}