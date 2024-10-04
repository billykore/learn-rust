use regex::Regex;

/// # Chapter 3. Fundamental Types
///
/// Programming Rust
///
/// Fast, Safe System Development
///
/// Jim Blandy, Jason Orendorff & Leonora F.S. Tindall

/// # Rust's type inference.
///
/// Given a function below, with explicit type.
///
/// ```
/// fn build_vector() -> Vec<i16> {
///     let mut v: Vec<i16> = Vec::<i16>::new();
///     v.push(10i16);
///     v.push(20i16);
///     v
/// }
/// ```
///
/// Rust’s type inference applies, allowing you to instead write:
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

#[test]
fn test_build_vector() {
    let v1 = build_vector();
    let v2 = build_vector();
    assert_eq!(v1, v2);
}

/// # Fixed-Width Numeric Types
///
/// # Integer Type
/// Checked, Wrapping, Saturating, and Overflowing Arithmetic
///
/// 1. Checked operations return an Option of the result:
/// Some(v) if the mathematically correct result can be
/// represented as a value of that type, or None if it
/// cannot.
#[test]
fn test_checked_op() {
    // The sum of 10 and 20 can be represented as an u8.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None);

    // Oddly, signed division can overflow too, in one particular case.
    assert_eq!((-128_i8).checked_div(-1), None);
}

/// 2. Wrapping operations return the value equivalent to
/// the mathematically correct result modulo the range
/// of the value:
#[test]
fn test_wrapping_op() {
    // The first product can be represented as an u16;
    // the second cannot, so we get 250000 modulo 2¹⁶.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);
}

/// 3. Saturating operations return the representable
/// value that is closest to the mathematically correct
/// result. In other words, the result is “clamped” to the
/// maximum and minimum values the type can
/// represent:
#[test]
fn test_saturating_op() {
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

/// 4. Overflowing operations return a tuple (result, overflowed),
/// where result is what the wrapping version of the function would return,
/// and overflowed is a bool indicating whether an overflow occurred:
#[test]
fn test_overflowing_op() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}

/// # Floating-Point Types
///
/// Rust provides IEEE single- and double-precision floating-point types.
#[test]
fn test_floating_point_types() {
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(f32::MIN, -f32::MAX);
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);
    println!("{}", (2.0_f64).sqrt()); // with suffix
    println!("{}", f64::sqrt(2.0));
}

/// # The `bool` Type
///
/// Rust’s Boolean type, bool, has the usual two values for such types, true and false.
#[test]
fn test_bool_type() {
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
    // if x != 0 { ... } not equals if x { ... }
}

/// # Characters
///
/// Rust’s character type `char` represents a single Unicode character, as a 32-bit value.
///
/// Rust uses the char type for single characters in isolation,
/// but uses the UTF-8 encoding for strings and streams of
/// text. So, a String represents its text as a sequence of UTF-8 bytes,
/// not as an array of characters.
#[test]
fn test_char_type() {
    assert_eq!('*' as i32, 42);
    println!("{}", 0xca0);
    println!("{}", -0x60); // U+0CA0 truncated to eight bits,signed

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

/// # Tuples
///
/// A tuple is a pair, or triple, quadruple, quintuple, etc.
/// (hence, n-tuple, or tuple), of values of assorted types.
#[test]
fn test_tuple_type() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let unit = ();
    println!("{:?}", unit);
}

/// # Pointer Types
///
/// Rust has several types that represent memory addresses.
///
/// ## References
///
/// A value of type `&String` (pronounced “ref String”) is a reference to a `String` value,
/// an `&i32` is a reference to an `i32`, and so on.
#[test]
fn test_reference_type() {
    let x = 0;
    let y = &x; // reference
    let z = &x;
    assert_eq!(*y, *z);

    let mut r = 1;
    let s = &mut r;
    // let t = &mut r; // cannot borrow second time.
    assert_eq!(*s, 1);
}

/// # Boxes
///
/// The simplest way to allocate a value in the heap is to use `Box::new:`
#[test]
fn test_boxes_type() {
    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);
}

/// # Raw Pointers
///
/// Rust also has the raw pointer types `*mut T` and `*const T`.
/// Raw pointers really are just like pointers in C++. Using a
/// raw pointer is unsafe, because Rust makes no effort to
/// track what it points to.
/// However, you may only dereference raw pointers within an unsafe block.
/// An unsafe block is Rust’s opt-in mechanism for advanced language features
/// whose safety is up to you.
#[test]
fn test_raw_pointer() {

}

/// # Arrays, Vectors, and Slices
///
/// Rust has three types for representing a sequence of values in memory:
///
/// 1. The type `[T; N]` represents an array of `N` values, each of type `T`.
/// An array’s size is a constant determined at compile time and is part of the type;
/// you can’t append new elements or shrink an array.
///
/// 2. The type `Vec<T>`, called a vector of `Ts`, is a dynamically allocated,
/// growable sequence of values of type `T`.
/// A vector’s elements live on the heap, so you can resize vectors at will:
/// push new elements onto them, append other vectors to them,
/// delete elements, and so on.
///
/// 3. The types `&[T]` and `&mut [T]`, called a shared slice of `Ts` and mutable slice of `Ts`,
/// are references to a series of elements that are a part of some other value,
/// like an array or vector. You can think of a slice as a pointer to its first element,
/// together with a count of the number of elements you can access starting at that point.
/// A mutable slice `&mut [T]` lets you read and modify elements, but can’t be shared;
/// a shared slice `&[T]` lets you share access among several readers,
/// but doesn’t let you modify elements.
///
/// # Arrays
/// There are several ways to write array values.
/// The simplest is to write a series of values within square brackets:
#[test]
fn test_arrays() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000]; // [V; N] form
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let kb = [0u8; 1024];
    assert_eq!(kb.len(), 1024);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

/// # Vectors
///
/// A vector `Vec<T>` is a resizable array of elements of type `T`,
/// allocated on the heap.
#[test]
fn test_vector_vec_macro() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

#[test]
fn test_new_pixel_buffer() {
    let buffer = new_pixel_buffer(100, 100);
    assert_eq!(buffer.len(), 10000);
}

#[test]
fn test_vector_vec_new() {
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);
}

#[test]
fn test_vector_create_from_iterator() {
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
}

#[test]
fn test_vector_slice_method() {
    // A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    // Reasonable yet disappointing:
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
}

#[test]
fn test_vector_len_and_capacity() {
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    // Typically prints "capacity is now 4":
    println!("capacity is now {}", v.capacity());
}

#[test]
fn test_vector_insert_and_remove_element() {
    let mut v = vec![10, 20, 30, 40, 50];
    // Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}

#[test]
fn test_vector_pop() {
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);
}

/// # Slices
///
/// A slice, written `[T]` without specifying the length, is a region of an array or vector.
/// Since a slice can be any length, slices can’t be stored directly in variables or passed
/// as function arguments. Slices are always passed by reference.
#[test]
fn test_slices() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    println!("{:?}", sv);
    println!("{:?}", sa);
}

/// # String Types
///
/// ## String Literals
/// Are enclosed in double quotes.
/// They use the same backslash escape sequences as char literals:
#[test]
fn test_string_literals() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    println!("In the room the women come and go,
        Singing of Mount Abora");

    println!("It was a bright, cold day in April, and \
        there were four of us—\
        more or less.");
}

#[test]
fn test_raw_string() {
    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*").expect("Invalid regular expression");
    println!("{}", pattern.is_match(default_win_install_path));

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);
}

/// ## Byte Strings
///
/// A string literal with the b prefix is a byte string.
/// Such a string is a slice of u8 values—that is, bytes—rather than Unicode text:
#[test]
fn test_byte_strings() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}

/// ## Strings in Memory
///
/// Rust strings are sequences of Unicode characters,
/// but they are not stored in memory as arrays of chars.
/// Instead, they are stored using UTF-8, a variable-width encoding.
/// Each ASCII character in a string is stored in one byte.
/// Other characters take up multiple bytes.
#[test]
fn test_strings_in_memory() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];

    println!("{}", noodles);
    println!("{}", oodles);
}

/// ## `String`
///
/// `&str` is very much like `&[T]`: a fat pointer to some data.
/// `String` is analogous to `Vec<T>`.
///
/// There are several ways to create `String`s:
///
/// 1. The `.to_string()` method converts a `&str` to a `String`. This copies the string:
#[test]
fn test_to_string() {
    let error_message = "too many pets".to_string();
    println!("{}", error_message);
}

/// 2. The `format!()` macro works just like `println!()`,
/// except that it returns a new `String` instead of writing text to stdout,
/// and it doesn’t automatically add a newline at the end:
#[test]
fn test_format_macro() {
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());
}

/// 3. Arrays, slices, and vectors of strings have two methods, 
/// `.concat()` and .`join(sep)`, that form a new `String` from many strings:
#[test]
fn test_concat_and_join() {
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
}

/// ## Using Strings
///
/// Strings support the == and != operators.
/// Strings also support the comparison operators <, <=, >, and >=.
/// Two strings are equal if they contain the same characters in the same order (regardless of
/// whether they point to the same location in memory):
#[test]
fn test_using_strings() {
    assert_eq!("ONE".to_lowercase(), "one");
    assert!("peanut".contains("nut"));
    assert_eq!("_".replace("", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}

/// ## Other String-Like Types
///
/// Rust guarantees that strings are valid UTF-8.
/// Sometimes a program really needs to be able to deal with strings that are not valid Unicode.
/// This usually happens when a Rust program has to interoperate with some other system that
/// doesn’t enforce any such rules.
///
/// Rust’s solution is to offer a few string-like types for these situations:
///
/// 1. Stick to `String` and `&str` for Unicode text.
///
/// 2. When working with filenames, use `std::path::PathBuf` and `&Path` instead.
///
/// 3. When working with binary data that isn’t UTF-8 encoded at all, use `Vec<u8>` and `&[u8]`.
///
/// 4. When working with environment variable names and command-line arguments in the native form
/// presented by the operating system, use `OsString` and `&OsStr`.
///
/// 5. When interoperating with C libraries that use null-terminated strings,
/// use `std::ffi::CString` and `&CStr`.
#[test]
fn test_other_string_like_types() {

}

/// # Type Aliases
///
/// The type keyword can be used like typedef in C++ to declare a new name for an existing type:
type Bytes = Vec<u8>;

/// The type `Bytes` that we’re declaring here is shorthand for this particular kind of `Vec`:
fn decode(bytes: &Bytes) {
    println!("{:?}", bytes);
}

#[test]
fn test_decode() {
    decode(&Vec::new())
}

/// # Beyond the Basics
///
/// Types are a central part of Rust.
/// We’ll continue talking about types and introducing new ones throughout the book.
/// In particular, Rust’s user-defined types give the language much of its flavor,
/// because that’s where methods are defined.
#[test]
fn test_beyond_the_basics() {

}
