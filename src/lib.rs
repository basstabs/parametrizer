extern crate num;

use num::Num;
use num::ToPrimitive;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::str::FromStr;
use std::fmt;

pub mod term;

pub trait Number: Num + ToPrimitive + FromPrimitive + PartialOrd + FromStr + Copy + 'static {}
impl<T: Num + ToPrimitive + FromPrimitive + PartialOrd + FromStr + Copy + 'static> Number for T {}

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

pub struct ParametrizerFunction
{

    shorthand: String,
    function: fn(f64) -> f64

}

impl ParametrizerFunction
{

    fn shorthand(&self) -> String
    {

        return format!("{}(", self.shorthand);

    }

    fn function(&self) -> fn(f64) -> f64
    {

        return self.function;

    }

}

pub struct Parametrizer<T: Number>
{

    //The top-level term for the parametrized function. Must be placed on the heap as the
    //recursion could be of theoretically unbounded depth
    term: Box<dyn term::Term<T>>

}

impl<T: Number> Parametrizer<T>
{

    fn new(param: &str) -> Result<Parametrizer<T>, ParametrizerError>
    {

        return Parametrizer::new_functions(param, vec![
        
            ParametrizerFunction { shorthand: "sin".to_string(), function: f64::sin },
            ParametrizerFunction { shorthand: "cos".to_string(), function: f64::cos }

        ]);

    }

    fn new_functions(param: &str, functions: Vec<ParametrizerFunction>) -> Result<Parametrizer<T>, ParametrizerError>
    {

        let term = term::create_parametrization::<T>(param, &functions[..])?;

        return Ok(Parametrizer::<T> { term });

    }

    fn quick_new(param: &str, functions: Vec<ParametrizerFunction>) -> Result<Parametrizer<T>, ParametrizerError>
    {

        let term = term::quick_parametrization::<T>(param, &functions[..])?;

        return Ok(Parametrizer::<T> { term });

    }

    fn evaluate(&self, t: T) -> T
    {

        return (*self.term).evaluate(t);

    }

}
