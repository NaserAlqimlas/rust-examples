fn main() {
    println!("{} days", 31);
    println!("base 2: {:b}", 120333);
    println!("{number:0>width$}", number=1, width=5);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
    /* Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of
     * decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an
     * estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the
     * number of decimals to display) */
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}
