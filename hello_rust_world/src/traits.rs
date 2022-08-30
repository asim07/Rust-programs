trait Animal {
    //static func
    fn create(name: &'static str) -> Self; //return a implementor type, and create an instance of human and cat
                                           //there is need to implement these trait because the another func have a default implementation that you can override!
                                           //below 2 are the instance func
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk!", self.name());
    }
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

//Human is an animal and can implement some of the traits of Animal
impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name } //initializing a Human with a given name
    }
    //Now Human can use the functions of trait Animal
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} say hello", self.name())
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name } //initializing a Cat with a given name
    }
    //Now Human can use the functions of trait Animal
    fn name(&self) -> &'static str {
        self.name //name getting from Cat struct
    }
    fn talk(&self) {
        println!("{} say meow", self.name())
    }
}

//i'm making my own trait here, vector by itself is a generic type so,
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

pub fn trait_func() {
    // let h = Human { name: "Maham" };
    // let h = Human::create("maham"); //:: means it's a factory func
    let h: Human = Animal::create("sheeza");
    h.talk();

    //let c = Cat { name: "Misty" };
    let c = Cat::create("Misty");
    c.talk();

    // you can add behavior to times which you don't even own, which you haven't yourself created.
    let a = vec![1, 2, 3];
    println!("sum is {}", a.sum());
}

//////////////////////////////////////////////////////////////
// trait parameters!
//////////////////////////////////////////////////////////////

use std::fmt::Debug;

#[derive(Debug)] //ask compiler to implement the debug trait for me too
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
//fn print_info(shape: impl Shape + Debug) { //it implements 2 traits now
//fn print_info<T: Shape + Debug>(shape: T) {
//the benefit of using the second approach is if we have multiple arguments then we can use (shape: T, shape2: T)
//otherwise you have to write shape: impl Shape + Debug then shape2:  impl Shape + Debug
fn print_info<T>(shape: T)
where
    T: Shape + Debug, //immediatly follows the signature of actual function
{
    //type T is built by the combination of Shape and Debug
    //shape of type T is the argument here
    println!("{:?}", shape); //Now i can get the debug output here
    println!("The area is {}", shape.area());
}

pub fn trait_parameters() {
    let c = Circle { radius: 2.0 };
    print_info(c);
}

//Into trait

struct Person {
    name: String,
}

//I want a factory method to actually construct a  Person

impl Person {
    // fn new(name: &str) -> Person {
    //     Person {
    //         name: name.to_string(),
    //     }
    // }
    //function that is generic <> as well as trait
    //fn new<S: Into<String>>(name: S) -> Person {
    fn new<S>(name: S) -> Person
    where
        S: Into<String>,
    {
        Person { name: name.into() } //call a coversion that would covert the name into string
    }
}

pub fn into_trait() {
    let maham = Person::new("Maham");
    let name: String = "Sheeza".to_string();
    //we would have to convert that name, which is a string into a SDR reference.
    let sheeza = Person::new(name /*.as_ref() */);
    //into allows automatic conversion where possilble
}

// Drop trait!

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters in the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    //creature gets destroyed at this point, and the variable gets cleaned up
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

pub fn drop_trait() {
    let goblin = Creature::new("Jeff");
    println!("Game proceeds!");
    drop(goblin); //if i call here then it doesn't gets call it again and you cannot use the goblin variable anymore!
}

//////////////////////////////////////////////////////////////
// Operator Overloading !
//////////////////////////////////////////////////////////////

//Operator overloading can be done from trait
use std::ops::{Add, AddAssign, Neg};

//derive debug output
#[derive(Debug, PartialEq, Eq)] //the way to actually outputing the type
struct Complex<T> {
    re: T, //real
    im: T, //imaginary
}
//for the construction of the Complex numbers, we need a factory method
impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}
//let's implement the trait of Add
//Operater Overloading
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>; //Associated type or method,a type that you have to specify for a particular trait to work.
                              //In this case, what we need to specify is we need to specify the result type of the addition operation.
                              //In a+b, self is the reference of the left side a, right side rhs have the reference of b
    fn add(self, rhs: Self) -> Self::Output {
        //we need to specify the result type of a addition operation
        //Self represents the type where we are right now that is Complex<i32> Now
        Complex {
            re: self.re + rhs.re, //After making this generic, to get this + functionality you need to add where in above to specify that T supports addition because currently it is basically T + T
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

//other then the below code, i can also derive it, you can see line 189

//2 types of equality ->
//partial eq x
//full eq  -> x = x //cannot satisfy in floating case
//NAN = not a number 0/0 inf/inf
//If you check NAN == NAN --> false .........> full eq is violated at this point
// impl<T> PartialEq for Complex<T>
// where
//     T: PartialEq,
// {
//     //An Traits, there are 2 functions there is need to implement eq and ne (not equal)
//     fn eq(&self, other: &Self) -> bool {
//         self.re == other.re && self.im == other.im
//     }
// }
//Eq relies on partial eq
// impl<T: Eq> Eq for Complex<T> where T: Eq {}

pub fn operator_overloading() {
    let a = Complex::new(1, 2);
    let b = Complex::new(1, 2);

    // println!("{:?}]", a + b); //"+" cannot be applied for type complex but after implementing Add trait, it can be possible

    // a += b;
    // println!("{:?}", a);

    //print!("{:?}", -a);
    print!("{:?}", a == b);
}

//////////////////////////////////////////////////////////////
// Static Dispatch !
//////////////////////////////////////////////////////////////

trait Printable {
    fn format(&self) -> String;
}

//Defining the Trait of i32 and String

impl Printable for i32 {
    //format macro
    fn format(&self) -> String {
        format!("i32: {}", *self) //this is the return statement so we dont need semi-colon here
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

//static dispatch --> The decision of dispatching the format either i32 or String is actually happens on compile time!
//at compile time, compiler knows this function fn print_it(z:String)
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format())
    //monomorphisation
}

pub fn static_dispatch() {
    let a = 123;
    let b = "hellow maham!".to_string();
    // println!("{}", a.format());
    // println!("{}", b.format());
    print_it(a);
    print_it(b);
}

//////////////////////////////////////////////////////////////
// Dynamic Dispatch !
//////////////////////////////////////////////////////////////

//run time execution, required heavy computation
fn print_it_dynamically(z: &dyn Printable) {
    println!("{}", z.format()) //z type tells about the format type either i32 or String
                               //monomorphisation
}

pub fn dynamic_dispatch() {
    let a = 123;
    let b = "hellow maham!".to_string();

    print_it_dynamically(&a);
    print_it_dynamically(&b);
}

// another example of dynamic dispatch

// struct Circle {
//     radius: f64,
// }
// struct Square {
//     side: f64,
// }

// trait Shape {
//     fn area(&self) -> f64;
// }

// impl Shape for Square {
//     fn area(&self) -> f64 {
//         self.side * self.side
//     }
// }

// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         self.radius * self.radius * std::f64::consts::PI
//     }
// }

pub fn another_dynamic_dispatch() {
    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.0 },
        &Circle { radius: 2.0 },
        &Square { side: 4.0 },
    ];
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area()); //dynamic dispatch
    }
}

/////////////////////////////////////////////////////////////
// Vectors of Different Objects !
//////////////////////////////////////////////////////////////
enum Creaturee {
    Human(Human), //Human uses the Human struct
    Cat(Cat),
}
trait Animall {
    //below 2 are the instance func
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk!", self.name());
    }
}
struct Humann {
    name: &'static str,
}

struct Catt {
    name: &'static str,
}

impl Animall for Humann {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} say hello", self.name())
    }
}

impl Animall for Catt {
    fn name(&self) -> &'static str {
        self.name //name getting from Cat struct
    }
    fn talk(&self) {
        println!("{} say meow", self.name())
    }
}

pub fn vectors_with_different_objects() {
    let mut creatures = Vec::new();
    // creatures.push(Human { name: "Maham" });
    // creatures.push(Cat { name: "Fluffy" });
    creatures.push(Creaturee::Human(Human { name: "Maham" }));
    creatures.push(Creaturee::Cat(Cat { name: "Fluffy" }));

    for c in creatures {
        match c {
            Creaturee::Human(h) => h.talk(),
            Creaturee::Cat(c) => c.talk(),
        }
    }

    //vectors with traits

    let mut animals: Vec<Box<dyn Animall>> = Vec::new(); //when we add the Box, then it's size will be known
    animals.push(Box::new(Humann { name: "Maham" }));
    animals.push(Box::new(Catt { name: "Fluffy" }));

    for a in animals.iter() {
        //iter -->unpacking the whole thing
        a.talk();
    }
}
