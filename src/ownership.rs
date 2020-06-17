pub fn run() {
    // Will fail, block scopes apply
    // {
    //     let s = "hello";
    // }
    // println!("{}", s);

    let mut s1 = String::from("Hello");
    let mut s2 = s1.clone();

    // unlike PHP (pass by ref), "ownership" of s2 passes to fn worldify
    // so we return the value and assign to s2, seems correct (transfer "ownership" back)
    s1 = worldify(s1);

    // Using the & is passing in a non-mutable reference, called "borrowing" in Rust world
    let l = strlen(&s1);

    // this is a mutable reference ("mutable borrow"); appears to work in same way as PHP's pass by ref
    append_world_with_mutable_reference(&mut s2);

    // Cannot have multiple mutable references; only multiple immutable refs
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    let r1 = &s1;
    let r2 = &s1;
    println!("r1={}\nr2={}", r1, r2);

    let fw = first_word(&s1);

    println!("s1={} (l={})\ns2={}\nfw={}", s1, l, s2, fw);
}

fn worldify(mut s: String) -> String {
    s.push_str(" world");
    return s;
}

// If we want read-only access, we can pass by ref with & (both in signature and call-time)
fn strlen(s: &String) -> usize {
    return s.len();

    // If we wanted to mutate, we'd have to "deep copy" or clone:
    // let mut s_copy = s.clone();
    // s_copy.push_str("longer than you thought");
    // return s_copy.len();
}

fn append_world_with_mutable_reference(s: &mut String) {
    s.push_str(" WORLD");
}

// Demonstrates "slices", can slice strings, arrays
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
