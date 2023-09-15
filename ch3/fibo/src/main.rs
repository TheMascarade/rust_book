use clap::Parser;

#[derive(Parser)]
#[command(author = "Matias Bella, <dummy@gmail.com>")]
#[command(about = "Utilidad que imprime el -n elemento en la secuencia de Fibonacci")]
#[command(version = "1.0")]
struct Cli {
    /// El eneabo numbero en la secuencia Fibonacci
    #[arg(long, short, default_value = "1")]
    number: usize,
}

fn main() {
    let args = Cli::parse();
    let fibo_number = args.number;
    println!(
        "Fibonacci number {}: {}",
        fibo_number,
        get_fibo(fibo_number)
    );
}

fn get_fibo(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        get_fibo(n - 1) + get_fibo(n - 2)
    }
}
