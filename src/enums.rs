// Yes, same can be accomplished with Option, just experiments...
enum NullableString {
    HasValue(String),
    NoValue,
}
impl NullableString {
    fn to_string(&self) -> String {
        match self {
            NullableString::HasValue(i) => i.to_owned(),
            NullableString::NoValue => String::from("No value"),
        }
    }
}

pub fn run()
{
    let w: Option<String> = Some(String::from("Hi"));
    let x: Option<String> = None;

    // match w {
    //     Some(i) => println!("w={}", i),
    //     // None => println!("w is None"),
    //     None => (),
    // }
    // match x {
    //     Some(i) => println!("x={}", i),
    //     // None => println!("x is None"),
    //     None => (),
    // }

    // can just match one "arm" with: (i.e. don't do anything with None)
    if let Some(a) = w {
        println!("w WAS {}", a);
    }

    let y = NullableString::NoValue;
    let z = NullableString::HasValue(String::from("LOLZ"));
    println!("y={} z={}", y.to_string(), z.to_string());

    // println!("{}", x.unwrap());

}
