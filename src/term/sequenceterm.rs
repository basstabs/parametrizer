use crate::Number;
use super::Term;

///An enum defining the different operations supported by sequence terms
pub enum SequenceOperations
{

    Addition,
    Multiplication

}

///A term which takes in a vector of terms and combines them together with an operator (i.e. +, *)
pub struct SequenceTerm<T: Number>
{

    terms: Vec<Box<dyn Term<T>>>,
    operation: SequenceOperations

}

///Creates a sequence term from the list of terms and the given operation
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::constantterm;
/// use crate::parametrizer::term::variableterm;
/// use crate::parametrizer::term::sequenceterm::SequenceOperations;
/// use crate::parametrizer::term::sequenceterm::create_sequence_term;
/// use crate::parametrizer::term::Term;
///
/// let const1 = Box::new(constantterm::create_constant_term(13));
/// let const2 = Box::new(constantterm::create_constant_term(5));
/// let variable = Box::new(variableterm::create_variable_term());
///
/// let terms : Vec<Box<dyn Term<i32>>> = vec![const1, const2, variable];
///
/// let addition = create_sequence_term(terms, SequenceOperations::Addition);
///
/// assert_eq!(19, addition.evaluate(1));
/// assert_eq!(24, addition.evaluate(6));
/// ```
///
/// ```
/// use crate::parametrizer::term::constantterm;
/// use crate::parametrizer::term::variableterm;
/// use crate::parametrizer::term::sequenceterm::SequenceOperations;
/// use crate::parametrizer::term::sequenceterm::create_sequence_term;
/// use crate::parametrizer::term::Term;
///
/// let const1 = Box::new(constantterm::create_constant_term(13));
/// let const2 = Box::new(constantterm::create_constant_term(5));
/// let variable = Box::new(variableterm::create_variable_term());
///
/// let terms : Vec<Box<dyn Term<i32>>> = vec![const1, const2, variable];
///
/// let addition = create_sequence_term(terms, SequenceOperations::Multiplication);
///
/// assert_eq!(65, addition.evaluate(1));
/// assert_eq!(390, addition.evaluate(6)); 
/// ```
///
pub fn create_sequence_term<T: Number>(terms: Vec<Box<dyn Term<T>>>, operation: SequenceOperations) -> SequenceTerm<T>
{

    return SequenceTerm {terms, operation};

}

impl<T: Number> SequenceTerm<T>
{

    fn unit(&self) -> T
    {

        match self.operation
        {

            SequenceOperations::Addition => T::zero(),
            SequenceOperations::Multiplication => T::one()

        }

    }

    fn compound(&self, l: T, r: T) -> T
    {

        match self.operation
        {

            SequenceOperations::Addition => l + r,
            SequenceOperations::Multiplication => l * r

        }

    }

}

impl<T: Number> Term<T> for SequenceTerm<T>
{

    fn evaluate(&self, t: T) -> T
    {

        let mut computed = self.unit();

        for term in &self.terms
        {

            computed = self.compound(computed, term.evaluate(t));

        }

        return computed;

    }

}
