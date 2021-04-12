use crate::Number;
use super::Term;

/// A Term that returns a constant value no matter what value is passed in
pub struct ConstantTerm<T: Number>
{

    c: T

}

///Creates a ConstantTerm obtained by parsing a string slice
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::constantterm::create_constant_term;
/// use crate::parametrizer::term::Term;
///
/// let int_constant = create_constant_term::<i32>(17);
/// let float_constant = create_constant_term::<f32>(5.2);
///
/// assert_eq!(17, int_constant.evaluate(9));
/// assert_eq!(17, int_constant.evaluate(-1));
/// assert_eq!(5.2, float_constant.evaluate(3.4));
/// assert_eq!(5.2, float_constant.evaluate(5.0));
/// ```
pub fn create_constant_term<T: Number>(param: T) -> ConstantTerm<T>
{

    return ConstantTerm::<T> { c: param };

}

impl<T: Number> Term<T> for ConstantTerm<T>
{

    ///Returns the associated constant number no matter what value of t is passed in
    fn evaluate(&self, _t: T) -> T
    {

        return self.c;

    }

}
