extern crate primitive_types;
#[cfg(test)]
extern crate relliptic;
use crate::relliptic::utils::error::RellipticError;
use primitive_types::U256;
use relliptic::Point;

#[test]
fn test_new_point_valid() {
    let result = Point::new(3, 2, 1, 0, 13);
    assert_eq!(result.unwrap().x.unwrap(), U256::from(3));
}
#[test]
fn test_new_point_invalid() {
    let result = Point::new(3, 2, 1, 0, 11);
    assert!(result.is_err());
}

#[test]
fn test_point_addition() {
    let p1 = Point::new(3, 2, 1, 0, 13).unwrap();
    let p2 = Point::new(4, 4, 1, 0, 13).unwrap();
    let p3 = p1 + p2;
    assert_eq!(p3.x.unwrap(), U256::from(10));
    assert_eq!(p3.y.unwrap(), U256::from(10));
}
#[test]
fn test_point_negation() {
    let p = Point::new(3, 2, 1, 0, 13).unwrap();
    let pn = p.neg();
    assert_eq!(pn.x, p.x);
    assert_eq!(pn.y.unwrap(), U256::from(11));
}

#[test]
fn test_point_subtraction() {
    let p1 = Point::new(4, 4, 1, 0, 13).unwrap();
    let p2 = Point::new(3, 2, 1, 0, 13).unwrap();
    let p3 = p1 - p2;
    assert_eq!(p3.x.unwrap(), U256::from(3));
    assert_eq!(p3.y.unwrap(), U256::from(2));
}
