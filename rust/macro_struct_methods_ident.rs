use core::future::Future;
use std::pin::Pin;
use std::collections::HashMap;
use tokio; // 1.15.0

macro_rules! impl_foo {
    ($t:ty, $($field:ident),+) => {
        impl $t {
            async fn methods(self) -> HashMap<String, Incrementer> {
                let mut methods_map: HashMap<String, Incrementer> = HashMap::new();
                $(
                    methods_map.insert(
                        stringify!($field).to_string(),
                        force_boxed::<_, $t>(<$t>::$field, self),
                    );
                )*
                methods_map
                
            }
        }
    }
}

fn force_boxed<T, S>(f: fn(S, u32) -> T, instance: S) -> Incrementer
where
    T: Future<Output = u32> + 'static,
    S: 'static,
{
    Box::new(move |n| Box::pin(f(instance, n)))
}

#[derive(Clone, Copy)]
struct Test {
    a: u32,
    b: u32,
}

#[derive(Clone, Copy)]
struct Plop {
    a: u32,
    b: u32,
}

type Incrementer = Box<dyn FnOnce(u32) -> Pin<Box<dyn Future<Output = u32>>>>;

impl Plop {
    async fn inc(self, src: u32) -> u32 {
        self.a + src
    }
    
    async fn inc2(self, src: u32) -> u32 {
        self.b + src
    }
}

impl Test {
    async fn test_a(self, src: u32) -> u32 {
        self.a + src
    }
    
    async fn test_b(self, src: u32) -> u32 {
        self.b + src
    }
}

impl_foo!(Plop, inc, inc2);
impl_foo!(Test, test_a, test_b);

async fn increment_printer(incr: Incrementer) {
    println!("{}", incr(1).await);
}

#[tokio::main]
async fn main() {
    let plop = Plop{a: 10, b: 100};
    let test = Test{a: 10, b: 1000};
    
    let mut plop_m = plop.methods().await;
    let mut test_m = test.methods().await;
    
    increment_printer(plop_m.remove(&"inc".to_string()).expect("Error")).await;
    increment_printer(plop_m.remove(&"inc2".to_string()).expect("Error")).await;
    
    increment_printer(test_m.remove(&"test_a".to_string()).expect("Error")).await;
    increment_printer(test_m.remove(&"test_b".to_string()).expect("Error")).await;
}
