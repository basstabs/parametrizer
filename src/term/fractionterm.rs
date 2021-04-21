use crate::Number;
use super::Term;

///A term which divides one stored term by another
pub struct FractionTerm<T: Number>
{

    numerator: Box<dyn Term<T>>,
    denominator: Box<dyn Term<T>> 

}

impl<T: Number> FractionTerm<T>
{

    ///Creates a fraction term from the given numerator and denominator terms
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parametrizer::term::fractionterm::FractionTerm;
    /// use crate::parametrizer::term::variableterm::VariableTerm;
    /// use crate::parametrizer::term::constantterm::ConstantTerm;
    /// use crate::parametrizer::term::Term;
    ///
    /// let const1 = ConstantTerm::new(6);
    /// let const2 = ConstantTerm::new(2);
    /// let const3 = ConstantTerm::new(10);
    /// let variable = VariableTerm::new();
    ///
    /// let frac1 = FractionTerm::new(Box::new(const1), Box::new(const2));
    /// let frac2 = FractionTerm::new(Box::new(const3), Box::new(variable));
    ///
    /// assert_eq!(3, frac1.evaluate(1));
    /// assert_eq!(2, frac2.evaluate(5)); 
    /// ```
    ///
    /// ```should_panic
    /// use crate::parametrizer::term::fractionterm::FractionTerm;
    /// use crate::parametrizer::term::variableterm::VariableTerm;
    /// use crate::parametrizer::term::constantterm::ConstantTerm;
    /// use crate::parametrizer::term::Term;
    ///
    /// let const1 = ConstantTerm::new(6);
    /// let variable = VariableTerm::new();
    ///
    /// let frac1 = FractionTerm::new(Box::new(const1), Box::new(variable));
    /// frac1.evaluate(0);
    /// ```
    pub fn new(numerator: Box<dyn Term<T>>, denominator: Box<dyn Term<T>>) -> FractionTerm<T>
    {

        return FractionTerm::<T> { numerator, denominator };

    }

}

impl<T: Number> Term<T> for FractionTerm<T>
{

    ///Divides the numerator by the denominator.
    ///
    /// # Panics
    /// Panics if the denominator evaluates to 0
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
