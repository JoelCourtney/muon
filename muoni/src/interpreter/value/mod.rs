use ast::{RValue,LValue,Statement};
use self::{Value::*,Number::*};
use na::*;
use nc::{Complex,Complex64};
use ni::{BigInt,ToBigInt};
use num_traits::{ToPrimitive,Zero};

#[derive(Debug,PartialEq)]
pub struct Unit {
    n: i64,
    d: i64,
}

#[derive(Debug,PartialEq)]
pub enum Number {
    RI(i64),
    RB(BigInt),
    RF(f64),
    RM(DMatrix<f64>),
    CI(Complex<i64>),
    CB(Complex<BigInt>),
    CF(Complex<f64>),
    CM(DMatrix<Complex64>),
    RG(f64,f64,f64),
}

#[derive(Debug,PartialEq)]
pub enum Value {
    S(String),
    N(Number),
    U(Number,Unit),
    B(bool),
    L(Vec<Value>),
    Z(Box<RValue>),
    F(Vec<LValue>,Vec<LValue>,Box<Statement>),
}

impl<'a> From<&'a RValue> for Value {
    fn from(rv: &'a RValue) -> Self {
        match rv {
            RValue::Number(f) if f % 1. == 0. => {
                N(RI(*f as i64))
            }
            RValue::Number(f) => {
                N(RF(*f))
            }
            RValue::StringLiteral(s) => {
                S(s.clone())
            }
            RValue::Bool(b) => {
                B(*b)
            }
            RValue::AnonFunction(a,c,b) => {
                F(a.clone(),c.clone(),b.clone())
            }
            _ => panic!("i wasn't ready yet"),
        }
    }
}

pub fn add(e1: &Value, e2: &Value) -> Value {
    why();
    match (e1,e2) {
        (N(n1),N(n2)) => {
            N(match (n1,n2) {
                (RB(n1),RB(n2)) => RB(n1+n2),
                (RI(n1),RI(n2)) => {
                    let n3 = n1.checked_add(*n2);
                    match n3 {
                        Some(n3) => RI(n3),
                        None => RB(n1.to_bigint().unwrap() + n2),
                    }
                }
                (RF(n1),RF(n2)) => RF(n1+n2),
                (RM(m1),RM(m2)) => RM(m1+m2),
                (CI(c1),CI(c2)) => CI(c1+c2),
                (CF(c1),CF(c2)) => CF(c1+c2),
                (CB(c1),CB(c2)) => CB(c1+c2),
                (CM(m1),CM(m2)) => CM(m1+m2),

                (RI(n1),RF(n2)) | (RF(n2),RI(n1)) => RF(*n1 as f64 + n2),
                (RI(n1),RM(m2)) | (RM(m2),RI(n1)) => RM(m2.add_scalar(*n1 as f64)),
                (RF(n1),RM(m2)) | (RM(m2),RF(n1)) => RM(m2.add_scalar(*n1)),
                
                (CI(c1),RI(n2)) | (RI(n2),CI(c1)) => CI(c1 + Complex::from(*n2)),
                (CI(c1),RF(n2)) | (RF(n2),CI(c1)) => CF(Complex64::new(c1.re as f64+n2,c1.im as f64)),
                (CI(c1),CF(c2)) | (CF(c2),CI(c1)) => CF(ci_to_cf(c1) + c2),
                (CF(c1),RI(n2)) | (RI(n2),CF(c1)) => CF(c1+Complex64::from(*n2 as f64)),
                (CF(c1),RF(n2)) | (RF(n2),CF(c1)) => CF(c1+Complex64::from(*n2)),

                (RB(n1),RF(n2)) | (RF(n2),RB(n1)) => RF(n1.to_f64().unwrap()+n2),
                (RB(n1),RI(n2)) | (RI(n2),RB(n1)) => RB(n1+n2),
                (RB(n1),RM(m2)) | (RM(m2),RB(n1)) => RM(m2.add_scalar(n1.to_f64().unwrap())),
                (RB(n1),CI(c2)) | (CI(c2),RB(n1)) => CB(Complex::from(n1)+ci_to_cb(c2)),
                (RI(n1),CB(c2)) | (CB(c2),RI(n1)) => CB(c2 + Complex::from(n1.to_bigint().unwrap())),
                (RI(n1),CM(m2)) | (CM(m2),RI(n1)) => CM(m2.add_scalar(Complex64::from(*n1 as f64))),
                (RB(n1),CB(c2)) | (CB(c2),RB(n1)) => CB(c2 + Complex::from(n1)),
                (RB(n1),CF(c2)) | (CF(c2),RB(n1)) => CF(c2+Complex::from(n1.to_f64().unwrap())),
                (RB(n1),CM(m2)) | (CM(m2),RB(n1)) => CM(m2.add_scalar(Complex::from(n1.to_f64().unwrap()))),
                (RF(n1),CB(c2)) | (CB(c2),RF(n1)) => CF(Complex::from(n1)+cb_to_cf(c2)),
                (RF(n1),CM(m2)) | (CM(m2),RF(n1)) => CM(m2.add_scalar(Complex::from(n1))),

                (RM(m1),CI(c2)) | (CI(c2),RM(m1)) => CM(rm_to_cm(m1).add_scalar(ci_to_cf(c2))),
                (RM(m1),CB(c2)) | (CB(c2),RM(m1)) => CM(rm_to_cm(m1).add_scalar(cb_to_cf(c2))),
                (RM(m1),CF(c2)) | (CF(c2),RM(m1)) => CM(rm_to_cm(m1).add_scalar(*c2)),
                (RM(m1),CM(m2)) | (CM(m2),RM(m1)) => CM(rm_to_cm(m1) + m2),
                (CI(c1),CB(c2)) | (CB(c2),CI(c1)) => CB(c2 + ci_to_cb(c1)),
                (CI(c1),CM(m2)) | (CM(m2),CI(c1)) => CM(m2.add_scalar(ci_to_cf(c1))),
                (CB(c1),CF(c2)) | (CF(c2),CB(c1)) => CF(c2 + cb_to_cf(c1)),
                (CB(c1),CM(m2)) | (CM(m2),CB(c1)) => CM(m2.add_scalar(cb_to_cf(c1))),
                (CF(c1),CM(m2)) | (CM(m2),CF(c1)) => CM(m2.add_scalar(*c1)),

                (RG(_,_,_),_) | (_,RG(_,_,_)) => unimplemented!(),
            })
        }

        _ => unimplemented!(),
    }
}

enum Precision {
    int,
    float,
    big,
    none,
}

struct ValType {
    list: bool,
    string: bool,
    complex: bool,
    matrix: (bool,usize),
    precision: Precision,
    boolean: bool,
}

trait Calc {
    fn type_of(&self) -> ValType;

    fn add_mat_real(&self, f64) -> DMatrix<f64>;
    fn add_mat_comp(&self, Complex64) -> DMatrix<Complex64>;

    fn to_ri(&self) -> i64;
    fn to_rf(&self) -> f64;
    fn to_rb(&self) -> BigInt;
    fn to_rm(&self) -> DMatrix<f64>;
    fn to_ci(&self) -> Complex<i64>;
    fn to_cf(&self) -> Complex64;
    fn to_cb(&self) -> Complex<BigInt>;
    fn to_cm(&self) -> DMatrix<Complex64>;
    fn to_str(&self) -> String;
    fn to_bool(&self) -> bool;
}

impl Calc for i64 {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: false,
            matrix: (false,0),
            precision: Precision::int,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on ri");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on ri");
    }

    fn to_ri(&self) -> i64 {
        *self
    }
    fn to_rf(&self) -> f64 {
        *self as f64
    }
    fn to_rb(&self) -> BigInt {
        self.to_bigint().unwrap()
    }
    fn to_rm(&self) -> DMatrix<f64> {
        DMatrix::from_element(1,1,*self as f64)
    }
    fn to_ci(&self) -> Complex<i64> {
        Complex::from(self)
    }
    fn to_cf(&self) -> Complex64 {
        Complex::from(*self as f64)
    }
    fn to_cb(&self) -> Complex<BigInt> {
        Complex::from(self.to_bigint().unwrap())
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,Complex::from(*self as f64))
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
        *self != 0
    }
}

impl Calc for f64 {
    fn type_of(&self) -> ValType {
        ValType{
            list: false,
            string: false,
            complex: false,
            matrix: (false,0),
            precision: Precision::float,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on rf");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on rf");
    }

    fn to_ri(&self) -> i64 {
        *self as i64
    }
    fn to_rf(&self) -> f64 {
        *self
    }
    fn to_rb(&self) -> BigInt {
        (*self as i64).to_bigint().unwrap()
    }
    fn to_rm(&self) -> DMatrix<f64> {
        DMatrix::from_element(1,1,*self)
    }
    fn to_ci(&self) -> Complex<i64> {
        Complex::from(*self as i64)
    }
    fn to_cf(&self) -> Complex64 {
        Complex::from(*self)
    }
    fn to_cb(&self) -> Complex<BigInt> {
        Complex::from((*self as i64).to_bigint().unwrap())
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,Complex::from(*self))
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
        *self != 0.
    }
}

impl Calc for BigInt {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: false,
            matrix: (false, 0),
            precision: Precision::big,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on rb");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on rb");
    }


    fn to_ri(&self) -> i64 {
        self.to_i64().unwrap()
    }
    fn to_rf(&self) -> f64 {
        self.to_f64().unwrap()
    }
    fn to_rb(&self) -> BigInt {
        self.clone()
    }
    fn to_rm(&self) -> DMatrix<f64> {
        DMatrix::from_element(1,1,self.to_f64().unwrap())
    }
    fn to_ci(&self) -> Complex<i64> {
        Complex::from(self.to_i64().unwrap())
    }
    fn to_cf(&self) -> Complex64 {
        Complex::from(self.to_f64().unwrap())
    }
    fn to_cb(&self) -> Complex<BigInt> {
        Complex::from(self.clone())
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,Complex::from(self.to_f64().unwrap()))
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
         !self.is_zero()
    }
}

impl Calc for DMatrix<f64> {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: false,
            matrix: (true,self.len()),
            precision: Precision::float,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        self.add_scalar(o)
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        self.to_cm().add_scalar(o)
    }

    fn to_ri(&self) -> i64 {
        if self.len() == 1 {
            *self.data.get(0).unwrap() as i64
        } else {
            panic!("cannot convert to ri")
        }
    }
    fn to_rf(&self) -> f64 {
        if self.len() == 1 {
            *self.data.get(0).unwrap()
        } else {
            panic!("cannot convert to rf")
        }
    }
    fn to_rb(&self) -> BigInt {
        if self.len() == 1 {
            self.data.get(0).unwrap().to_bigint().unwrap()
        } else {
            panic!("cannot convert to rb")
        }
    }
    fn to_rm(&self) -> DMatrix<f64> {
        self.clone()
    }
    fn to_ci(&self) -> Complex<i64> {
        if self.len() == 1 {
            Complex::from(*self.data.get(0).unwrap() as i64)
        } else {
            panic!("cannot convert to ci")
        }
    }
    fn to_cf(&self) -> Complex64 {
        if self.len() == 1 {
            Complex::from(self.data.get(0).unwrap())
        } else {
            panic!("cannot convert to cf")
        }
    }
    fn to_cb(&self) -> Complex<BigInt> {
        if self.len() == 1 {
            Complex::from(self.data.get(0).unwrap().to_bigint().unwrap())
        } else {
            panic!("cannot convert to cb")
        }
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        self.map(|n| Complex::from(n))
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
         !self.data.contains(&0.)
    }
}

impl Calc for Complex<i64> {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: true,
            matrix: (false,0),
            precision: Precision::int,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on ci");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on ci");
    }

    fn to_ri(&self) -> i64 {
        if self.im == 0 {
            self.re
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rf(&self) -> f64 {
        if self.im == 0 {
            self.re as f64
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rb(&self) -> BigInt {
        if self.im == 0 {
            self.re.to_bigint().unwrap()
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rm(&self) -> DMatrix<f64> {
        if self.im == 0 {
            DMatrix::from_element(1,1,self.re as f64)
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_ci(&self) -> Complex<i64> {
        *self
    }
    fn to_cf(&self) -> Complex64 {
        Complex::new(self.re as f64, self.im as f64)
    }
    fn to_cb(&self) -> Complex<BigInt> {
        Complex::new(self.re.to_bigint().unwrap(), self.im.to_bigint().unwrap())
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,self.to_cf())
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
        self.re != 0 || self.im != 0
    }
}

impl Calc for Complex64 {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: true,
            matrix: (false,0),
            precision: Precision::float,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on cf");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on cf");
    }

    fn to_ri(&self) -> i64 {
        if self.im == 0. {
            self.re as i64
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rf(&self) -> f64 {
        if self.im == 0. {
            self.re
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rb(&self) -> BigInt {
        if self.im == 0. {
            self.re.to_bigint().unwrap()
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rm(&self) -> DMatrix<f64> {
        if self.im == 0. {
            DMatrix::from_element(1,1,self.re)
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_ci(&self) -> Complex<i64> {
        Complex::new(self.re as i64, self.im as i64)
    }
    fn to_cf(&self) -> Complex64 {
        *self
    }
    fn to_cb(&self) -> Complex<BigInt> {
        Complex::new(self.re.to_bigint().unwrap(), self.im.to_bigint().unwrap())
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,*self)
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
        self.re != 0. || self.im != 0.
    }
}

impl Calc for Complex<BigInt> {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: true,
            matrix: (false, 0),
            precision: Precision::big,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("unable to do matrix add on cb");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("unable to do matrix add on cb");
    }


    fn to_ri(&self) -> i64 {
        if self.im.is_zero() {
            self.re.to_i64().unwrap()
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rf(&self) -> f64 {
        if self.im.is_zero() {
            self.re.to_f64().unwrap()
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rb(&self) -> BigInt {
        if self.im.is_zero() {
            self.re.clone()
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_rm(&self) -> DMatrix<f64> {
        if self.im.is_zero() {
            DMatrix::from_element(1,1,self.re.to_f64().unwrap())
        } else {
            panic!("cannot truncate non-zero imaginary component.");
        }
    }
    fn to_ci(&self) -> Complex<i64> {
        Complex::new(self.re.to_i64().unwrap(),self.im.to_i64().unwrap())
    }
    fn to_cf(&self) -> Complex64 {
        Complex::new(self.re.to_f64().unwrap(),self.im.to_f64().unwrap())
    }
    fn to_cb(&self) -> Complex<BigInt> {
        self.clone()
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        DMatrix::from_element(1,1,self.to_cf())
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
         !self.is_zero()
    }
}

impl Calc for DMatrix<Complex64> {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: false,
            complex: true,
            matrix: (true,self.len()),
            precision: Precision::float,
            boolean: false,
        }
    }

    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        self.to_rm().add_scalar(o)
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        self.add_scalar(o)
    }

    fn to_ri(&self) -> i64 {
        if self.len() == 1 {
            let d = self.data.get(0).unwrap();
            if d.im == 0. {
                d.re as i64
            } else {
                panic!("cannot truncate non-zero imaginary component.");
            }
        } else {
            panic!("cannot convert to ri")
        }
    }
    fn to_rf(&self) -> f64 {
        if self.len() == 1 {
            let d = self.data.get(0).unwrap();
            if d.im == 0. {
                d.re
            } else {
                panic!("cannot truncate non-zero imaginary component.");
            }
        } else {
            panic!("cannot convert to rf")
        }
    }
    fn to_rb(&self) -> BigInt {
        if self.len() == 1 {
            let d = self.data.get(0).unwrap();
            if d.im == 0. {
                d.re.to_bigint().unwrap()
            } else {
                panic!("cannot truncate non-zero imaginary component.");
            }
        } else {
            panic!("cannot convert to rb")
        }
    }
    fn to_rm(&self) -> DMatrix<f64> {
        self.map(|c| {
            if c.im == 0. {
                c.re
            } else {
                panic!("cannot truncate non-zero imaginary component.");
            }
        })
    }
    fn to_ci(&self) -> Complex<i64> {
        if self.len() == 1 {
            self.data.get(0).unwrap().to_ci()
        } else {
            panic!("cannot convert to ci")
        }
    }
    fn to_cf(&self) -> Complex64 {
        if self.len() == 1 {
            *self.data.get(0).unwrap()
        } else {
            panic!("cannot convert to cf")
        }
    }
    fn to_cb(&self) -> Complex<BigInt> {
        if self.len() == 1 {
            self.data.get(0).unwrap().to_cb()
        } else {
            panic!("cannot convert to cb")
        }
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        self.clone()
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn to_bool(&self) -> bool {
        !self.data.contains(&Complex64::zero())
    }
}

impl Calc for String {
    fn type_of(&self) -> ValType {
        ValType {
            list: false,
            string: true,
            complex: false,
            matrix: (false,0),
            precision: Precision::none,
            boolean: false,
        }
    }
    
    fn add_mat_real(&self, o: f64) -> DMatrix<f64> {
        panic!("realy");
    }
    fn add_mat_comp(&self, o: Complex64) -> DMatrix<Complex64> {
        panic!("nah man")
    }

    fn to_ri(&self) -> i64 {
        self.parse::<i64>().unwrap()
    }
    fn to_rf(&self) -> f64 {
        self.parse::<f64>().unwrap()
    }
    fn to_rb(&self) -> BigInt {
        BigInt::parse_bytes(self.as_bytes(), 10).unwrap()
    }
    fn to_rm(&self) -> DMatrix<f64> {
        panic!("no can do")
    }
    fn to_ci(&self) -> Complex<i64> {
        panic!("not till we got an interpreter")
    }
    fn to_cf(&self) -> Complex64 {
        panic!("not till we got an interpreter")
    }
    fn to_cb(&self) -> Complex<BigInt> {
        panic!("not till we got an interpreter")
    }
    fn to_cm(&self) -> DMatrix<Complex64> {
        panic!("not till we got an interpreter")
    }
    fn to_str(&self) -> String {
        self.clone()
    }
    fn to_bool(&self) -> bool {
        self.len() != 0
    }
}


fn ci_to_cf(c: &Complex<i64>) -> Complex64 {
    Complex::new(c.re as f64, c.im as f64)
}

fn cb_to_cf(c: &Complex<BigInt>) -> Complex64 {
    Complex::new(c.re.to_f64().unwrap(), c.im.to_f64().unwrap())
}

fn ci_to_cb(c: &Complex<i64>) -> Complex<BigInt> {
    Complex::new(c.re.to_bigint().unwrap(), c.im.to_bigint().unwrap())
}

fn rm_to_cm(m: &DMatrix<f64>) -> DMatrix<Complex64> {
    m.map(|n| Complex::from(n))
}

#[derive(Debug,Copy,Clone)]
struct V<T>(T);

impl<T> V<T> {
    fn unwrap(self) -> T {
        self.0
    }
    fn new<T1: Calc+'static>(n: T1) -> V<Box<Calc>> {
        let result: Box<Calc> = Box::new(n);
        V(result)
    }
}

use std::ops::Add;
impl Add<V<Box<Calc>>> for V<Box<Calc>> {
    type Output = V<Box<Calc>>;

    fn add(self, other: V<Box<Calc>>) -> Self::Output {
        let v1 = self.unwrap();
        let v2 = other.unwrap();
        let t1 = v1.type_of();
        let t2 = v2.type_of();
        if t1.list {
            unimplemented!();
        }
        if t1.string || t2.string {
            let result: Box<Calc> = Box::new(v1.to_str() + &v2.to_str());
            return V(result);
        }

        unimplemented!()
    }
}

use std::collections::HashMap;

type RI = V<i64>;
type RF = V<f64>;
type RB = V<BigInt>;
type RM = V<DMatrix<f64>>;
type CI = V<Complex<i64>>;
type CF = V<Complex64>;
type CM = V<DMatrix<Complex64>>;

fn why() {
    let mut x = RI::new(5);
    let y = RF::new(5);
    let z = x+y;
    let a = RI::new(5) + RF::new(3) + RB::new(400.to_rb());
    let mut m = HashMap::new();
    m.insert("z".to_string(),z.unwrap());
    println!("{:?}",m);
    let m = DMatrix::from_element(2,2,4);
}

use std::fmt::{Debug,Formatter,Display,Result};

impl Debug for Calc {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",self.to_str())
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",5)
    }
}