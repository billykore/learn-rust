mod greet;
mod call;
mod sort;
mod search;

fn main() {
    println!("Hello, world")
}

fn sum_array(nums: [i32; 5]) -> i32 {
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    sum
}

#[test]
fn test_sum_array() {
    let nums = [1, 2, 3, 4, 5];
    assert_eq!(sum_array(nums), 15);
}

fn factorial(n: i32) -> i32 {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }
    res
}

#[test]
fn test_factorial() {
    let n = 5;
    assert_eq!(factorial(n), 120);
}

fn fizzbuzz(i: i32) -> String {
    let mut res = String::new();

    if i % 3 == 0 {
        res.push_str("Fizz");
    }
    if i % 5 == 0 {
        res.push_str("Buzz");
    }
    if res.is_empty() {
        res.push_str(&*i.to_string());
    }

    res
}

#[test]
fn test_fizzbuzz() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

#[test]
fn test_shadowing() {
    let my_var = "Billy Kore";
    println!("{}", my_var);

    // my_var get shadowed
    let my_var = 10;
    println!("{}", my_var);
}

#[test]
fn test_comment() {
    // this is single line comment

    /*
    multi
    line
    comment
     */

    println!("Not a comment");
}

#[test]
fn test_tuple() {
    let my_tuple = ("Billy", 25, false);
    let (a, b, c) = my_tuple;
    println!("{} {} {}", a, b, c);
}

fn unit() {
    println!("Hello")
}

#[test]
fn test_unit() {
    let res = unit();
    println!("{:?}", res)
}

#[test]
fn test_array() {
    let array: [char; 5] = ['H', 'e', 'l', 'l', 'o'];
    println!("{:?}", array);
    println!("{}", array.len());
    println!("{} {}", array[0], array[4]);
}

#[test]
fn matrix() {
    let mtx: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4],
    ];
    println!("{:?}", mtx);
}

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    const MAXIMUM: i32 = 100;
    println!("{}", MINIMUM);
    println!("{}", MAXIMUM);
}

#[test]
fn test_if_statement() {
    let a = 0;
    if a < 5 {
        println!("Less than 5");
    } else if a > 5 && a <= 10 {
        println!("More than 5 and less or equal 10");
    } else {
        println!("Too big");
    }
}

#[test]
fn test_loop() {
    let mut i = 0;
    loop {
        if i > 5 { break; }
        println!("i = {}", i);
        i += 1;
    }
}

#[test]
fn while_loop() {
    let mut a = 0;
    while a <= 5 {
        println!("a = {}", a);
        a += 1;
    }
}

#[test]
fn test_for_loop() {
    let array = ["A", "B", "C", "D", "E"];
    for char in array {
        println!("Value: {}", char);
    }
}

#[test]
fn test_for_range_loop() {
    let range = 0..5;
    println!("{}", range.start);
    println!("{}", range.end);

    for v in range {
        println!("{}", v);
    }
}

fn factorial_recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return n * factorial_recursive(n - 1);
}

#[test]
fn test_factorial_recursive() {
    let res = factorial_recursive(5);
    println!("res = {}", res);
    assert_eq!(res, 120);
}

fn print_num(num: i32) {
    println!("Number: {}", num);
}

#[test]
fn test_print_number() {
    let num = 10;
    print_num(num); // num copied to fn print_num.
    println!("{}", num);
}

fn hi(name: String) {
    println!("Hi {}", name);
}

#[test]
fn test_hi() {
    let name = String::from("Oyen");
    hi(name);
    // println!("{}", name); // ERROR! Cannot access name because the ownership is moves to fn hi.
}

fn hi_with_reference(name: &String) {
    println!("Hi {}", name);
}

#[test]
fn test_hi_with_reference() {
    let name = String::from("Oyen");
    hi_with_reference(&name); // name is borrowed by fn hi_with_reference.
    println!("{}", name); // Not error.
}

struct Person {
    first_name: String,
    age: u8,
}

#[test]
fn test_struct() {
    let me = Person {
        first_name: String::from("Oyen"),
        age: 25,
    };
    println!("{}", me.first_name);
    println!("{}", me.age);

    let you = Person { ..me };
    println!("{}", you.first_name);
    println!("{}", you.age);

    let name = you.first_name.clone();
    println!("{}", name);
}

// tuple struct
struct Point(i32, i32);

#[test]
fn test_tuple_struct() {
    let origin = Point(0, 0);
    let my_position = Point(1, 1);

    println!("{} {}", origin.0, origin.1);
    println!("{} {}", my_position.0, my_position.1);
}

struct Employee {
    name: String,
    rule: String,
}

impl Employee {
    // Method
    fn say_hello(&self) {
        println!("Hello! I am {}, My rule is {}.", self.name, self.rule);
    }

    // Associated function
    fn new(name: &str, rule: &str) -> Employee {
        Employee {
            name: String::from(name),
            rule: String::from(rule),
        }
    }
}

#[test]
fn test_struct_method() {
    let manager = Employee {
        name: String::from("Billy"),
        rule: String::from("Manager"),
    };

    manager.say_hello();

    let officer = Employee::new("Oyen", "Officer");
    officer.say_hello();
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let _regular = Level::Regular;
    let _premium = Level::Premium;
    let _platinum = Level::Platinum;
}

#[test]
fn test_pattern_matching() {
    let level = Level::Platinum;

    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

type Age = u8;

struct Student {
    name: String,
    age: Age,
}

#[test]
fn test_type_alias() {
    let student1 = Student {
        name: String::from("Oyen"),
        age: 23,
    };

    println!("{}", student1.name);
    println!("{}", student1.age);
}

mod model {
    pub struct User {
        pub name: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hi(&self, name: &str) {
            println!("Hi {}, I am {}, {} years old", name, self.name, self.age)
        }
    }
}

#[test]
fn test_module() {
    let user = model::User {
        name: String::from("Oyen"),
        age: 24,
    };
    user.say_hi("Billy");
}

use greet::say_good_morning;

#[test]
fn test_use() {
    say_good_morning("Oyen")
}

trait Greeter {
    fn say_hi_to(&self, name: &str) -> String;
}

impl Greeter for Student {
    fn say_hi_to(&self, name: &str) -> String {
        format!("Hi {}! I am {}", self.name, name)
    }
}

#[test]
fn test_trait() {
    let student = Student {
        name: String::from("Oyen"),
        age: 24,
    };

    let greet = student.say_hi_to("Billy");
    println!("{}", greet)
}

fn use_greeter(greeter: &impl Greeter, name: &str) {
    println!("{}", greeter.say_hi_to(name))
}

#[test]
fn test_use_greeter() {
    let student = Student {
        name: String::from("Oyen"),
        age: 24,
    };
    use_greeter(&student, "Ciko")
}

trait Eater {
    fn eat(&self) -> String;
}

impl Eater for Student {
    fn eat(&self) -> String {
        format!("{} eating...", self.name)
    }
}

fn say_hi_and_eat(o: &(impl Greeter + Eater)) {
    println!("{}", o.say_hi_to("there"));
    println!("{}", o.eat())
}

#[test]
fn test_multiple_traits() {
    let student = Student {
        name: String::from("Oyen"),
        age: 25,
    };

    say_hi_and_eat(&student)
}

struct Someone {
    name: String,
}

impl Greeter for Someone {
    fn say_hi_to(&self, name: &str) -> String {
        format!("Hai {}, I am {}", name, self.name)
    }
}

fn new_greeter(name: String) -> impl Greeter {
    Someone { name }
}

#[test]
fn test_return_trait() {
    let greeter = new_greeter(String::from("Oyen"));
    println!("{}", greeter.say_hi_to("Ciko"))
}

impl Eater for Someone {
    fn eat(&self) -> String {
        format!("{} eating...", self.name)
    }
}

trait Human: Greeter + Eater {
    fn alive(&self);
}


impl Human for Someone {
    fn alive(&self) {
        println!("{}", self.say_hi_to("Ciko"));
        println!("{}", self.eat());
    }
}

#[test]
fn test_super_trait() {
    let oyen = Someone {
        name: String::from("Oyen"),
    };

    oyen.alive();
}

// generic struct
struct Position<T> {
    x: T,
    y: T,
}

// generic impl
impl<T> Position<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

// generic trait
trait Getter<T> {
    fn get_values(&self);
}

impl<T: std::fmt::Display> Getter<T> for Position<T> where T: PartialOrd {
    fn get_values(&self) {
        println!("{} {}", self.x, self.y)
    }
}

#[test]
fn test_generic() {
    let pos1 = Position { x: 1, y: 1 };
    let pos2 = Position { x: 1.0, y: 1.0 };

    println!("{} {}", pos1.get_x(), pos1.get_y());
    println!("{} {}", pos2.get_x(), pos2.get_y());

    pos1.get_values();
    pos2.get_values();
}

use core::ops::{Add, Sub};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity
        }
    }
}

impl Sub for Apple {
    type Output = Apple;

    fn sub(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity - rhs.quantity
        }
    }
}

#[test]
fn test_ops_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    let apple3 = apple1.add(apple2);
    println!("{}", apple3.quantity);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

#[test]
fn test_partial_eq() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    println!("{}", apple1.eq(&apple2))
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn partial_ord() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    println!("{}", apple1.gt(&apple2))
}

fn double(val: Option<i32>) -> Option<i32> {
    match val {
        None => None,
        Some(val) => Some(val * 2)
    }
}

#[test]
fn test_option() {
    let some = double(Some(10));
    println!("{}", some.unwrap());

    let none = double(None);
    println!("{:?}", none);
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_formating() {
    let category = Category {
        id: String::from("smartphone"),
        name: String::from("iPhone 15"),
    };

    println!("{:?}", category)
}

#[test]
fn test_closure() {
    let sum = |a: i32, b: i32| -> i32 {
        a + b
    };

    let res = sum(17, 11);
    println!("{}", res)
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let res = filter(value);
    println!("{}", res)
}

#[test]
fn test_closure_as_parameter() {
    print_with_filter(String::from("Oyen"), |val: String| -> String {
        val.to_uppercase()
    })
}

fn upper(s: String) -> String {
    s.to_uppercase()
}

#[test]
fn test_upper() {
    print_with_filter(String::from("oyen"), upper)
}

struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
        println!("Incrementing");
    }
}

#[test]
fn test_increment() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}", counter.count)
}

#[test]
fn test_vec() {
    let mut nums = Vec::<i32>::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);

    println!("{:?}", nums);
}

#[test]
fn test_vec_deque() {
    let mut nums = VecDeque::<i32>::new();
    nums.push_back(1);
    nums.push_back(3);
    nums.push_front(2);

    println!("{:?}", nums);
}

#[test]
fn test_linked_list() {
    let mut nums = LinkedList::<i32>::new();
    nums.push_back(1);
    nums.push_back(2);
    nums.push_back(3);

    println!("{:?}", nums);
}

#[test]
fn test_hash_map() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);

    let one = map.get("one");
    let two = map.get("two");

    println!("{}", one.unwrap());
    println!("{}", two.unwrap());
}

#[test]
fn test_set() {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("Oyen"));
    set.insert(String::from("Oyen"));
    set.insert(String::from("Oyen"));

    for v in set {
        println!("{}", v)
    }
}

#[test]
fn test_iter() {
    let arr = [1, 2, 3, 4, 5];
    let mut iterator = arr.iter();

    while let Some(v) = iterator.next() {
        println!("{}", v)
    }

    println!("{:?}", iterator)
}

#[test]
fn test_iterator_method() {
    let nums = vec![1, 2, 3, 4, 5];

    let sum: i32 = nums.iter().sum();
    println!("{}", sum);

    let count = nums.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = nums.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_to_db(host: Option<&str>) {
    match host {
        None => {
            panic!("Configuration not found!")
        }
        Some(host) => {
            println!("Connected to host: {}", host)
        }
    }
}

#[test]
fn test_panic() {
    connect_to_db(Some("localhost"));
    connect_to_db(None); // will panic!
}

fn divide(a: i32, b: Option<i32>) -> Result<i32, String> {
    match b {
        None => Err(String::from("Divider required")),
        Some(0) => Err(String::from("Dividing by zero")),
        Some(b) => Ok(a / b)
    }
}

#[test]
fn test_result() {
    let none = divide(10, None);
    match none {
        Ok(res) => println!("res = {}", res),
        Err(err) => println!("{}", err)
    }

    let zero_divider = divide(10, Some(0));
    match zero_divider {
        Ok(res) => println!("res = {}", res),
        Err(err) => println!("{}", err)
    }

    let res = divide(10, Some(2));
    match res {
        Ok(res) => println!("res = {}", res),
        Err(err) => println!("{}", err)
    }
}

fn divide_and_add_one(a: i32, b: Option<i32>) -> Result<i32, String> {
    let res = divide(a, b)?;
    Ok(res + 1)
}

#[test]
fn test_divide_and_add_one() {
    let res = divide_and_add_one(4, Some(0));
    match res {
        Ok(a) => println!("{}", a),
        Err(err) => println!("error: {}", err)
    }
}

fn longest<'s>(s1: &'s str, s2: &'s str) -> &'s str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}

#[test]
fn test_lifetime_annotation() {
    let s1 = "Oyen";
    let s2 = "Chiko";
    let res = longest(s1, s2);
    println!("Longest: {}", res)
}

struct Dog<'n> {
    name: &'n str,
    age: i32,
}

impl<'n> Dog<'n> {
    fn get_name(&self) -> &'n str {
        self.name
    }
}

#[test]
fn test_dog() {
    let chiko = Dog {
        name: "Chiko",
        age: 1,
    };

    println!("{}", chiko.name);
    println!("{}", chiko.age);

    println!("{}", chiko.get_name())
}

#[test]
fn test_box() {
    let value: Box<i32> = Box::new(5);
    println!("{}", value)
}

#[test]
fn test_dereference() {
    let a = Box::new(10);
    let b = Box::new(10);
    let result = *a + *b;

    println!("{}", result)
}

struct MyValue<T> {
    value: T
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref_trait() {
    let v = MyValue { value: 10 };
    let x = *v;

    println!("{}", x)
}

fn say_hello(name: &String) {
    println!("Hello, {}!", name)
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: String::from("Oyen")
    };
    say_hello(&name)
}

struct Book {
    title: String
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book {}", self.title)
    }
}

#[test]
fn test_drop() {
    let b = Book {
        title: String::from("Bible")
    };
    println!("{}", b.title)
}

macro_rules! hi {
    () => {
        println!("Hi")
    };

     ($name: expr) => {
        println!("Hi, {}", $name)
    };
}

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i);
        }
    };

    ($($item: expr), *) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_macro() {
    hi!();
    hi!("Oyen");

    iterate!([1, 2, 3, 4, 5]);
    iterate!(1, 2, 3, 4, 5);
}

trait Pedestrian {
    fn talk(&self);
    fn walk(&self);
    fn run(&self);
}

enum PoliceUnit {
    City,
    State,
    Federal,
    FBI,
    SWAT,
}

enum Feel {
    Happy,
    Neutral,
    Sad,
    Angry
}

impl Feel {
    fn express(&self) {
        match self {
            Feel::Happy =>  println!("Happy"),
            Feel::Neutral => println!("Neutral"),
            Feel::Sad => println!("Sad"),
            Feel::Angry => println!("Angry")
        }
    }
}

#[test]
fn test_talk_with_feel() {
    let sad = Feel::Sad;
    sad.express()
}

struct Police {
    health: u64,
    feel: Feel,
    unit: PoliceUnit,
}

impl Pedestrian for Police {
    fn talk(&self) {
        self.feel.express()
    }

    fn walk(&self) {
        println!("Walk");
    }

    fn run(&self) {
        println!("Run");
    }
}

struct Resident {
    health: u64,
    feel: Feel,
}

impl Pedestrian for Resident {
    fn talk(&self) {
        self.feel.express()
    }

    fn walk(&self) {
        println!("Walk");
    }

    fn run(&self) {
        println!("Run");
    }
}

struct Num {
    x: u64
}

impl FromStr for Num {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u64>() {
            Ok(x) => Ok(Num { x }),
            Err(err) => panic!("{}", err)
        }
    }
}

#[test]
fn test_num() {
   let num = Num::from_str("5").unwrap();
    assert_eq!(num.x, 5);
}
