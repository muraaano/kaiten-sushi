use std::env;
use rand::seq::SliceRandom;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!();
    println!("へい　らっしゃい！");
    println!();

    shuffle(&mut args);

    for (i, arg) in args.iter().enumerate() {
        println!("{}皿目は、{}", i+1, arg);
    }

    println!();
    println!("おあいそ！");
    println!();
}


fn shuffle(args: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    args.shuffle(&mut rng);
}