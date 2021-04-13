use crate::Number;
use super::Term;

///A term which divides one stored term by another
pub struct FractionTerm<T: Number>
{

    numerator: Box<dyn Term<T>>,
    denominator: Box<dyn Term<T>> 

}

///Creates a fraction term from the given numerator and denominator terms
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::fractionterm::create_fraction_term;
/// use crate::parametrizer::term::variableterm::create_variable_term;
/// use crate::parametrizer::term::constantterm::create_constant_term;
/// use crate::parametrizer::term::Term;
///
/// let const1 = create_constant_term::<i32>(6);
/// let const2 = create_constant_term::<i32>(2);
/// let const3 = create_constant_term::<i32>(10);
/// let variable = create_variable_term();
///
/// let frac1 = create_fraction_term(Box::new(const1), Box::new(const2));
/// let frac2 = create_fraction_term(Box::new(const3), Box::new(variable));
///
/// assert_eq!(3, frac1.evaluate(1));
/// assert_eq!(2, frac2.evaluate(5)); 
/// ```
///
/// ```should_panic
/// use crate::parametrizer::term::fractionterm::create_fraction_term;
/// use crate::parametrizer::term::variableterm::create_variable_term;
/// use crate::parametrizer::term::constantterm::create_constant_term;
/// use crate::parametrizer::term::Term;
///
/// let const1 = create_constant_term::<i32>(6);
/// let variable = create_variable_term();
///
/// let frac1 = create_fraction_term(Box::new(const1), Box::new(variable));
/// frac1.evaluate(0);
/// ```
pub fn create_fraction_term<T: Number>(numerator: Box<dyn Term<T>>, denominator: Box<dyn Term<T>>) -> FractionTerm<T>
{

    return FractionTerm::<T> { numerator, denominator };

}

impl<T: Number> Term<T> for FractionTerm<T>
{

    fn evaluate(&self, t: T) -> T
    {

        let d = self.denominator.evaluate(t);

        if d == T::zero() //If the denominator is 0, panic
        {

            panic!("Cannot divide by 0 in parametrized InverseTerm. Make sure the function you set as your denominator is never zero on your inputs.");

        }
        else
        {

            return self.numerator.evaluate(t) / d; 

        }

    }

}
