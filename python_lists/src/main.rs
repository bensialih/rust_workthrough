fn main() {
    let names = vec!["abdel", "mohammed", "lucy", "murad"];

    let map_of_names: Vec<String> = names
        .into_iter()
        .map(|x| format!("{}x", x))
        .filter(|x| *x != String::from("mohammedx"))
        .collect();

    println!("our vector of names {:?}", map_of_names);
}
