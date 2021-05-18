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


}
