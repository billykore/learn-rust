use std::rc::Rc;

/// # Chapter 4. Ownership and Moves
///
/// Programming Rust
///
/// Fast, Safe System Development
///
/// Jim Blandy, Jason Orendorff & Leonora F.S. Tindall

/// # Ownership
///
/// In Rust, however, the concept of ownership is built into the
/// language itself and enforced by compile-time checks.
/// Every value has a single owner that determines its lifetime.
/// When the owner is freed—`dropped`, in Rust terminology—the
/// owned value is dropped too. These rules are meant to make
/// it easy for you to find any given value’s lifetime simply by
/// inspecting the code, giving you the control over its lifetime
/// that a systems language should provide.
///
/// A variable owns its value. When control leaves the block in
/// which the variable is declared, the variable is dropped, so
/// its value is dropped along with it. For example:
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
    // dropped here
}

#[test]
fn test_print_padovan() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    print_padovan()
}

struct Person {
    name: String,
    birth: i32,
}

#[test]
fn test_composer() {
    let mut composers = Vec::new();

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

/// # Moves
///
/// In Rust, for most types, operations like assigning a value to a variable,
/// passing it to a function, or returning it from a function
/// don’t copy the value: they `move` it.
/// The source relinquishes ownership of the value to the destination and
/// becomes uninitialized; the destination now controls the value’s lifetime.
/// Rust programs build up and tear down complex structures one value at a time,
/// one move at a time.
#[test]
fn test_moves() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s; // vector s moved here
    // let u = s; // error because s already moved to t

    let t = s.clone();
    let u = s.clone();

    println!("{:?}", t);
    println!("{:?}", u);

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here
    println!("{}", s);

    let mut s = "Govinda".to_string();
    let t = s;
    println!("{}", t);
    s = "Siddhartha".to_string(); // nothing is dropped here
    println!("{}", s);
}

/// ## Moves and Control Flow
///
/// The previous examples all have very simple control flow;
/// how do moves interact with more complicated code?
/// The general principle is that, if it’s possible for a variable to
/// have had its value moved away, and it hasn’t definitely been
/// given a new value since, it’s considered uninitialized.
/// ```
/// let x = vec![10, 20, 30];
/// if c {
///     f(x); // ... ok to move from x here
/// } else {
///     g(x); // ... and ok to also move from x here
/// }
/// h(x);
/// ```
///
/// For similar reasons, moving from a variable in a loop is forbidden:
///
/// ```
/// let x = vec![10, 20, 30];
/// while f() {
///     g(x); // bad: x would be moved in first iteration, uninitialized in second
/// }
/// ```
///
/// That is, unless we’ve definitely given it a new value by the next iteration:
///
/// ```
/// let mut x = vec![10, 20, 30];
/// while f() {
///     g(x); // move from x
///     x = h(); // give x a fresh value
/// }
/// e(x);
/// ```
#[test]
fn test_moves_and_control_flow() {
    //
}

/// ## Moves and Indexed Content
///
/// We’ve mentioned that a move leaves its source uninitialized,
/// as the destination takes ownership of the value.
/// But not every kind of value owner is prepared to become uninitialized.
#[test]
fn test_moves_and_indexed_content() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // Pull out random elements from the vector.
    // let third = v[2]; // error: Cannot move out of index of Vec
    // let fifth = v[4]; // here too

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

/// # Copy Types: The Exception to Moves
///
/// The examples we’ve shown so far of values being moved involve vectors, strings,
/// and other types that could potentially use a lot of memory and be expensive to copy.
///
/// Moves keep ownership of such types clear and assignment cheap.
/// But for simpler types like integers or characters,
/// this sort of careful handling really isn’t necessary.
#[test]
fn test_the_exception_to_moves() {
    // Assigning a String moves the value,
    // whereas assigning an i32 copies it

    let string1 = "somnambulance".to_string();
    let string2 = string1;
    // println!("{}", string1); // error: Value used after being moved
    println!("{}", string2);

    let num1: i32 = 36;
    let num2 = num1;
    println!("{}", num1);
    println!("{}", num2);
}

/// What about types you define yourself?
/// By default, struct and enum types are not Copy:
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

#[test]
fn test_print() {
    let l = Label { number: 3 };
    print(l);
    // println!("My label number is: {}", l.number); // error: borrow of moved value: `l`
}

/// If all the fields of your struct are themselves Copy,
/// then you can make the type Copy as well by placing the attribute
/// `#[derive(Copy, Clone)]` above the definition, like so:
#[derive(Copy, Clone)]
struct LabelCopy {
    number: u32,
}

fn print2(l: LabelCopy) {
    println!("STAMP: {}", l.number);
}

#[test]
fn test_print2() {
    let l = LabelCopy { number: 3 };
    print2(l);
    println!("My label number is: {}", l.number); // no error
}

/// # Rc and Arc: Shared Ownership
///
/// Although most values have unique owners in typical Rust code,
/// in some cases it’s difficult to find every value a single owner
/// that has the lifetime you need; you’d like the value
/// to simply live until everyone’s done using it.
/// For these cases, Rust provides the reference-counted pointer types Rc and Arc.
#[test]
fn test_rc_arc() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // A value owned by an Rc pointer is immutable. Suppose you
    // try to add some text to the end of the string:
    // s.push_str(" noodles"); // error: cannot borrow data in an `Rc` as mutable
}
