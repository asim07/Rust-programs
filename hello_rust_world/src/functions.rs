// functions!

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) // start with i32, it borrowed the element and perform some manipulation
{
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 // return value
{
    let z = x * y;
    z // no semicolons
}

pub fn function_demo() {
    print_value(123);

    let mut z = 1;
    increase(&mut z); // lend z, and make it mut so that the another one can increase it.
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

// closures!

fn say_hello() {
    println!("hello");
}

pub fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    //closure, a func that is defined inline, |x: i32| is the argument, i32 is the return type and {x + 1} is the function body, but it is not

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two; //after it is borrowed by closer then the two is no longer usable So, there is need to add it in scope by writing {}
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    } //As we go out from the scope of two variable, everything is declared locally in this code gets destroyed.

    let borrow_two = &mut two;

    // T: by value //take the entire copy of variable
    // T& //reference
    // &mut & //mutable ref
    //let plus_three = |mut x: i32| x += 3;
    //or
    let pluss_three = |x: &mut i32| *x += 3; //dereferencing the x at the end

    let mut f = 12;
    //plus_three(f);
    pluss_three(&mut f);
    println!("f = {}", f);
}

// higher-order functions!

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

//functions returning functions
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    //-> impl Fn is basically the function traits or signature
    move |y| y > limit //we can extend the lifetime by using move keyword
}

pub fn high_order_functions() {
    // functions that take functions
    // functions that return functions

    // sum of all even squares <= 500

    let limit = 500;
    let mut sum = 0;

    //let above_limit_2 = |y| y > limit;
    let above_limit = greater_than(limit); //generator

    for i in 0.. {
        //0.. means 0 to infinity
        let isq = i * i;

        //if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);
    //feeding functions into the functions

    let sum2 = (0..)
        .map(|x| x * x) //map at |x| this point, takes the value of x and returns the transform of x
        .take_while(|&x| x < limit) // So here this can be done using take while, so take a while will basically only take elements from the sequence while they conform to a particular predicate.
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x); //fold a sequence into a single value by performing pair wise operations, init is 0, because i'm using accumulator here
    println!("hof sum = {}", sum2);
}
