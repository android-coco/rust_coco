use std::ops::Add;
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("Hello, world! {:?} {:?}", r1, r2);
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    let address = 0x7ff7b93f8d7cusize;

    let r = address as *const i32;
    println!("Hello, world! {:?}", r);

    //创建不安全的代码

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //调用另外一个语言中定义的extern函数
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according  to C: {}", abs(-13));
    }

    //给C语言调用的函数
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    //全局变量  静态变量
    static HELLO_WORLD: &str = "Hello, world!";

    println!("name is: {}", HELLO_WORLD);

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    //运算符重载
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Add tait 使用了 泛型默认参数  pub trait Add<Rhs = Self> {....}
    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
