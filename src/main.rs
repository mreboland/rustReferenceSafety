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



    // Passing References as Arguments

    // Now that we've shown how a function's signature relates to its body, let's examine how it relates to the function's callers:

    // This could be written more briefly: fn g(p: &i32),
    // but we'll write out the lifetimes for now.
    fn g<'a>(p: &'a i32) { ... }

    let x = 10;
    g(&x);

    // From g's signature alone, Rust knows it will not save p anywhere that might outlive the call. Any lifetime that encloses the call must work for 'a. So Rust chooses the smallest possible lifetime for &x, that of the call to g. This meets all constrains. It doesn't outlive x, and encloses the entire call to g so the code passes muster.

    // Note that although g takes a lifetime parameter 'a, we didn't need to mention it when calling g. We only need to worry about lifetime parameters when defining functions and types. When using them, Rust infers the lifetimes for us.

    // What if we tried to pass &x to our function f from earlier that stores its argument in a static?
    fn f(p: &'static i32) { ... }

    let x = 10;
    f(&x);

    // The above fails to compile. The reference &x must not outlive x, but by passing it to f, we constrain it to live at least as long as 'static (which outlives all other lifetimes in a Rust program). There's no way to satisfy everyone here, so Rust rejects the code.



    // Returning References

    // It's common for a function to take a reference to some data structure, and then return a reference into some part of that structure. For example, here's a function that returns a reference to the smallest element of a slice:

    // v should have at least one element.
    fn smallest(v: &[i32]) -> i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r };
        }
        
        s
    }

    // We've omitted lifetimes from the function's signature in the usual way. When a function takes a single ref as an argument, and returns a single ref, Rust assumes that the two must have the same lifetime. Writing it out explicitly:
    fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }

    // Suppose we call smallest like this:
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
    }
    assert_eq!(*s, 0); // bad: points to element of dropped array

    // From smallest's signature, we can see that its argument and return value must have the same lifetime, 'a. In our call, the argument &parabola must not outlive parabola itself. Yet smallest's return value must live at least as long as s. There's no possible lifetime 'a that can satisfy both constraints, so Rust reject the code:
    // error: `parabola` does not live long enough...

    // Moving assert_eq into the the inner brackets so that it's contained within parabola's lifetime fixes the issue.

    // Lifetimes in function signatures let Rust asses the relationships between the references we pass to the function and those the function returns, and ensure they're being used safely.



    // Structs Containing References

    // How does Rust handle references stored in data structures? Using the previous error prone example, we've moved the reference inside a structure:

    // This does not compile
    struct S {
        r: &i32
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
    }

    assert_eq!(*s.r, 10); // bad: reads from dropped 'x'

    // The safety constraints Rust places on references can't magically disappear just because we hid the ref inside a struct. Somehow, those constraints must end up applying to S as well. Rust throws an error:
    // Missing lifetime specifier...
    // expected lifetime parameter...

    // Whenever a ref type appears inside another type's definition, we must write out its lifetime:
    struct S {
        r: &'static i32
    }
    // The above says that r can only refer to i32 values that will last for the lifetime of the program, which is rather limiting. The alternative is to give the type a lifetime parameter 'a, and use that for r:
    struct S<'a> {
        r: &'a i32
    }
    // Now the S type has a lifetime, just as ref types do. Each value we create of type S gets a fresh lifetime 'a, which becomes constrained by how we use the value. The lifetime of any ref we store in r had better enclose 'a, and 'a must outlast the lifetime of wherever we store the S.

    // Looking back at the previous code, the expression S { r: &x } creates a fresh S value with some lifetime 'a. When we store &x in the r field, we constrain 'a to lie entirely within x's lifetime.

    // The assignment s = S { ... } stores this S in a variable whose lifetime extends to the end of the example, constraining 'a to outlast the lifetime of s. Rust has now arrived at the same contradictory constraints as before. 'a must not outlive x, yet must live at least as long as s. No satisfactory lifetime exists, and Rust rejects the code.

    // How does a type with a lifetime parameter behave when placed inside some other type?
    struct T {
        s: S // not adequate
    }
    // Rust errors just as it did when we tried placing a ref in S without specifying its lifetime:
    // missing lifetime specifier...
    // Expected lifetime parameter...

    // We can't leave off S's lifetime parameter here. Rust needs to know how a T's lifetime relates to that of the ref in its S in order to apply the same checks to T that it does for S and plain references.

    // Giving s the 'static lifetime works:
    struct T {
        s: S<'static>
    }
    // The s field may only borrow values that live for the entire execution of the program. This means that a T can't borrow a local variable, there are no special constraints on a T's lifetime.

    // The other approach would be to give T its own lifetime parameter, and pass that to S:
    struct T<'a> {
        s: S<'a>
    }
    // By taking a lifetime parameter 'a and using it in s's type, we've allowed Rust to relate a T value's lifetime to that of the ref its S holds.

    // A type's lifetime parameters always reveal whether it contains references with interesting (that is, non-'static) lifetimes, and what those lifetimes can be.

    // For example, suppose we have a parsing function that takes a slice of bytes, and returns a structure holding the results of the parse:
    fn parse_record<'i>(input: &'i [u9]) -> Record<'i> { ... }
    // Looking at the above, we can tell that if we received a Record from parse_record, whatever refs it contains must point into the input buffer we passed in, and nowhere else (except perhaps at 'static values).

    // This exposure of internal behaviour is the reason Rust requires types that contain refs to take explicit lifetime parameters. There's no reason Rust cound't simply make up a distinct lifetime for each ref in the struct and save us the trouble of writing them out. Early on Rust worked this way but devs fount it confusing. It is helpful to know when one value borrows something from another value, especially when working through errors.

    // Every type in Rust has a lifetime, including i32 and String. Most are simply 'static, meaning that values of those types can live for as long as we'd like. For example, a Vec<i32> is self-contained, and needn't be dropped before any particular variable goes out of scope. But a type like Vec<&'a i32> has a lifetime that must be enclosed by 'a. It must be dropped whiles its referents are still alive.



    






}
