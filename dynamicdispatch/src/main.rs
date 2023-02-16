
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!{"{}", *self}
    }
}

impl Foo for String {
    fn method(&self) -> String {
        self.to_string()
    }
}

fn trait_bound_static_dispatch<X: Foo>(member: X) -> String {
    member.method()
}

pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

struct FooVtable {
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}


fn call_method_on_u8(x: *const ()) -> String {
    // the compiler guarantees that this function is only called
    // with `x` pointing to a u8
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    size: 1,
    align: 1,
    // cast to a function pointer
    method: call_method_on_u8 as fn(*const()) -> String,
};

// String:

fn call_method_on_String(x: *const ()) -> String {
    // the compiler guarantees that this function is only called
    // with `x` pointing to a String
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    // values for a 64-bit computer, halve them for 32-bit ones
    size: 24,
    align: 8,
    method: call_method_on_String as fn(*const()) -> String,
};

use std::mem;

fn main() {
    let a: String = "foo".to_string();
    let x: u8 = 1;

    // let b: &Foo = &a;
    let b = TraitObject {
        // store the data
        data: &a as *const _ as *mut (),
        // store the methods
        vtable: &Foo_for_String_vtable as *const _ as *mut ()
    };

    // let y: &Foo = x;
    let y = Box::new(TraitObject {
        // store the data
        data: &x as *const _ as *mut (),
        // store the methods
        vtable: &Foo_for_u8_vtable as *const _ as *mut ()
    });

    type StringFunctionAlias = fn(*const()) -> String;

    // b.method();
    //(b.vtable as &Foo_for_String_vtable).method(b.data)
    //unsafe{<(*b.vtable) as Into<T>>::into(Foo_for_String_vtable).method(b.data)};

    // y.method();
    //unsafe{(*y.vtable).into().method(y.data)};

    // b.method();

/*     unsafe {
        let b = mem::transmute::<*mut (), StringFunctionAlias>(*b.vtable);
        (b)(b.method);
    } */

/*     // y.method();
    (y.vtable.method)(y.data); */
}
