// Generates random numbers using various methods
use rand::distributions::Uniform;
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};

fn main() -> Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    let num1: u8 = rng.gen();
    let num2: u16 = rng.gen();

    println!("{}, {}", num1, num2);

    let num3 = rand::random::<u32>();
    let num4 = rand::random::<u64>();

    println!("{}, {}", num3, num4);

    // random numbers within a range
    let num5 = rng.gen_range(1..100);
    println!("Random Integer: {}", num5);

    let num6 = rng.gen_range(0.0..100.0);
    println!("Random Float: {}", num6);

    // Generate Uniform and Distributed random numbers

    let dice = Uniform::from(1..7);

    // roll the dice

    loop {
        let throw = dice.sample(&mut rng); // sample uniformly from a range;
        println!("Roll: {}", throw);
        if throw == 6 {
            break;
        }
    }

    // using various distribution types
    let normal = Normal::new(2.0, 5.0)?;
    let smpl = normal.sample(&mut rng);
    println!("normal: {}", smpl);

    Ok(())
}
