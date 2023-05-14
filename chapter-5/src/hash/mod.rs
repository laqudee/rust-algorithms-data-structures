/// Rust hash
///
pub mod hash_map;

pub fn hash1(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for c in astr.chars() {
        sum += c as usize;
    }

    sum % size
}

pub fn hash2(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in astr.chars().enumerate() {
        sum += (i + 1) * (c as usize);
    }
    sum % size
}

pub fn it_work() {
    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash1(s1, size);
    let p2 = hash1(s2, size);
    println!("s1 in slot {}, s2 in slot {}", p1, p2);

    let p3 = hash2(s1, size);
    let p4 = hash2(s2, size);
    println!("s3 in slot {}, s4 in slot {}", p3, p4);

    // Test HashMap
    let mut hmap = hash_map::HashMap::new(11);
    hmap.insert(10, "cat");
    hmap.insert(2, "dog");
    hmap.insert(3, "mouse");

    println!("HashMap size {:?}", hmap.len());
    println!("HashMap contains key 2: {}", hmap.contains(2));
    println!("HashMap key 3: {:?}", hmap.get(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));
}
