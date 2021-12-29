#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Cordinates {
    x : f32,
    y: f32
}

struct Line {
    starting_point : Cordinates,
    ending_point : Cordinates
}

pub fn test() {
    let p1 = Cordinates {x:5.0,y:10.0};
    let p2 = Cordinates {x:55.0,y:23.56};

    let line = Line{starting_point: p1, ending_point: p2};

    println!("line starting from X : {}  y: {}     and line edning at x : {}  y : {}",line.starting_point.x,line.starting_point.y,line.ending_point.x,line.ending_point.y);
}