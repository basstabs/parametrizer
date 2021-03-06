//! A simple, safe crate for parsing properly-formatted math strings which represent parametric functions into Rust functions that compute them. Ported from an earlier version for Javascript.
//!
//! # Example Usage
//!
//! Simply create a Parametrizer struct by passing a string to one of its constructors and call the evaluate method:
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let division = Parametrizer::new("4\\2").unwrap();
//! let subtraction = Parametrizer::new("15-3*t").unwrap();
//! let spaces = Parametrizer::new("6 + T").unwrap();
//! let sin = Parametrizer::new("sin(t*t + t - 1)").unwrap();
//!
//! assert_eq!(2, division.evaluate(8));
//! assert_eq!(6, subtraction.evaluate(3));
//! assert_eq!(8, spaces.evaluate(2));
//! assert_eq!(11.0_f64.sin(), sin.evaluate(3.0));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let constant = Parametrizer::new("1.35").unwrap();
//!
//! assert_eq!(1.35, constant.evaluate(2.0));
//! assert_eq!(1.35, constant.evaluate(3.4));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let variable = Parametrizer::new("t").unwrap();
//!
//! assert_eq!(3.0, variable.evaluate(3.0));
//! assert_ne!(4.2, variable.evaluate(1.25));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let addition = Parametrizer::new("1+t").unwrap();
//!
//! assert_eq!(9.0, addition.evaluate(8.0));
//! assert_eq!(1.16, addition.evaluate(0.16));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let equation = Parametrizer::new("13+((2*t)+5)").unwrap();
//!
//! assert_eq!(20, equation.evaluate(1));
//! assert_eq!(30, equation.evaluate(6));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let division = Parametrizer::new("6/t").unwrap();
//!
//! assert_eq!(2, division.evaluate(3));
//! assert_eq!(3, division.evaluate(2));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let equation = Parametrizer::new("13-t").unwrap();
//! let negation = Parametrizer::new("-t").unwrap();
//!
//! assert_eq!(10, equation.evaluate(3));
//! assert_eq!(-9, negation.evaluate(9));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! let dynamic_rand = Parametrizer::new("rd(2+t<4*t)").unwrap();
//! let computed_rand = Parametrizer::new("rc(4<8)").unwrap();
//!
//! assert_eq!(computed_rand.evaluate(2), computed_rand.evaluate(4));
//! assert!(4 <= dynamic_rand.evaluate(2));
//! assert!(16 > dynamic_rand.evaluate(4));
//! ```
//!
//! ```
//! use crate::parametrizer::Parametrizer;
//!
//! //Piecewise functions
//! let p1 = Parametrizer::new("p2>0|4>2|8>6").unwrap();
//! let p2 = Parametrizer::new("p2*t>0|4>2").unwrap();
//!
//! //Looping piecewise functions
//! let p3 = Parametrizer::new("p[10]18>0|23>4").unwrap();
//!
//! assert_eq!(2, p1.evaluate(1));
//! assert_eq!(4, p1.evaluate(5));
//! assert_eq!(2, p2.evaluate(1));
//! assert_eq!(4, p2.evaluate(9));
//!
//! assert_eq!(18, p3.evaluate(23));
//! assert_eq!(23, p3.evaluate(106));
//! ```
//!
//! The underlying terms are public to allow for the manual composition of terms in code to avoid
//! the string parsing overhead. See the `term` module documentation for more information. See the
//! `Parametrizer` struct's implementation documentation to see more usage examples.

extern crate num;

use num::Num;
use num::ToPrimitive;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::str::FromStr;
use std::fmt;

pub mod term;

pub trait Number: Num + ToPrimitive + FromPrimitive + PartialOrd + FromStr + Copy + Send + Sync + 'static {}
impl<T: Num + ToPrimitive + FromPrimitive + PartialOrd + FromStr + Copy + Send + Sync + 'static> Number for T {}

///An error which describes why parametrization failed. Contains the param string which failed as
///well as the reason for failure.
#[derive(Debug)]
pub struct ParametrizerError
{

    param: String,
    reason: &'static str

}

impl fmt::Display for ParametrizerError
{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {

        return write!(f, "Parametrizer failed to parse string: {}, with failure reason: {}", self.param, self.reason);

    }

}

///A pair containing a function on 64-bit float numbers and a shorthand associated with it.
pub struct ParametrizerFunction
{

    shorthand: String,
    function: fn(f64) -> f64

}

impl ParametrizerFunction
{

    ///Function for creating a ParametrizerFunction pair for use in Parametrizer
    ///
    /// # Examples
    /// 
    /// ```
    /// use crate::parametrizer::ParametrizerFunction;
    ///
    /// let pair = ParametrizerFunction::new("Sin".to_string(), f64::sin);
    ///
    /// assert_eq!("sin(", pair.shorthand());
    /// assert_eq!(2.0_f64.sin(), (pair.function())(2.0));
    /// ```
    pub fn new(identifier: String, function: fn(f64) -> f64) -> ParametrizerFunction
    {

        let shorthand = identifier.to_lowercase();
        let shorthand = format!("{}(", shorthand);

        return ParametrizerFunction { shorthand, function };

    }

    ///A function which returns the shorthand for the function as parsed by parametrize_string,
    ///i.e. adds a "(" to the end of the user-defined identifier.
    pub fn shorthand(&self) -> &String
    {

        return &self.shorthand;

    }

    ///Returns the stored function
    pub fn function(&self) -> fn(f64) -> f64
    {

        return self.function;

    }

}

///Main struct for parametrizing strings. Contains a pointer to the top-level term, which will
///contain pointers to lower leves for recursive evaluations
pub struct Parametrizer<T: Number>
{

    //The top-level term for the parametrized function. Must be placed on the heap as the
    //recursion could be of theoretically unbounded depth
    term: Box<dyn term::Term<T> + Send + Sync>

}

impl<T: Number> Parametrizer<T>
{

    ///Default constructor. Formats the param string before parsing to handle uppercase letters, spaces,
    ///and the like, which may cause some performance slowdown. Already properly formatted strings can
    ///be parsed using Parametrizer::quick_new.Supports sine and cosine via "sin" and "cos".
    ///
    /// # Examples
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let division = Parametrizer::new("4\\2").unwrap();
    /// let subtraction = Parametrizer::new("15-3*t").unwrap();
    /// let spaces = Parametrizer::new("6 + T").unwrap();
    /// let sin = Parametrizer::new("sin(t*t + t - 1)").unwrap();
    ///
    /// assert_eq!(2, division.evaluate(8));
    /// assert_eq!(6, subtraction.evaluate(3));
    /// assert_eq!(8, spaces.evaluate(2));
    /// assert_eq!(11.0_f64.sin(), sin.evaluate(3.0));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let constant = Parametrizer::new("1.35").unwrap();
    ///
    /// assert_eq!(1.35, constant.evaluate(2.0));
    /// assert_eq!(1.35, constant.evaluate(3.4));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let variable = Parametrizer::new("t").unwrap();
    ///
    /// assert_eq!(3.0, variable.evaluate(3.0));
    /// assert_ne!(4.2, variable.evaluate(1.25));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let addition = Parametrizer::new("1+t").unwrap();
    ///
    /// assert_eq!(9.0, addition.evaluate(8.0));
    /// assert_eq!(1.16, addition.evaluate(0.16));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let equation = Parametrizer::new("13+((2*t)+5)").unwrap();
    ///
    /// assert_eq!(20, equation.evaluate(1));
    /// assert_eq!(30, equation.evaluate(6));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let division = Parametrizer::new("6/t").unwrap();
    ///
    /// assert_eq!(2, division.evaluate(3));
    /// assert_eq!(3, division.evaluate(2));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let equation = Parametrizer::new("13-t").unwrap();
    /// let negation = Parametrizer::new("-t").unwrap();
    ///
    /// assert_eq!(10, equation.evaluate(3));
    /// assert_eq!(-9, negation.evaluate(9));
    /// ```
    ///
    /// ```
    /// use crate::parametrizer::Parametrizer;
    ///
    /// let dynamic_rand = Parametrizer::new("rd(2+t<4*t)").unwrap();
    /// let computed_rand = Parametrizer::new("rc(4<8)").unwrap();
    ///
    /// assert_eq!(computed_rand.evaluate(2), computed_rand.evaluate(4));
    /// assert!(4 <= dynamic_rand.evaluate(2));
    /// assert!(16 > dynamic_rand.evaluate(4));
    /// ```
    pub fn new(param: &str) -> Result<Parametrizer<T>, ParametrizerError>
    {

        return Parametrizer::new_functions(param, vec![
        
            ParametrizerFunction::new("sin".to_string(), f64::sin),
            ParametrizerFunction::new("cos".to_string(), f64::cos)

        ]);

    }

    ///Constructor which allows for the user to define additional functions using a vector of
    ///ParametrizerFunction structs. Formats the param string like Parametrizer::new, with similar
    ///potential slowdown. Note that sine and cosine are not supported by default, but can be
    ///included in the user-defined list.
    ///
    /// # Examples
    /// ```
    /// use crate::parametrizer::Parametrizer;
    /// use crate::parametrizer::ParametrizerFunction;
    /// 
    /// fn square(t: f64) -> f64
    /// {
    ///
    ///     return t * t;
    ///
    /// }
    ///
    /// let logarithm_and_square = Parametrizer::new_functions("Log( square(t) + 3 )", vec![
    ///
    ///     ParametrizerFunction::new("LOG".to_string(), f64::ln),
    ///     ParametrizerFunction::new("square".to_string(), square)
    ///
    /// ]).unwrap();
    ///
    /// assert_eq!(7.0_f64.ln(), logarithm_and_square.evaluate(2.0));
    /// assert_eq!(28.0_f64.ln(), logarithm_and_square.evaluate(5.0));
    /// ```
    pub fn new_functions(param: &str, functions: Vec<ParametrizerFunction>) -> Result<Parametrizer<T>, ParametrizerError>
    {

        let term = term::create_parametrization::<T>(param, &functions[..])?;

        return Ok(Parametrizer::<T> { term });

    }

    ///Constructor which skips the added string formatting of Parametrizer::new and
    ///Parametrizer::new_functions, potentially speeding up parsing at the cost of unpredictable
    ///behavior when a string is not formatted exactly correctly. (I.e., includes extra spaces
    ///or capital letters.) Requires users to specify a vector of ParametrizerFunctions as in
    ///the case for Parametrizer::new_functions, and does not include sine or cosine by default.
    ///
    /// # Examples
    /// ```
    /// use crate::parametrizer::Parametrizer;
    /// use crate::parametrizer::ParametrizerFunction;
    ///
    /// let division = Parametrizer::quick_new("4/2", Vec::new()).unwrap();
    /// let subtraction = Parametrizer::quick_new("15+(-3*t)", Vec::new()).unwrap();
    /// let spaces = Parametrizer::quick_new("6+t", Vec::new()).unwrap();
    /// let sin = Parametrizer::quick_new("sin(t*t+t+-1)", vec![ ParametrizerFunction::new("sin".to_string(), f64::sin) ]).unwrap();
    /// let log = Parametrizer::quick_new("log(t+3)", vec![ ParametrizerFunction::new("log".to_string(), f64::ln)
    /// ]).unwrap();
    ///
    /// assert_eq!(2, division.evaluate(8));
    /// assert_eq!(6, subtraction.evaluate(3));
    /// assert_eq!(8, spaces.evaluate(2));
    /// assert_eq!(11.0_f64.sin(), sin.evaluate(3.0));
    /// assert_eq!(8.0_f64.ln(), log.evaluate(5.0));
    /// ```
    pub fn quick_new(param: &str, functions: Vec<ParametrizerFunction>) -> Result<Parametrizer<T>, ParametrizerError>
    {

        let term = term::quick_parametrization::<T>(param, &functions[..])?;

        return Ok(Parametrizer::<T> { term });

    }

    ///Used to compute the parametric function at a specific point. As the parsing is done once at
    ///creation time, the only overhead is due to pointers and recursion.
    pub fn evaluate(&self, t: T) -> T
    {

        return (*self.term).evaluate(t);

    }

}
