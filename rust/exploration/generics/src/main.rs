// use len_trait::len::Len;
use rand::Rng;

fn main() {
    per_type_functions();
    generic_functions();
    generic_structs();
    custom_trait();
}

fn per_type_functions() {
    println!("Per type functions");
    let mut example_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut example_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Before shuffle example_array: {:?}", example_array);
    let example_slice: &mut [i32] = &mut example_array[..];
    let mut example_f64_vec: Vec<f64> = example_vec.iter().map(|x| *x as f64).collect::<Vec<_>>();
    println!("Before shuffle example_slice: {:?}", example_slice);
    println!("Before shuffle example_vec: {:?}", example_vec);
    println!("Before shuffle example_f64_vec: {:?}", example_f64_vec);
    // We have to use a different shuffle for the different types
    shuffle_i32_slice(&mut example_vec);
    shuffle_i32_slice(example_slice);
    shuffle_f64_slice(&mut example_f64_vec);
    println!("After shuffle example_slice: {:?}", example_slice);
    println!("After shuffle example_array (same as the slice !): {example_array:?}",);
    println!("After shuffle example_vec: {:?}", example_vec);
    println!("After shuffle example_f64_vec: {:?}", example_f64_vec);
    println!()
}

fn generic_functions() {
    println!("Generic functions");
    let mut example_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut example_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Before shuffle example_array: {:?}", example_array);
    let example_slice: &mut [i32] = &mut example_array[..];
    let mut example_f64_vec: Vec<f64> = example_vec.iter().map(|x| *x as f64).collect::<Vec<_>>();
    println!("Before shuffle example_slice: {:?}", example_slice);
    println!("Before shuffle example_vec: {:?}", example_vec);
    println!("Before shuffle example_f64_vec: {:?}", example_f64_vec);
    // We have to use a different shuffle for the different types
    shuffle(&mut example_vec);
    shuffle(example_slice);
    shuffle(&mut example_f64_vec);
    println!("After shuffle example_slice: {:?}", example_slice);
    println!("After shuffle example_array (same as the slice !): {example_array:?}",);
    println!("After shuffle example_vec: {:?}", example_vec);
    println!("After shuffle example_f64_vec: {:?}", example_f64_vec);
    println!()
}

fn shuffle_i32_slice(array: &mut [i32]) -> &mut [i32] {
    // A slice is a reference to any contiguous subset of elements
    // we can use the slice as a reference to the array/vector data so typing it like this
    // is more flexible than typing it as a vector
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let len: usize = array.len();
    for i in 0..len {
        let j = rng.gen_range(0..len);
        array.swap(i, j);
    }
    array
}

fn shuffle_f64_slice(array: &mut [f64]) -> &mut [f64] {
    let mut rng = rand::thread_rng();
    for i in 0..array.len() {
        let j = rng.gen_range(0..array.len());
        array.swap(i, j);
    }
    array
}

fn shuffle<T>(array: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in 0..array.len() {
        let j = rng.gen_range(0..array.len());
        array.swap(i, j);
    }
}

#[derive(Debug)]
struct Bag<T> {
    brand: String,
    items: Vec<T>,
}
impl<T> Bag<T> {
    fn get_first_item(&self) -> Option<&T> {
        self.items.first()
    }
}
impl Bag<JugglingBall> {
    fn juggle(&mut self) {
        shuffle(&mut self.items)
    }
}

#[derive(Debug)]
enum JugglingBallColour {
    Red,
    Blue,
    Green,
}
#[allow(dead_code)]
#[derive(Debug)]
struct JugglingBall {
    colour: JugglingBallColour,
}
// cheeky aliasing of a type here for brevity
type JBC = JugglingBallColour;

fn generic_structs() {
    println!("Generic structs");
    println!("Here we make two bags, one with strings and one with juggling balls");
    let nike_bag: Bag<&str> = Bag {
        brand: String::from("Nike"),
        items: vec!["hat", "shirt", "pants"],
    };
    let mut juggling_bag: Bag<JugglingBall> = Bag {
        brand: String::from("Clown max"),
        items: vec![
            JugglingBall { colour: JBC::Red },
            JugglingBall { colour: JBC::Blue },
            JugglingBall { colour: JBC::Green },
        ],
    };
    println!("My {}  bag has items {:?}", nike_bag.brand, nike_bag.items);
    println!(
        "My {}  bag has items {:?}",
        juggling_bag.brand, juggling_bag.items
    );
    println!(
        "The first item in nike_bag is {:?}",
        nike_bag.get_first_item()
    );
    juggling_bag.juggle();
}

pub trait Len {
    fn len(&self) -> usize;
}

impl Len for Vec<i32> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for &[T; 5] {
    // I don't understand this fully
    // I think the ** goes from &&[T; 5] to [T; 5]
    // where len is defined
    fn len(&self) -> usize {
        (**self).len()
    }
}

fn get_len<T>(t: T) -> i32
where
    T: Len,
{
    t.len() as i32
}

fn custom_trait() {
    println!("Custom traits");
    let example_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let example_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("example_vec has len {}", get_len(example_vec));
    println!("example_array has len {}", get_len(&example_array));
    println!();
}
