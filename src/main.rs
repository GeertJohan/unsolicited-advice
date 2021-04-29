use clap::Clap;
use rand::Rng;

mod pool;

#[derive(Clap)]
#[clap(version = "0.1", author = "Geert-Johan Riemer <opensource@geertjohan.net>")]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let list = pool::pool();
    let len = list.len();
    if opts.verbose {
        println!("Have {} lines of unsolicited advice", len);
    }

    let mut rng = rand::thread_rng();
    let pick = rng.gen_range(0..len);
    if opts.verbose {
        println!("Picked random number {}", pick);
    }

    println!("{}", list[pick]);
}
