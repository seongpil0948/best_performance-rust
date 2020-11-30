// 러스트는 기본적으로 디버깅을 지원하지만 구조체는 명시적 구현 필요
// 관계를 돈독히.
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Method
    // define in struct context
    // method's args[0]은 self 여야 한다.
    // signiture = f64 
    // mut &self
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    println!("p1: {:#?} \n p2: {:?}", p1, p2);

    // 이 둘은 기능이 같다 근데
    p1.distance(&p2);
    (&p1).distance(&p2);
}
