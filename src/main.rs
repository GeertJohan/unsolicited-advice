use clap::Parser;
use rand::Rng;

mod pool;

#[derive(Parser)]
#[command(
    version = "0.1",
    author = "Geert-Johan Riemer <opensource@geertjohan.net>",
    about = "Advice by Kevin Kelly, https://kk.org/thetechnium/68-bits-of-unsolicited-advice/"
)]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[arg(short, long)]
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
