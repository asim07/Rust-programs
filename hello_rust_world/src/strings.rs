pub fn strings_and_characters() {
    // a slice that always points to a valid UTF-8 sequence
    // a view into a String
    let s: &'static str = "hi there!"; // &'static str -->
                                       // statically allocated (part of the program)

    // s = "bar"; // cannot reassign immutable

    //let a = s[0]; // cannot index

    for c in s.chars().rev()
    // reversed! also as_bytes()
    {
        println!("{}", c);
    }
    // when you try to access something from a sequence, you get an option
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // heap allocated construct is String
    // Vec<u8>, guaranteed to be valid UTF-8

    let mut letters = String::new(); //making new object of type string
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char); //it's have push function because it's basically have vectors
        letters.push_str(","); // note the _str that takes a string slice
        a = a + 1;
    }

    println!("{}", letters);
    // APIs
    // &str <> String
    // str from String, conversion from String to str
    let u: &str = &letters; // deref conversion
                            // there are situations when the coercion does NOT happen
    println!("{:?}", u);
    // concatenation
    // String + str
    let z = letters + "abc"; //we have a sequence of letter and we want to add abc to it
                             // String + &String
                             //let mut abc = String::from("hello world!"); //make a string from a string slice

    // String from str
    //let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}, {}", abc, abc.replace("ello", "goodbye"));

    // format macro!

    let name = "Maham";
    let greeting = format!("hi my name is {}", name);
    println!("{}", greeting);

    let onee = "1";
    let two = "2";
    let one_two = format!("{0}, {1}, {0}", onee, two);
    println!("{}", one_two);

    //another way to specifying an argument that you want to use
    let info = format!(
        "the name's {last}. {first} {last}",
        first = "maham",
        last = "hafeez"
    );
    println!("{}", info);

    let mixed = format!("{1}, {},{},{0}, {data}", "alpha", "beta", data = "delta");
    println!("{}", mixed);
}
