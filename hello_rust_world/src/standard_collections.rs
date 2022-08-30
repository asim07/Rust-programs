// vectors !

use std::collections::HashMap;
use std::collections::HashSet;

pub fn vectors() {
    let mut a = Vec::new(); //here :: is a factory func
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    //isize, usize , size is actually the size of memory in your system
    let idx: usize = 0; //if i write i32 here then i gives me an error
    println!("a[0] = {:?}", a[idx]);
    a[idx] = 123;
    println!("a = {:?}", a);
    //get function returns an option, by using get we can avoid the exception of out of index
    match a.get(5) {
        Some(x) => println!("a[5] = {}", x),
        None => println!("error, no such element"),
    }

    // iterate
    for x in &a {
        println!("{}", x);
    }

    // adding/removing
    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop(); // can easily yield nothing, in case pop, we can also get Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // explain why this doesn't work
    //let Some(last_value) = a.pop();

    // print the elements in reverse order
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

// hash map !

pub fn hashmaps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    println!("A square has {} sides", shapes["square".into()]);

    shapes.insert("square".into(), 5);

    //print entire hashmap
    for (key, value) in &shapes {
        println!("{} = {}", key, value);
    }

    shapes.entry("circle".into()).or_insert(1); //So shapes.entry would actually allow you to look for a value like circle and if it is not found then insert the value of 1.
    println!("{:?}", shapes);

    //gonna borrowing the entire hashmap as mutable
    {
        //it gives me a reference of a circle entity or value
        let actual = shapes.entry("circle".into()).or_insert(2);
        //now we can change the value by using this ref.
        *actual = 0;
    }

    println!("{:?}", shapes);
}

// hash sets !

pub fn hashsets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("delta");

    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    // let added_vega = greeks.insert("vega");
    if added_vega {
        println!("We added vega! horray!")
    }
    if !greeks.contains("kappa") {
        println!("We don't have kappa");
    }
    let removed = greeks.remove("delta");
    if removed {
        println!("We removed delta")
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset -->every single item is contains in the other set.
    //e.g. a set of numbers from 2 to 8 is obviously contains in a set from 1 to 10
    println!(
        "Is {:?} is a subset of {:?}: {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoints ->no common elements
    println!(
        "Is {:?} is a subset of {:?}: {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );
    println!(
        "Items in either {:?} and {:?} are: {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );

    //difference --> items in the first set but not in the second
    //symmetric_difference --> union - intersection {items in the union but not in the intersection}

    //union (contains both set elements), intersection (only the element that contains in both the sets)
}

// iterators !

pub fn iterators() {
    let mut vec = vec![3, 2, 1];

    // ordinary iteration causes a move
    for x in &vec {
        //the reason of using &ven instead of vec is, after that we cannot able to use the value after this iteration because we moved it!
        println!("{}", *x);
    }

    // iter() = a bunch of immutable references
    //vec.iter() --> acts as above that is &vec

    //  ^^^
    for x in vec.iter() {
        println!("we got {}", x);
        // cannot modify things!
        // x += 1;
    }

    // iter adapter methods
    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    // iter_mut() - mutable references, requires
    //              the vector to be declared mut
    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    // into_iter() - move operation that transforms the collection into a by-value iterator
    //               not the same as ordinary iteration!
    //               useful when you need values but not the collection itself
    // extend() - automatically calls into_iter() to move elements from one collection to another
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec); //after that vec not works
    println!("{:?}", vec2);
}
