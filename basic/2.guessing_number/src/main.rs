// Import Library IO in Std
use std::io;

// return a value of Enum(Less, Greater, Equal)
use std::cmp::Ordering;
// Rng Trait(특징/특성 -> Interface and Include Abstract Class (구현 요구 메소드도 있을 수 있다.))
use rand::Rng;

fn main() {
    println!("어디한번 숫자를 맞춰보시지!");
    // thread_rng = nansu generator in excuted Thread
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!( "사실 정답은 {} 이야", secret_num);

    loop {
        println!("정답을 입력 하세요! ");
        // default is Imutable
        // define variable as mutable
        // encoded by UTF-8
        // new is method of String
        let mut guess = String::new();

        // std::io::stdin if not use
        // read_line is return a value of Type io::Result (Enum value has Ok and Error)
        // expect is method of Result and print arg if result value is Error
        // input by stdin  is include \n because enter
        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했다.");
        println!("입력한 값: {}", guess);

        // u32(unsigned) is 32bit not allow negative but contrasting i32(signed) is allow
        // this imutable variable that shadowing before defined
        // parser convert type from string to inffered u32
        // matching result from parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // cmp(compare) result a value of type according Ordering
        // match consist of several(여러가지) arms(가지들)
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("입력한 숫자가 작다."),
            Ordering::Greater => println!("입력한 숫자가 크다."),
            Ordering::Equal => {
                println!(" 정 답 ");
                break;
            }
        }
    }
}
