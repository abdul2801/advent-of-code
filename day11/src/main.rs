use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("failed to read input.txt");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(':').unwrap();
        let src = left.trim().to_string();
        let dests = right.split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(src, dests);
    }

    let mut memo = HashMap::new();
    let count = dfs(&graph, "svr", false, false, &mut memo);
    println!("Number of paths: {}", count);
}

fn dfs(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>
) -> usize {
    let key = (start.to_string(), seen_dac, seen_fft);

    if let Some(&v) = memo.get(&key) {
        return v;
    }

    if start == "out" {
        let result = if seen_dac && seen_fft { 1 } else { 0 };
        return result;
    }

    let seen_dac = seen_dac || (start == "dac");
    let seen_fft = seen_fft || (start == "fft");

    let mut count = 0;

    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            count += dfs(graph, neighbor, seen_dac, seen_fft, memo);
        }
    }

    memo.insert(key, count);
    count
}
