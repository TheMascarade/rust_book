use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "tempcv")]
#[command(author = "Matias Bella <dummy@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Aplicacion para convertir entre unidades de temperatura")]
#[command(help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]

/// Convertur de unidades de temperatura escrito en Rust
struct Temp {
    /// El valor de la medicion
    #[arg(short = 'd', long)]
    temp: f64,
    /// La unidad de la medicion
    #[arg(value_enum, long = "from", short = 'f')]
    units_from: Units,
    /// La unidad de conversion
    #[arg(value_enum, long = "to", short = 't')]
    units_to: Units,
}

#[derive(PartialEq, Clone, ValueEnum)]
enum Units {
    Celsius,
    Farenheit,
}

fn main() {
    let args = Temp::parse();
    if args.units_from == args.units_to {
        println!("{}", args.temp);
        return;
    }
    match args.units_to {
        Units::Celsius => println!("{}", farenheit_to_celsius(args.temp)),
        Units::Farenheit => println!("{}", celsius_to_farenheit(args.temp)),
    }
}

fn farenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}
