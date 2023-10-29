use std::{cell::RefCell, ops::Deref, rc::Rc};
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = &x;

    let z = Box::new(x);

    let z1 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *z1);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");

    //显示丢弃值
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    //Rc<T>  引用计数  多重所有权

    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    let b = ListRc::Cons(3, Rc::clone(&a));
    let c = ListRc::Cons(4, Rc::clone(&a));
    println!("{:?}-{:?}-{:?}", a, b, c);

    //打印引用计数
    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //Rc 和 RefCell 结合使用来实现一个拥有多重所有权的可变数据

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListRcAndRef::Cons(
        Rc::clone(&value),
        Rc::new(ListRcAndRef::Nil),
    ));

    let b = ListRcAndRef::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    let c = ListRcAndRef::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:#?}",a);
    println!("b after = {:#?}",b);
    println!("c after = {:#?}",c);
}

#[derive(Debug)]
enum ListRcAndRef {
    Cons(Rc<RefCell<i32>>, Rc<ListRcAndRef>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

// 无法编译
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
