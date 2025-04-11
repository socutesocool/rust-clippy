#![allow(unused)]
#![warn(clippy::foo_functions)]

// Impl methods
struct A;
impl A {
    pub fn fo(&self) {}        // 不触发
    pub fn foo(&self) {}       // 触发 lint
    //~^ foo_functions
    pub fn food(&self) {}      // 触发 lint
    //~^ foo_functions
}

// Default trait methods
trait B {
    fn fo(&self) {}            // 不触发
    fn foo(&self) {}           // 触发 lint
    //~^ foo_functions
    fn food(&self) {}          // 触发 lint
    //~^ foo_functions
}

// Plain functions
fn fo() {}                     // 不触发
fn foo() {}                    // 触发 lint
//~^ foo_functions
fn food() {}                   // 触发 lint
//~^ foo_functions

fn main() {
    // 方法调用不触发
    foo();
    let a = A;
    a.foo();
}