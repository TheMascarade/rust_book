use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    /// Print area
    #[arg(short, long)]
    area: bool,
    /// Print perimeter
    #[arg(short, long)]
    perimeter: bool,
    /// A shape
    #[command(subcommand)]
    shape: Shapes,
}

trait ShapeOps {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

#[derive(Subcommand)]
enum Shapes {
    /// Circle
    Circle(Circle),
    /// Rectangle
    Rectangle(Rectangle),
    /// Triangle
    Triangle(Triangle),
}

impl ShapeOps for Shapes {
    fn get_area(&self) -> f64 {
        match self {
            Shapes::Circle(circle) => circle.get_area(),
            Shapes::Rectangle(rectangle) => rectangle.get_area(),
            Shapes::Triangle(triangle) => triangle.get_area(),
        }
    }
    fn get_perimeter(&self) -> f64 {
        match self {
            Shapes::Circle(circle) => circle.get_perimeter(),
            Shapes::Rectangle(rectangle) => rectangle.get_perimeter(),
            Shapes::Triangle(triangle) => triangle.get_perimeter(),
        }
    }
}

#[derive(Args)]
struct Circle {
    /// The radius
    #[arg(short, long)]
    radius: f64,
}

impl ShapeOps for Circle {
    fn get_area(&self) -> f64 {
        self.radius * 2.0 * std::f64::consts::PI
    }
    fn get_perimeter(&self) -> f64 {
        self.radius.exp2() * std::f64::consts::PI
    }
}

#[derive(Args)]
struct Rectangle {
    /// The width
    #[arg(short, long)]
    width: f64,
    /// The height
    #[arg(short = 'e', long)]
    height: f64,
}

impl ShapeOps for Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }
    fn get_perimeter(&self) -> f64 {
        2.0 * self.height + 2.0 * self.width
    }
}

#[derive(Args)]
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl ShapeOps for Triangle {
    fn get_area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn get_perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn main() {
    let args = Cli::parse();
    if args.perimeter {
        println!("Perimeter: {:.2}", args.shape.get_perimeter())
    }
    if args.area {
        println!("Area: {:.2}", args.shape.get_area())
    }
}
