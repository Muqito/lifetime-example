use std::collections::HashMap;

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
    contents: Vec<&'a str>,
}
#[derive(Debug)]
struct BagOwnedData {
    color: String,
    contents: Vec<String>,
}
fn pt1_string(input: &str) -> HashMap<String, Vec<BagOwnedData>> {
    let mut bags: HashMap<String, Vec<BagOwnedData>> = HashMap::new();
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .for_each(|row| {
            let mut s = row.split(" bags contain ");
            let color = s.next().unwrap();
            let contents = s.next().unwrap();

            bags.insert(
                color.to_string(),
                vec![BagOwnedData {
                    color: color.to_string(),
                    contents: vec![contents.to_string()],
                }],
            );
        });

    bags
}
fn pt1_str(input: &str) -> HashMap<&str, Vec<Bag>> {
    let mut bags: HashMap<&str, Vec<Bag>> = HashMap::new();
    input
        .lines()
        .filter(|s| !s.is_empty())
        //.map(ToString::to_string) This would break the lifetime, however if you used BagOwnedData and String it would work, check pt1_string
        .for_each(|row| {
            let mut s = row.split(" bags contain ");
            //dbg!(row);
            let color = s.next().unwrap();
            let contents = s.next().unwrap();

            // This is fine, since input: &str goes out when pt1 goes out
            bags.insert(
                color,
                vec![Bag {
                    color,
                    contents: vec![contents],
                }],
            );
        });

    bags
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_DATA: &'static str = {
        r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#
    };
    #[test]
    fn test_str() {
        let d = pt1_str(INPUT_DATA);
        dbg!(d);
    }
    #[test]
    fn test_string() {
        let d = pt1_string(INPUT_DATA);
        dbg!(d);
    }
}
