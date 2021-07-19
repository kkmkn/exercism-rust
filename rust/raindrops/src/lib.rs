pub fn raindrops(n: u32) -> String {
    let is_factor = (n % 3 == 0, n % 5 == 0, n % 7 == 0);
    let n = n.to_string();
    let mut result: Vec<&str> = vec![];
    if is_factor.0 {
        result.push("Pling")
    }
    if is_factor.1 {
        result.push("Plang")
    }
    if is_factor.2 {
        result.push("Plong")
    }
    if is_factor == (false, false, false) {
        result.push(&n[..])
    }
    result.into_iter().collect()
}
