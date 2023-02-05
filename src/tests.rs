use super::*;

#[test]
fn case_sensitive() {
    let query= "duct";
    let contents = "Rust: \nsafe, fast, productive.\nPick three.\nDuCt";

    
    let mut vector: Vec<&str> = Vec::new();
    vector.push("safe, fast, productive.");

    assert_eq!(vector, search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rust";

    let content = "Rust:\nsafe, fast, productive.\nPick three.\nrUst.";

    let mut vector: Vec<&str> = Vec::new();
    vector.push("Rust:");
    vector.push("rUst.");

    assert_eq!(vector, search_case_insensitive(query, content));
}