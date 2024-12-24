#[derive(Debug, PartialEq)]
pub enum RellipticError{
    PointNotOnCurve,
    PointsNotOnSameCurve,
    FieldNotSame
}