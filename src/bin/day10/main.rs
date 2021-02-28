use std::collections::BTreeMap;

#[test]
fn test_simple_adapters() {
    let adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let frequencies = adapter_chain_distribution(&adapters);
    assert_eq!(7, frequencies[&1]);
    assert_eq!(5, frequencies[&3]);
}

#[test]
fn test_complex_adapters() {
    let adapters = [28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    let frequencies = adapter_chain_distribution(&adapters);
    assert_eq!(22, frequencies[&1]);
    assert_eq!(10, frequencies[&3]);
}

fn create_default_adapters(adapters: &[usize]) -> Vec<usize> {
    let mut adapters:Vec<usize> = adapters.into();
    adapters.sort();
    adapters.insert(0,0);
    adapters.push(3 + adapters.iter().max().unwrap());
    adapters
}

fn adapter_chain_distribution(adapters: &[usize]) -> BTreeMap<usize, usize> {
    let adapters = create_default_adapters(adapters);
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    adapters.windows(2)
        .map(|x| x.last().unwrap() - x.first().unwrap())
        .for_each(|i| *res.entry(i).or_insert(0) += 1);
    res
}

fn nr_valid_chains(adapters: &[usize]) -> usize {
    let adapters = create_default_adapters(adapters);
    let mut cache: BTreeMap<usize, usize> = BTreeMap::new();
    cache.insert(0, 1);
    for (k, v) in adapters[1..].iter().enumerate() {
        let acc = adapters[..k + 1].iter()
            .enumerate()
            .filter(|(_, v2)| v - *v2 <= 3)
            .filter_map(|(k, _)| cache.get(&k))
            .sum();
        cache.insert(k + 1, acc);
    }
    *cache.values().last().unwrap()
}


fn main() {
    let s = include_str!("input.txt");
    let adapters: Vec<_> = s.lines().map(|x| x.parse().unwrap()).collect();
    let frequencies = adapter_chain_distribution(&adapters);
    println!("Problem1: {}", frequencies[&1] * frequencies[&3]);
    println!("Problem2: {}", nr_valid_chains(&adapters));
}
