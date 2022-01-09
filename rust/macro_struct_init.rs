#[derive(Debug)]
#[allow(dead_code)]
struct Plop {
    a: i32,
    b: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Test {
    a: i32,
    b: String,
}

macro_rules! test {
    ($t:ident, $($name:literal),+) => {
        // type tmp = $t;
        $(
            println!("{:?}", $t{a: 4, b: $name.to_string()});
        )+
    };
}

fn main(){
    test!(Plop, "blabla", "plop", "test");
    test!(Test, "blabla", "plop", "test");
}
