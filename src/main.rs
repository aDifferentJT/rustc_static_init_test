extern "C" {
    pub fn f();
}

fn main() {
unsafe { f(); }
}
