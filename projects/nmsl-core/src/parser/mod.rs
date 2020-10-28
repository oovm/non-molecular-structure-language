use crate::Expression;
use crate::particle::Particle;

pub fn parse(input: &str) -> Vec<Expression> {
    let mut out = vec![];
    for line in input.lines() {
        match parse_line(line) {
            None => (),
            Some(s) => out.extend(s),
        }
    }
    return out;
}

pub fn parse_line(input: &str) -> Option<Vec<Expression>> {
    assert_eq!(input.lines().count(), 1);
    None
}

pub fn parse_formula(input: &str)->Vec<Particle> {

}

#[test]
fn test() {
    println!("{:?}", parse_formula("C2H5OH"))
}
