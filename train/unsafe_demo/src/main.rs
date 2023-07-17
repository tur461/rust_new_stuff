fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);

    unsafe {
        println!("*r1: {:?}", *r1);
        *r2 = 100;
        println!("*r2: {:?}", *r2);
        //*r1 = 110; this reuslt into compile err
    }
}
