// structures!

use std::mem;
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    //method

    fn len(&self) -> f64 {
        //self basically the reference of a class in which we are working
        let dx = self.start.x - self.end.x; //basically it refers to the struct Point
        let dy = self.start.y - self.end.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({},{})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    println!("point p is at ({},{})", p2.x, p2.y);

    let myline = Line { start: p, end: p2 };

    // destructuring

    // member functions
    println!("line length is {}", myline.len());
}

// enumerations !

enum Color {
    Red, // unit-like struct
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple struct
    CmykColor {
        //struct like definition
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

pub fn enums() {
    let c = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    //let c = Color::RgbColor(0, 0, 0);
    //let c = Color::Blue;

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"), //if i remover this or any one enum data and _ => () this case, then it will throw an exception
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),

        Color::RgbColor(r, g, b) => println!("rgb({},{},{}", r, g, b),
        _ => (), //if it is some other color
    }
}

// unions !

// we don't know whether this contains an int
// or a float
//it only occupies 32 bits
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }

            //           ↓↓↓ we get to treat it as f32
            IntOrFloat { f } => {
                // we don't know if it's int or float
                //reinterprets cast.

                //You basically take this memory, which is an integer, and you're going to print it out as if it were representing a floating point number. So the result can be virtually anything.
                //when we take an integer and treat it as floating point
                println!("got some value which could be a float {}", f);
            }
        }
    }
}

pub fn union() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // cannot access member without an unsafe block, because compiler not sures that it would be int or float
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    let iof2 = IntOrFloat { f: 42.01 };

    process_value(iof2);

    // this will interpret an int as a float
    process_value(IntOrFloat { i: 123456 });
}

// option !
pub fn option_some_none() {
    let x: f32 = 2.0;
    let y: f32 = 1.0;
    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(v) => println!("{}/{}={}", x, y, v),
        None => println!("Cannot divide {} by {}", x, y),
    }
    //Rust also has special keywords specifically for checking whether or not something has none.
    //So one of those keywords is if let, while let etc.
    //now if the codition of Some(v) = result true then the if statement gets executed! basically what it does is it, first of all takes the result and then tries to match it against some of Z.
    if let Some(v) = result {
        println!("result = {}", v);
    }
}

// arrays !

pub fn arrays() {
    let mut a/*:[i32;5]*/ = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("first value of a is {}", a[0]);

    //assert_eq!(a, [321, 2, 3, 4, 5]);

    if a != [1, 2, 3, 5, 6]
    // size must match
    {
        println!("arrays not equal!");
    }

    // fill an array with 1s
    let b = [1u16; 10]; // try changing to 5
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // just print the entire arary
    println!("{:?}", b); //debug output
    println!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional array
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]]; //2 rows having 3 col.
    println!("{:?}", mtx);

    // print all the diagonal values
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

// slices !

fn use_slice(slice: &mut [i32]) {
    //&[ tells us that it is slice and we are borrowing a part of an array
    //so, we take a chunk of the data and feed it into this function
    println!("first elem is {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;

    // will crash
    //let z = slice[10];
}

pub fn slices() {
    // a slice is part of an array
    // its size is not known at compile time
    let mut data = [1, 2, 3, 4, 5];

    // start w/o mut, borrow as a slice
    use_slice(&mut data[1..4]); //[2,3,4]
    use_slice(&mut data); // entire array

    println!("data after slice use = {:?}", data);
}

// touples !

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub fn touple() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(3, 4);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1); // try sp.5

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element is {}", (combined.1).0); //(1 means, second row, and another 0 means first col.

    let ((c, d), (e, f)) = combined;

    // tuple of different elements
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // tuple of a single element
    let meanings = (42,); // start w/o comma, touple of single element
    println!("{:?}", meanings);
}

// pattern matching !

fn how_many(x: i32) -> &'static str {
    //&' means it returns a static string
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        //z @ 20...30 => "between 20 and 30", //z @ means, I'm just giving it a random name that is z
        //z if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 0); //point as a touple

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y), //if the x-axis matches to 0
        // also try ref and ref mut
        (ref x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?,{})", y), //we don't care about the value of x
    }

    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) | Color::CmykColor { black: 255, .. } => println!("black"), //here after balck:255, --> .. means we dont care about the other value
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

//generics

struct GenericPoint<T> {
    x: T,
    y: T,
}

struct GenericLine<T> {
    start: GenericPoint<T>,
    end: GenericPoint<T>,
}

pub fn generics() {
    let a: GenericPoint<i32> = GenericPoint { x: 0, y: 0 };
    let b = GenericPoint { x: 1.2, y: 3.4 };

    // won't work initially
    //let myline = Line { start: a, end: b }; //because the Point of a is integer and the point of b is float
}
