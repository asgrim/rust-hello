pub fn run()
{
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

    println!("{} (len: {}) ... {}", s1, l, s2);
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
