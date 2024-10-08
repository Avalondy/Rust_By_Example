use std::str::FromStr;

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else { // count_str.parse::<u64>() else {
        panic!("Can't parse integer: '{count_str}'");
    };
    
    (count, item)
}