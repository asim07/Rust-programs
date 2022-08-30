//lifetime and memory

// ownership!
pub fn ownership() {
    //&vector is just pointing to the data
    let v = vec![1, 2, 3]; //v owns the memory
    let v2 = v; //its copy the pointer and rust guarantee that only one variable will points to the data,
                //So the the v is borrowed or moved and no longer usable, Now we can access the data using v2
                //even if you try to use it in closure, it's no longer useable,
                //let foo = |v: Vec<i32>| ();
                //foo(v); //warning comes of moved value v
    println!("{:?}", v2);

    //but it works for primitive e.g.
    //allocate i32 bits in memory
    //full copy
    let u = 1;
    let u2 = u;
    println!("{}", u); //working fine

    // let u = Box::new(1); //i32 copy
    // let u2 = u; //u2 basically takes the ownership of the variable
    // println!("{}", u); //move warning comes again

    //borrowing
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        //first i take the ownership of x vector
        println!("{:?}", x);
        x //return the ownership
    };

    let vv = print_vector(v2);
    println!("{}", vv[0]);
}

// borrowing!

pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
        //x.push(3); //not works
    };
    let v = vec![3, 2, 1];
    print_vector(&v); //borrowing
    println!("v[0] = {}", v[0]); //accessible after borrowing

    let mut a = 40;
    let c = &a;
    {
        let b = &mut a; //a is borrowed at this point and cannot be usable until we release it or add in scope
        *b += 2;
    }
    println!("a = {}", a)
}

// lifetime!

struct Person {
    name: String,
}

struct Company<'z> {
    name: String,
    ceo: &'z Person, //missing lifetime specifier warning because rust not allows the invalid specifier, now you have to write <'lifetime_name>
                     //after adding guarantee like 'z, we tells the compiler that lifetime of the company and person are same
}

impl Person {
    //fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        //lifetime illusion
        &self.name
    }
}

pub fn lifetime() {
    //In &'static str --> static is the lifetime that means, it will live until the programs live
    // let boss = Person {
    //     name: String::from("Elon Musk"),
    // };
    // let tesla = Company {
    //     name: String::from("Tesla"),
    //     ceo: &boss,
    // };

    let z: &String;
    {
        let p = Person {
            name: String::from("Elon Musk"),
        };
        z = p.get_ref_name();
        println!("{}", z)
    }
}

// lifetime in structure implementation!

struct Course<'a> {
    name: &'a str, //name life will until the person exist
}
impl<'b> Course<'b> {
    fn talk(&self) {
        println!("Hi!, I'm learning the {} course!", self.name);
    }
}

pub fn lifetime_with_struct() {
    let course = Course { name: "Rust" };
    course.talk();
}

// reference counted variables!

use std::rc::Rc;
use std::thread;

struct Country {
    name: Rc<String>,
}

impl Country {
    fn new(name: Rc<String>) -> Country {
        Country { name }
    }

    fn greetings(&self) {
        println!("Salam {}!", self.name)
    }
}

pub fn reference_counted_var() {
    let country = Rc::new("Pakistan".to_string()); //1 strong pointer
    println!(
        "country = {}, country has {} strong pointers!",
        country,
        Rc::strong_count(&country)
    );
    //so to share name on different locations on your code --> use reference counted variable
    {
        let pak = Country::new(country.clone()); //this increment the reference count,
                                                 //above one + this one  = 2 pointers
        pak.greetings();

        println!(
            "country = {}, country has {} strong pointers!",
            country,
            Rc::strong_count(&country)
        );
    } //pak variable will be destroyed at this point and reference count decreases here because of adding scope i.e. {}
    println!(
        "country = {}, country has {} strong pointers!",
        country,
        Rc::strong_count(&country) //first one strong point left yet
    );

    //now to figure out about how many locations keeps the pointer in our code
    //1) i will make the scope arount pak through {}
}

// atomic reference counted variables!
// there is need of Mutex for modification
use std::sync::{Arc, Mutex};
struct Countryy {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Countryy {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Countryy {
        Countryy { name, state }
    }

    fn greetings(&self) {
        //Mutex prevents the modification for other threads
        //lock the mutex and unwrap the result
        let mut state = self.state.lock().unwrap();
        //now can perform my manipulations
        state.clear();
        state.push_str("cool!");
        println!("Salam {}!, a {} country", self.name, state.as_str());
    }
}

pub fn atomic_reference_counted_var() {
    //if you want to pass a variable not only a thread but also in many threads then you can use arc
    let country = Arc::new("Pakistan".to_string()); //1 strong pointer
    let state = Arc::new(Mutex::new("strong".to_string())); //1 strong pointer
    let pak = Countryy::new(country.clone(), state.clone());

    let t = thread::spawn(move || {
        pak.greetings(); //passing object in thread causes an centrate issue that's why need to implement arc
    });
    println!(
        "Country Name = {}, State = {}",
        country,
        state.lock().unwrap()
    ); //this is the main thread and the other will be the another thread
       //waiting thread to finish
    t.join().unwrap();
}
