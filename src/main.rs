#[derive(Debug, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    fn degrade(self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn sum(&self) -> u32 {
        self.x + self.y
    }

    fn increase(&mut self) {
        self.x += 1;
        self.y += 1;
    }

    fn increase_chained(&mut self) -> &mut Self {
        self.x += 1;
        self.y += 1;
        self
    }

    fn increase_owned(mut self) -> Self {
        self.x += 1;
        self.y += 1;
        self
    }
}

fn main() {
    let mut p = Point::new(1, 2);
    println!("{}", p.sum());
    println!("{}", Point::sum(&p));

    let r = &mut p;
    r.increase();
    println!("{:?}", *r);

    p.increase();
    println!("{:?}", p);

    Point::increase(&mut p);
    println!("{:?}", p);

    p.increase_chained().increase_chained();
    println!("{:?}", p);

    let p = p.increase_owned();
    println!("{:?}", p);

    let p = p.degrade();
    println!("{:?}", p);
}
