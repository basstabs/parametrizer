use crate::Number;

pub mod constantterm;
pub mod variableterm;
pub mod sequenceterm;
pub mod scalarterm;
pub mod randomterm;
pub mod piecewiseterm;
pub mod fractionterm;

use super::ParametrizerError;

///A trait used to represent a particular component of a parametrized function
pub trait Term<T: Number>
{

    ///Takes in the parameter t and evaluates the output of the term
    fn evaluate(&self, t: T) -> T;

}

///Entry function for parametrizing, which does some QoL formatting on the param string
pub fn create_parametrization<T: Number>(text: &str) -> Result<Box<dyn Term<T>>, ParametrizerError>
{

    let mut lower = text.to_lowercase();
    lower.retain(|c| { return !c.is_whitespace(); }); //Allow users to use comfortable spacing
    lower = lower.replace("\\", "/"); //Allow users to use either division symbol

    let param = &lower[0..];

    return quick_parametrization(param);

}

///Checks the piecewise case, which can only occur at the top level, then recurses normally using
///parametrize_string. Can be called directly with a properly formatted param string to avoid the
///potentially expensive formatting operations of create_parametrization
pub fn quick_parametrization<T: Number>(param: &str) ->Result<Box<dyn Term<T>>, ParametrizerError>
{

    if param.starts_with("p") //Piecewise case
    {
    }

    //Not piecewise, recurse normally
    return parametrize_string(param);

}
///The main function which enables us to convert a string into a recursive stack of functions
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let constant = parametrize_string::<f32>("1.35").unwrap();
///
/// assert_eq!(1.35, (*constant).evaluate(2.0));
/// assert_eq!(1.35, (*constant).evaluate(3.4));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let variable = parametrize_string::<f32>("t").unwrap();
///
/// assert_eq!(3.0, (*variable).evaluate(3.0));
/// assert_ne!(4.2, (*variable).evaluate(1.25));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let addition = parametrize_string::<f32>("1+t").unwrap();
///
/// assert_eq!(9.0, addition.evaluate(8.0));
/// assert_eq!(1.16, addition.evaluate(0.16));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let equation = parametrize_string::<i32>("13+((2*t)+5)").unwrap();
///
/// assert_eq!(20, equation.evaluate(1));
/// assert_eq!(30, equation.evaluate(6));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let division = parametrize_string::<i32>("6/t").unwrap();
///
/// assert_eq!(2, division.evaluate(3));
/// assert_eq!(3, division.evaluate(2));
/// ```
pub fn parametrize_string<T: Number>(param: &str) -> Result<Box<dyn Term<T>>, ParametrizerError>
{

    //Terminal case: check if the passed in string is simply "t", in which case we want a variable
    //term to use in our calculations
    if param.eq("t")
    {

        return Ok(Box::new(variableterm::create_variable_term()));

    }

    //Terminal case: check if the passed in string can be parsed into a number of the desired type,
    //in which case we want a constant term returning that number
    let c = param.parse();
    match c
    {

        Ok(c) => return Ok(Box::new(constantterm::create_constant_term(c))),
        Err(_e) => ()

    };
   
    //Simplification case: If the entire string is in parentheses, slice them off and recurse
    let length = param.len();
    if param.starts_with("(") && param.ends_with(")")
    {

        return parametrize_string::<T>(&(param[1..length - 1]));

    }

    //Recursive case: If there is an addition symbol, we may need to split. PROCESSED before
    //multiplication so that multiplication is PERFORMED first
    if param.contains('+')
    {

        let terms = respectful_symbol_split(param, '+', '(', ')')?;

        if terms.len() > 1 //If we actually split, then create a SequenceTerm adding up the values. If there is no split, continue to a different case
        {

            let mut sum_terms = Vec::new();

            for term in terms
            {

                let new_term = parametrize_string(term)?;

                sum_terms.push(new_term);

            }

            return Ok(Box::new(sequenceterm::create_sequence_term(sum_terms, sequenceterm::SequenceOperations::Addition)));

        }

    }

    //Recursive case: If there is a multiplication symbol, we may need to split. PROCESSED after
    //addition so that multiplication is PERFORMED first
    if param.contains('*')
    {

        let terms = respectful_symbol_split(param, '*', '(', ')')?;

        if terms.len() > 1 //If we actually split, then create a SequenceTerm multiplying thevalues. If there is no split, continue to a different case
        {

            let mut product_terms = Vec::new();

            for term in terms
            {

                let new_term = parametrize_string(term)?;

                product_terms.push(new_term);

            }

            return Ok(Box::new(sequenceterm::create_sequence_term(product_terms, sequenceterm::SequenceOperations::Multiplication)));

        }

    }

    //Recursive case: Check for a division sign and use the splitting algorithm. If the split
    //returns more than two terms, then we throw an error because division is not associative and
    //we won't know how to proceed
    if param.contains('/') 
    {

        let terms = respectful_symbol_split(param, '/', '(', ')')?;

        if terms.len() > 1
        {

            if terms.len() > 2
            {

                return Err(ParametrizerError { param: param.to_string(), reason: "More than one division symbol in a term." });

            }

            let numerator = parametrize_string(terms[0])?;
            let denominator = parametrize_string(terms[1])?;

            return Ok(Box::new(fractionterm::create_fraction_term(numerator, denominator)));

        }

    }

    return Err(ParametrizerError { param: param.to_string(), reason: "Did not match any cases." });

}

//Used to parse parentheses, ignoring everything between an instance of left and an instance of
//right to be handled at a later step of the recursion.
fn respectful_symbol_split<'a>(param: &'a str, splitter: char, left: char, right: char) -> Result<Vec<&'a str>, ParametrizerError>
{

    //Counter used to keep track of "parentheses": We add one when we see left, and subtract one
    //when we see right. We only split if we encounter the splitting symbol when we are outside of
    //the "parentheses," i.e. balance is 0.
    let mut balance = 0;
    let mut last_split = 0;

    //A closure to match on instances of splitter, left, and right
    let symbols = |s: char| -> bool { return s == splitter || s == left || s == right; };

    //We iterate forward through all appearances of splitter, left, and right and act on each one
    let iter = param.match_indices(symbols);

    let mut splits = Vec::new();

    for symbol in iter
    {

        if symbol.1.contains(left)
        {

            balance += 1;

        }
        else if symbol.1.contains(right)
        {

            balance -= 1;

            if balance < 0 //More right than left at some point, which is a problem
            {

                return Err(ParametrizerError { param: param.to_string(), reason: "Malformed split, right exceeded left." });

            }

        }
        else //Must equal splitter
        {

            //If balance is 0, we are not in between left and right and so should split
            if balance == 0
            {

                splits.push(&(param[last_split..symbol.0]));

                last_split = symbol.0 + 1;

            }

        }

    }

    if balance > 0 //There were more left than right, which is a problem
    {

        return Err(ParametrizerError { param: param.to_string(), reason: "Malformed split, left exceeded right." });

    }
    else
    {

        //Push the final term, which wasn't captured by finding an instance of splitter
        splits.push(&(param[last_split..]));

        return Ok(splits);

    }

}

#[cfg(test)]
mod split_tests
{

    use super::*;

    #[test]
    fn test_splitting ()
    {

        let no_split = respectful_symbol_split("15*t", '+', '(', ')').expect("Splitting failed when there was nothing to split.");
        let ignore_split = respectful_symbol_split("(15*t)", '*', '(', ')').expect("Splitting failed when the splitter was in parentheses.");
        let easy_split = respectful_symbol_split("9+3*t+6", '+', '(', ')').expect("Splitting failed with no parentheses.");
        let hard_split = respectful_symbol_split("1+(6+9*t)+(6+(5+t))", '+', '(', ')').expect("Splitting failed with parentheses.");

        let right_split = respectful_symbol_split("(t+1))*5", '+', '(', ')');
        let left_split = respectful_symbol_split("((t+1)*5", '+', '(', ')');

        assert_eq!(no_split, ["15*t"]);
        assert_eq!(ignore_split, ["(15*t)"]);
        assert_eq!(easy_split, ["9", "3*t", "6"]);
        assert_eq!(hard_split, ["1", "(6+9*t)", "(6+(5+t))"]);

        match right_split
        {

            Ok(_) => panic!("Expected too many right parentheses error."),
            Err(e) => assert_eq!(e.reason, "Malformed split, right exceeded left.")

        }

        match left_split
        {

            Ok(_) => panic!("Expected too many left parentheses error."),
            Err(e) => assert_eq!(e.reason, "Malformed split, left exceeded right.")

        }

    }

    #[test]
    fn test_division ()
    {

        let division = parametrize_string::<f32>("6/(t+1)/2");

        match division
        {

            Ok(_) => panic!("Expected too many division terms error."),
            Err(e) => assert_eq!(e.reason, "More than one division symbol in a term.")

        }

    }

}
