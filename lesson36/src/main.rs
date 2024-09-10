use std::slice;

fn main() {
    let mut num = 5;

    // * IMMUTABLE RAW POINTER
    // * can't be directly assigned after it's been dereferenced
    // * alllowed to ignore Rust's borrowing rules by allowing

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // let address = 0x012345usize;
    // let r3 = address as *const i32;

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("The abvsolute value of -3 according to C: {}", abs(-3));
    }

    println!("greeting is: {}", HELLO_WORLD);

    add_to_count(4);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

extern "C" {
    fn abs(value: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(value: u32) {
    unsafe {
        COUNTER += value;
    }
}

pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }
