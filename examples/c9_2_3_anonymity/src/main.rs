// `F` must be generic.
fn apply_once<F>(f: F) where
    F: FnOnce() {
    f();
}


fn apply_mut<F>(mut f1: F) where
    F: Fn() {
    f1();
}


fn main() {
    let mut x = 7;
    let func = || {
        println!("{}", x);
        x += 1; //func is FnMut
    };
    apply_once(func); 
    println!("x={}", x);

    let func_once = move || {
        println!("{}", x);
        x += 1;
    };
    apply_mut(func_once);
    println!("{}", x);
}
