use wasmedge_hostfunctionexample_interface::*;

fn main() {
    set_class_id(123);
    set_class_name("WasmEdge");
    let mut num: u32 = add_student("Alice");
    println!("After adding Alice, num = {:?}", num);
    num = add_student("Fred");
    println!("After adding Fred, num = {:?}", num);
    num = add_student("Terry");
    println!("After adding Terry, num = {:?}", num);
    print();
    println!("--- setting class address ---");
    // Call Custom Host Func 0
    set_class_addr("Hsinchu");
    print();
}
