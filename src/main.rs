use options::Options;

mod options;

fn main() {
    let options = Options::new();
    let windowed = options.is_set("-windowed");
    let alpha = options.check_param::<u32>("-alpha");
    
    println!("{}, {:?}", windowed, alpha);
}
