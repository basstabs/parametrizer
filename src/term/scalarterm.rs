use crate::Number;
use super::Term;

///A term which multiplies a given term by a constant number. Especially useful for - signs
pub struct ScalarTerm<T: Number>
{

    term: Box<dyn Term<T> + Send + Sync>,
    scale: T

}

impl<T: Number> ScalarTerm<T>
{

    ///Creates a ScalarTerm using the given subterm and multiplying value
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parametrizer::term::scalarterm::ScalarTerm;
    /// use crate::parametrizer::term::constantterm::ConstantTerm;
    /// use crate::parametrizer::term::variableterm::VariableTerm;
    /// use crate::parametrizer::term::Term;
    ///
    /// let const_term = ConstantTerm::new(1.98);
    /// let variable_term = VariableTerm::new();
    ///
    /// let scalar1 = ScalarTerm::new(Box::new(const_term), 1.02);
    /// let scalar2 = ScalarTerm::new(Box::new(variable_term), 3);
    ///
    /// assert_eq!(1.02 * 1.98, scalar1.evaluate(3.0));
    /// assert_eq!(6, scalar2.evaluate(2));
    /// ```
    pub fn new(term: Box<dyn Term<T> + Send + Sync>, scale: T) -> ScalarTerm<T>
    {

        return ScalarTerm { term, scale };

    }

}

impl<T: Number> Term<T> for ScalarTerm<T>
{

    ///Multiplies the subterm by the given constant
    fn evaluate(&self, t: T) -> T
    {

        return self.scale * self.term.evaluate(t);

    }

}
