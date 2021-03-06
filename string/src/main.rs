fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // concatenating strings
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // push_str doesn't take ownership

    // adding a char
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // other ways to concatenate
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}",s3);

    // multiple strings
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s);
}
