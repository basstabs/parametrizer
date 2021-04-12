use crate::Number;
use super::Term;

///A term which always returns the value of the parameter, t
pub struct VariableTerm
{
}

///Creates a variable term, which always returns the passed in value
///
/// # Examples
/// 
/// ```
/// use crate::parametrizer::term::variableterm::create_variable_term;
/// use crate::parametrizer::term::Term;
///
/// let int_variable = create_variable_term();
/// let float_variable = create_variable_term();
///
/// assert_eq!(3, int_variable.evaluate(3));
/// assert_eq!(4.5, float_variable.evaluate(4.5));
/// ```
pub fn create_variable_term() -> VariableTerm
{

    return VariableTerm {};

}

impl<T: Number> Term<T> for VariableTerm
{

    ///Always returns the passed in value of t
    fn evaluate(&self, t: T) -> T
    {

        return t;

    }

}
