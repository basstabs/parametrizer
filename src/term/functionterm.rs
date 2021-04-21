use crate::Number;
use super::Term;

///A term which applies the stored function to the stored term evaluated at the given number
pub struct FunctionTerm<T: Number>
{

    term: Box<dyn Term<T>>,
    function: fn(f64) -> f64

}

///Creates a function term from the given term and function
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::functionterm::FunctionTerm;
/// use crate::parametrizer::term::variableterm::VariableTerm;
/// use crate::parametrizer::term::constantterm::ConstantTerm;
/// use crate::parametrizer::term::Term;
///
/// let const1 = ConstantTerm::new(20);
/// let variable = VariableTerm::new();
///
/// let sin = FunctionTerm::new(Box::new(const1), f64::sin);
/// let cos = FunctionTerm::new(Box::new(variable), f64::cos);
///
/// assert_eq!((20.0_f64.sin()) as i32, sin.evaluate(5));
/// assert_eq!(3.14_f64.cos(), cos.evaluate(3.14));
/// ```
impl<T: Number> FunctionTerm<T>
{

    pub fn new(term: Box<dyn Term<T>>, function: fn(f64) -> f64) -> FunctionTerm<T>
    {

        return FunctionTerm::<T> { term, function };

    }

}

impl<T: Number> Term<T> for FunctionTerm<T>
{

    ///Evaluates the function at the term evaluated for the given value of t
    ///
    /// # Panics
    /// Panics if the generic type T cannot be successfully converted to f64
    fn evaluate(&self, t: T) -> T
    {

        return T::from_f64((self.function)(self.term.evaluate(t).to_f64().expect("Unable to convert generic type to f64 for FunctionTerm"))).expect("Unable to create generic type T value from f64 for FunctionTerm");

    }

}
