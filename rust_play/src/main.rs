
mod test;

use std::rc::Rc;
use std::sync::Arc;


use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


fn main() {

    let rec = Rectangle{
        width: 30,
        height:50,
    };

    rec.area();
    rec.can_hold(&rec.clone());

    let (tx, rx) = mpsc::channel();

    let s = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    rx.try_recv().unwrap();

    let rect = Rectangle{
        width: 30,
        height:50,
    };

    test::hosting::t();

    let mut arr = vec![1,2,3,4,5];
    let mut arr2 = vec![1,2,3,4,5];
    let mut arr3 = vec![1,2,3,4,5];

    arr.push(1);
    arr.reserve(1024);
    arr.shrink_to_fit();
    arr.resize(512, 0);
    arr.append(&mut arr2);

    arr.capacity();
    arr.clear();
    arr.is_empty();
    let x = arr.clamp(arr2, arr3);

    let w = Box::new(1);
    let w2 = w.clone();


    println!("{:#?}", rect);

    let num = 1;

    let a = match num {
        1 => 1,
        _ => 0
    };

}

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &char) in bytes.iter().enumerate(){
        if char == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

use std::fmt::Display;

pub fn judge<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}



#[derive(Debug, Clone)]
pub struct tex_2d {
    width : u32,
    height : u32,
}

pub struct tex_3d {
    width : u32,
    height : u32,
    depth : u32,
}

pub struct tex_cube {
    faces : [tex_2d; 6],
}

impl tex_cube {
    pub fn new(width : u32, heigt : u32) -> tex_cube {
        tex_cube {
            faces : [
                tex_2d{ width : width, height : heigt },
                tex_2d{ width : width, height : heigt },
                tex_2d{ width : width, height : heigt },
                tex_2d{ width : width, height : heigt },
                tex_2d{ width : width, height : heigt },
                tex_2d{ width : width, height : heigt },
                ],
        }
    }
}

pub struct tex_rect {
    width : u32,
    height : u32,
}



struct Pair<T> {
    x : T,
    y : T
}

impl<T> Pair<T> {
    fn new () {}
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is larger than y");
        } else {
            println!("x is smaller than y");
        }
    }
}
