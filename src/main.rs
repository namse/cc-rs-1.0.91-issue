#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let mut a = vec![];
    for i in 0..10 {
        a.push(i);
    }
    for i in 0..10 {
        a.push(a[i] * 2);
    }
    println!("{:?}", a);
}
