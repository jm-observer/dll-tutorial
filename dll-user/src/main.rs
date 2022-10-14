use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api {
    adds: fn(a1: u8, a2: u16, a3: u32) -> usize,
    pub_adds: fn(a1: u8, a2: u16, a3: u32) -> usize,
    // 这2种写法都可以
    // pub_adds_v2: unsafe extern "C" fn(a1: u8, a2: u16, a3: u32) -> usize,
    // pub_adds_v2: extern "C" fn(a1: u8, a2: u16, a3: u32) -> usize,
    pub_adds_v2: fn(a1: u8, a2: u16, a3: u32) -> usize,
}

fn main() {
    let cont: Container<Api> = unsafe { Container::load("./dll-user/rust_dll_demo.dll") }
        .expect("Could not open library or load symbols");

    println!("{}", cont.adds(1, 2, 3));
    println!("{}", cont.pub_adds(1, 2, 3));
    println!("{}", cont.pub_adds_v2(1, 2, 3));
    // unsafe {
    //     println!("{}", cont.pub_adds_v2(1, 2, 3));
    // }
}
