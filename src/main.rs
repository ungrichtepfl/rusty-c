include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings_cpp.rs"));

fn main() {
    let mut my_array: [i32; 9] = [3, 8, 2, 5, 1, 4, 7, 6, 9];

    println!("Let's sort this array in C: {:?}", my_array);
    let len = my_array.len() as i32;

    unsafe {
        quicksort(my_array.as_mut_ptr(), len);
    }

    println!("The C function gave us the sorted array: {:?}", my_array);

    unsafe {
        reverse();
    }
}
