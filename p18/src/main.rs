fn main() {
    //混合使用if let  else if   else if let 和else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color {}, as ther background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using bule as the background color");
    }

    //while let

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //for 解构元组

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // | 或者  ..= 匹配范围

    let x = 1;
    match x {
        // 1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //解构结构体

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    //解构枚举

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g,b )) => {
            println!("Change the color to red {}, green {}, and blue {}",r,g,b);
        },
        Message::ChangeColor(Color::Hsv(h, s, v) ) => {
            println!("Change the color to hue {}, saturation {}, and value {}",h,s,v);
        },
        _ => println!(),
    }

    //解构结构体和元组 
    let((feet,inches),Point { x, y }) = ((3,10),Point{x:3,y:-10});

    //匹配守卫

    let num = Some(4);

    match num {
        Some(x) if x < 3 => println!("less than file: {}",x),
        Some(x) => println!("{}",x),
        None => (),
    }
}
