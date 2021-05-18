fn main() {
    println!("Hello, world!");



    // Reference Safety

    // Learning to see how Rust keeps its references under control. We'll start with the simplest examples and proceed to more complicated ones.



    // Borrowing a Local Variable

    // First example, we can't borrow a reference to a local variable and take it out of the variable's scope:
    {
        let r;
        {
            let x = 1;
            r = &x;
        }

        assert_eq!(*r, 1); // bad: reads memory 'x' used to occupy
    }

    // The Rust compiler rejects the above program stating:
    // error: 'x' does not live long enough
    // references_dangling.rs...

    // Rust's complaint is that x lives only until the end of the inner block, whereas the reference remains alive until the end of the outer block, making it a dangling pointer, which is verboten.

    // Rust tries to assign each reference type in our program a lifetime that meets the constraints imposed by how it is used. A lifetime is some stretch of your program for which a reference could be safe to use. A lexical block, a statement, an expression, the scop of some variable, etc. Lifetimes are entirely figments of Rust's compile-time imagination. At runtime, a reference is nothing but an address. Its lifetime is part of its type and has no runtime representation.

    // See page 168 for diagram

    // If we have a variable x, then a reference to x must not outlive x itself. Beyond the point where x goes out of scope, the reference would be a dangling pointer. We say that the variable's lifetime must contain or enclose that of the reference borrowed from it.

    // see page 169 for diagrams

    // Essentially, r is a reference to x, x goes out of scope once the inner function has reached the end, releasing it. Because its released, r holding the reference also releases. It's why the assert_eq! throws an error, as *r (remember that * is used to follow a reference) doesn't exist as the reference r was released.

    // We can fix the above by doing:
    {
        let x = 1;
        {
            let r = &x;
            assert_eq!(*r, 1);
        }
    }

    // The above rules apply in a natural way when we borrow a ref to some part of some larger data structure like an element of a vector:
    let v = vec![1, 2, 3];
    let  4 = &v[1];

    // Since v owns the vector, which owns its elements, the lifetime of v must enclose that of the reference type of &v[1]. Similarly, if we store a ref in some data structure, its lifetime must enclose that of the data structure. If we build a vector of references, say, all of them must have lifetimes enclsoing that of the variable that owns the vector.



    // Receiving References as Parameters


    // When we pass a ref to a function, how does Rust make sure the function uses it safely? Suppose we have a function f that takes a ref and stores it in a global variable.
    // The below code has problems and won't compile
    // static mut STASH: &i32;
    // fn f(p: &i32) { STASH = p; }

    // Rust's equivalent of a global variable is called a static. It's a value that's created when the program starts and lasts util it terminates. Like all things Rust, statics are only global in their lifetime, not their visibility. Statics are covered in chapter 8 in detail however a few rules that the code above doesn't follow:
    // 1. Every static must be initialized
    // 2. Mutable statics are inherently not thread-safe (any thread can access a static at any time), and even in single-threaded programs, they can fall prey to other sorts of re-entrancy problems. For these reasons, we may access a mutable static only within an unsafe block. Because this is an example, we update the above with an unsafe block and move on.

    // With the revisions made, we now have the following:
    static mut STASH: &i32 = &128;
    fn f(p: &i32) { // still not good enough
        unsafe {
            STASH = p;
        }
    }

    // The above still has a problem that we need to correct. The signature of f as written above is actually shorthand for the following:
    // fn f<'a>(p: &'a i32) {}

    // Here, the lifetime 'a (pronounced "tick A") is a lifetime parameter of f. We can read <'a> as "for any lifetime 'a" so when we write fn f<'a>(p...), we're defining a function that takes a ref to an i32 with any given lifetime 'a.

    // Since we must allow 'a to be any lifetime, things had better work out if it's the smallest possible lifetime. One just enclosing the call to f. This following assignment then becomes a point of contention:
    // STASH = p;

    // Since STASH lives for the program's entire execution, the reference type it holds must have a lifetime of the same length. Rust calls this the 'static lifetime. But the lifetime of p's reference is some 'a, which could be anything, as long as it encloses the call to f. So Rust rejects our code saying:
    // lifetime of reference outlives lifetime of borrowed content
    // note: ... the reference is valid for the static lifetime...
    // note: ... but the borrowed content is only valid for the anon lifetime #1 defined ont he function body at ...

    // It's clear that our function can't accept just any reference as an argument. But it should be able to accept a reference that has a 'static lifetime. Storing such a reference in STASH can't create a dangling pointer. As such, the following code compiles:
    static mut STASH: &i32 = &10;

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }

    // This time, f's signature spells out that p must be a reference with a lifetime 'static, so there's no longer any problem storing that in STASH. We can only apply f to references to other statics, but that's the only thing that's certain not to leave STASH dangling anyway. We can write:
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // Since WORTH... is a static, the type of &WORTH... is &'static i32, which is safe to pass to f.

    // Taking a step back, if we look at what happened to f's signature as the code was amended, the original f(p: &i32) ended up as f(p: &'static i32). In other words, we were unable to write a function that stashed a reference in a global variable without reflecting that intention in the function's signature. In Rust, a function's signature always exposes the body's behaviour.

    // Conversely, if we do see a function with a signature life g(p: &i32) (or with the lifetimes written out, g<'a>(p: &'a i32)), we can tell that it 'does not stash' its argument p anywhere that will outlive the call. There's no need to look into g's definition. The signature alone tells use what g can and can't do with its argument. This fact ends up being very useful when you're trying to establish the safety of a call to the function.






}
