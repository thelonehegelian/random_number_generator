use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num1: u8 = rng.gen();
    let num2: u16 = rng.gen();

    println!("{}, {}", num1, num2);

    let num3 = rand::random::<u32>();
    let num4 = rand::random::<u64>();

    println!("{}, {}", num3, num4);
}
