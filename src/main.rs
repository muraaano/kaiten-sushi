use std::env;
use rand::Rng;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("へい　らっしゃい！");
    println!();

    args = shuffle(args);

    for (i, arg) in args.iter().enumerate() {
        println!("{}皿目は、{}", i+1, arg);
    }

    println!();
    println!("おあいそ！");
}


fn shuffle(args: Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    args.shuffle(&mut rng);
}