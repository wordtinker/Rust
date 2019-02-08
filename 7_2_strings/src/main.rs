fn main() {
    // Unicode
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // both are borrowed
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // + is overloaded for strings
    let s4 = String::from(s2);
    // note s1 moved to s3
    let s3 = s1 + &s4;
    println!("{:?}", s3);
    // Format also works
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}", s);

    // indexing is forbidden
    let hello = String::from("नमस्ते");
    // deepest level raw bytes
    for b in hello.bytes() {
        println!("{:?}", b);
    }
    // mid level unicode scalars
    for c in hello.chars() {
        println!("{:?}", c);
    }
    // top level graphem clusters see crates
}
