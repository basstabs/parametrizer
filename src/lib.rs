extern crate num;

use num::Num;
use num::ToPrimitive;
use num::FromPrimitive;
use std::str::FromStr;
use std::fmt;

pub mod term;

pub trait Number: Num + ToPrimitive + FromPrimitive + FromStr + Copy + 'static {}
impl<T: Num + ToPrimitive + FromPrimitive + FromStr + Copy + 'static> Number for T {}

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

pub struct Parametrizer<T: Number>
{

    ///The top-level term for the parametrized function. Must be placed on the heap as the
    ///recursion could be of theoretically unbounded depth
    term: Box<dyn term::Term<T>>

}

impl<T: Number> Parametrizer<T>
{

    fn new(param: &str) -> Result<Parametrizer<T>, ParametrizerError>
    {

        let term = term::create_parametrization::<T>(param);

        match term
        {

            Ok(t) => return Ok(Parametrizer::<T> { term: t }),
            Err(e) => return Err(e)
            

        };

    }

    fn evaluate(&self, t: T) -> T
    {

        return (*self.term).evaluate(t);

    }

}
