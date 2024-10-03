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
}
