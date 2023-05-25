extern crate srs_poly;

#[test]
fn macros() {
    let sys = srs_poly::system!(
        a^2 - b^2*c^2,
        a + b*c,
        2*a + 3*b^2 + c + 10    
    );

    println!("{:?}", sys.members);
    println!("{:?}", sys.members[0].try_divide(&sys.members[1]));
    println!(
        "{:?}",
        sys.members[0].clone() * sys.constant(5) - sys.members[1].clone() + sys.var("a", 2)
    );
}
