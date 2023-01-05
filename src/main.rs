// Generates random numbers using various methods
use rand::distributions::{Standard, Uniform};
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
    #[derive(Debug)]
    // Generate random values of a custom type e.g. tuples
    struct Point {
        x: i32,
        y: i32,
    }

    /*
    The Distribution trait is a trait that defines how a type can be used to generate random values.
    It has a single associated type, Sample, which represents the type of the values that can be generated.
    The Standard type is a type that implements the Distribution trait for all types that implement the
    Rand trait.
    In this impl block, the Distribution trait is being implemented for the Standard type,
    with the Point type as the Sample associated type. This means that the Standard type
    can now be used to generate random values of type Point.
    */

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let x: i32 = rng.gen();
            let y: i32 = rng.gen();
            Point { x, y }
        }
    }

    // create a random tuple
    // The ::<> syntax is used to specify the type parameter for a function or method in Rust
    let rand_tuple = rng.gen::<(i32, f32, f32)>();
    let rand_point: Point = rng.gen();
    println!("Random Tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    Ok(())
}
