use core::future::Future;
use std::pin::Pin;
use std::collections::HashMap;
use tokio; // 1.15.0

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
    
    async fn methods(self) -> HashMap<String, Incrementer> {
        HashMap::from([
            ("inc".to_string(), force_boxed(Plop::inc, self)),
            ("inc2".to_string(), force_boxed(Plop::inc2, self)),
        ])
    }
}

fn force_boxed<T>(f: fn(Plop, u32) -> T, instance: Plop) -> Incrementer
where
    T: Future<Output = u32> + 'static,
{
    Box::new(move |n| Box::pin(f(instance, n)))
}

async fn increment_printer(incr: Incrementer) {
    println!("{}", incr(1).await);
}

#[tokio::main]
async fn main() {
    let plop = Plop{a: 10, b: 100};
    let mut m = plop.methods().await;
    
    // increment_printer(m.get(&"inc".to_string())).await;
    increment_printer(m.remove(&"inc".to_string()).expect("Error")).await;
    increment_printer(m.remove(&"inc2".to_string()).expect("Error")).await;
    // increment_printer(m[&"inc2".to_string()]).await;
}
