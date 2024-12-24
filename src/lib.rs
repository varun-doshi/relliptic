use std::{fmt, ops::{Add, Sub}};

use modc::modc_math::math::Field;
use primitive_types::U256;
pub mod utils;

//Weierstrass curves
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: Option<U256>,
    pub y: Option<U256>,
    pub a: U256,
    pub b: U256,
    pub r: Field,
}

impl Point {
    pub fn new<T>(x: T, y: T, a: T, b: T, r: T) -> Result<Self, utils::error::RellipticError>
    where
        T: Into<U256> + Copy,
    {
        let f = Field::new(r.into()).unwrap();
        let lhs = f.pow(y.into(), U256::from(2)).unwrap();
        let x_cubed = f.pow(x.into(), U256::from(3));
        let ax = f.mult(x.into(), a.into());
        let rhs = f
            .add(f.add(x_cubed.unwrap(), ax.unwrap()).unwrap(), b.into())
            .unwrap();

        if lhs != rhs {
            return Err(utils::error::RellipticError::PointNotOnCurve);
        } else {
            Ok(Self {
                x: Some(x.into()),
                y: Some(y.into()),
                a: a.into(),
                b: b.into(),
                r: Field::new(r.into()).unwrap(),
            })
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            x: self.x,
            y: Some(self.r.add_inv(self.y.unwrap()).unwrap()),
            a: self.a,
            b: self.b,
            r: self.r.clone(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point(x: {}, y: {}, a: {}, b: {}, r: {:?})",
            self.x
                .map(|v| v.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.y
                .map(|v| v.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.a,
            self.b,
            self.r,
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        println!("{}",self);
        println!("{}",rhs);
        if self.a != rhs.a || self.b != rhs.b {
            panic!("{:?}", utils::error::RellipticError::PointsNotOnSameCurve);
        }
        if self.r != rhs.r {
            panic!("{:?}", utils::error::RellipticError::FieldNotSame);
        }

        if self.x == rhs.x && self.y != rhs.y {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
                r: self.r,
            };
        } else {
            let y1y2 = self.r.sub(rhs.y.unwrap(), self.y.unwrap()).unwrap();
            let x1x2 = self.r.sub(rhs.x.unwrap(), self.x.unwrap()).unwrap();
            let slope = self.r.mult(y1y2, self.r.mult_inv(x1x2).unwrap()).unwrap();

            let x3 = self
                .r
                .sub(
                    self.r
                        .sub(self.r.pow(slope, 2.into()).unwrap(), self.x.unwrap())
                        .unwrap(),
                    rhs.x.unwrap(),
                )
                .unwrap();

            let y3 = self
                .r
                .sub(
                    self.r
                        .mult(slope, self.r.sub(self.x.unwrap(), x3).unwrap())
                        .unwrap(),
                    self.y.unwrap(),
                )
                .unwrap();

            return Self {
                x: Some(self.r.self_mod(x3)),
                y: Some(self.r.self_mod(y3)),
                a: self.a,
                b: self.b,
                r: self.r,
            };
        }
    }
}


impl Sub for Point {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}