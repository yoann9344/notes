/// This is how to pass an async struct's instance's method to a function
use core::future::Future;
use std::pin::Pin;
use tokio; // 1.15.0

#[derive(Clone, Copy)]
struct Plop {
    a: u32,
    b: u32,
}


impl Plop {
    async fn inc(self, src: u32) -> u32 {
        self.a + 1
    }
    
    async fn inc2(self, src: u32) -> u32 {
        self.b + 2
    }
}

type Incrementer = Box<dyn FnOnce(u32) -> Pin<Box<dyn Future<Output = u32>>>>;

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
    increment_printer(force_boxed(Plop::inc, plop)).await;
    increment_printer(force_boxed(Plop::inc2, plop)).await;
}
