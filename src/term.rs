use crate::Number;
use rand::Rng;

pub mod constantterm;
pub mod variableterm;
pub mod sequenceterm;
pub mod scalarterm;
pub mod randomterm;
pub mod piecewiseterm;
pub mod fractionterm;
pub mod functionterm;

use super::ParametrizerError;
use super::ParametrizerFunction;

const DYNAMIC_RANDOM_IDENTIFIER : &str = "rd(";
const COMPUTED_RANDOM_IDENTIFIER : &str = "rc(";

const PIECEWISE_IDENTIFIER : &str = "p";

///A trait used to represent a particular component of a parametrized function
pub trait Term<T: Number>
{

    ///Takes in the parameter t and evaluates the output of the term
    fn evaluate(&self, t: T) -> T;

}

///Entry function for parametrizing, which does some QoL formatting on the param string
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::create_parametrization;
///
/// let division = create_parametrization::<u32>("4\\2", &[]).unwrap();
/// let subtraction = create_parametrization::<i32>("15-3*t", &[]).unwrap();
/// let spaces = create_parametrization::<i32>("6 + T", &[]).unwrap();
///
/// assert_eq!(2, division.evaluate(8));
/// assert_eq!(6, subtraction.evaluate(3));
/// assert_eq!(8, spaces.evaluate(2));
/// ```
pub fn create_parametrization<T: Number>(text: &str, functions: &[ParametrizerFunction]) -> Result<Box<dyn Term<T> + Send + Sync>, ParametrizerError>
{

    let mut lower = text.to_lowercase();
    lower.retain(|c| { return !c.is_whitespace(); }); //Allow users to use comfortable spacing
    lower = lower.replace("\\", "/"); //Allow users to use either division symbol
    lower = lower.replace("-", "+-"); //Allow users to implement subtraction, i.e. 1-t will be read as 1+-t. Extra leading +'s will be trimmed during recursion

    let param = &(lower[0..]);

    return quick_parametrization(param, functions);

}

///Checks the piecewise case, which can only occur at the top level, then recurses normally using
///parametrize_string. Can be called directly with a properly formatted param string to avoid the
///potentially expensive formatting operations of create_parametrization
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::quick_parametrization;
///
/// let p1 = quick_parametrization::<i32>("p2>0|4>2|8>6", &[]).unwrap();
/// let p2 = quick_parametrization::<i32>("p2*t>0|4>2", &[]).unwrap();
/// let eq = quick_parametrization::<f32>("4+2*t", &[]).unwrap();
///
/// assert_eq!(2, p1.evaluate(1));
/// assert_eq!(4, p1.evaluate(5));
/// assert_eq!(2, p2.evaluate(1));
/// assert_eq!(4, p2.evaluate(9));
/// assert_eq!(9.0, eq.evaluate(2.5));
/// ```
///
/// ```
/// use crate::parametrizer::term::quick_parametrization;
///
/// let p1 = quick_parametrization::<i32>("p[10]t>0|t*t+-16>5", &[]).unwrap();
/// let p2 = quick_parametrization::<f32>("p[3.5]4>0|8>2", &[]).unwrap();
///
/// assert_eq!(4, p1.evaluate(4));
/// assert_eq!(48, p1.evaluate(18));
/// assert_eq!(3, p1.evaluate(23));
/// assert_eq!(4.0, p2.evaluate(8.9));
/// assert_eq!(8.0, p2.evaluate(30.1));
/// ```
pub fn quick_parametrization<T: Number>(param: &str, functions: &[ParametrizerFunction]) ->Result<Box<dyn Term<T> + Send + Sync>, ParametrizerError>
{

    //Check to see if the string starts with a function identifier, which is necessary because
    //identifiers may start with PIECEWISE_IDENTIFIER
    for function in functions
    {

        if param.starts_with(function.shorthand())
        {

            return parametrize_string(param, functions);

        }

    }

    if param.starts_with(PIECEWISE_IDENTIFIER) //Piecewise case
    {

        let simplified_string = &(param[PIECEWISE_IDENTIFIER.len()..]);

        let mut parts_string = &(param[PIECEWISE_IDENTIFIER.len()..]);

        let mut looping = false;

        let mut loop_value = T::zero();

        if simplified_string.starts_with("[")
        {

            let closing_index = match simplified_string.find("]")
            {

                Some(i) => i,
                None => return Err(ParametrizerError { param: simplified_string.to_string(), reason: "Unable to find closing bracket for looping piecewise term." })

            };


            let loop_string = &(simplified_string[1..closing_index]);

            loop_value = match loop_string.parse()
            {

                Ok(l) => l,
                Err(_e) => return Err(ParametrizerError { param: loop_string.to_string(), reason: "Could not parse the loop value for looping piecewise term." })

            };

            parts_string = &(param[closing_index+2..]);

            looping = true;

        }

        let parts : Vec<&str> = parts_string.split("|").collect();

        let mut piecewise = if looping { piecewiseterm::PiecewiseTerm::looping(loop_value) } else { piecewiseterm::PiecewiseTerm::new() };

        for part in parts
        {

            let part_info : Vec<&str> = part.split(">").collect();

            if part_info.len() != 2
            {

                return Err(ParametrizerError { param: part.to_string(), reason: "Unexpected number of splits for piecewise part. Each part should be separated by an = sign and contain a term and a number separated by a >" });

            }

            let term = parametrize_string(part_info[0], functions)?;

            let time = match part_info[1].parse()
            {

                Ok(t) => t,
                Err(_e) => return Err(ParametrizerError { param: part.to_string(), reason: "Could not parse the time value for piecewise part."})

            };

            piecewise.add_part(term, time);

        }

        return Ok(Box::new(piecewise));

    }

    //Not piecewise, recurse normally
    return parametrize_string(param, functions);

}

///The main function which enables us to convert a string into a recursive stack of functions
///
/// # Examples
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let constant = parametrize_string::<f32>("1.35", &[]).unwrap();
///
/// assert_eq!(1.35, (*constant).evaluate(2.0));
/// assert_eq!(1.35, (*constant).evaluate(3.4));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let variable = parametrize_string::<f32>("t", &[]).unwrap();
///
/// assert_eq!(3.0, (*variable).evaluate(3.0));
/// assert_ne!(4.2, (*variable).evaluate(1.25));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let addition = parametrize_string::<f32>("1+t", &[]).unwrap();
///
/// assert_eq!(9.0, addition.evaluate(8.0));
/// assert_eq!(1.16, addition.evaluate(0.16));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let equation = parametrize_string::<i32>("13+((2*t)+5)", &[]).unwrap();
///
/// assert_eq!(20, equation.evaluate(1));
/// assert_eq!(30, equation.evaluate(6));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let division = parametrize_string::<i32>("6/t", &[]).unwrap();
///
/// assert_eq!(2, division.evaluate(3));
/// assert_eq!(3, division.evaluate(2));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let equation = parametrize_string::<i32>("13+-t", &[]).unwrap();
/// let negation = parametrize_string::<i32>("-t", &[]).unwrap();
///
/// assert_eq!(10, equation.evaluate(3));
/// assert_eq!(-9, negation.evaluate(9));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
///
/// let dynamic_rand = parametrize_string::<i32>("rd(2+t<4*t)", &[]).unwrap();
/// let computed_rand = parametrize_string::<i32>("rc(4<8)", &[]).unwrap();
///
/// assert_eq!(computed_rand.evaluate(2), computed_rand.evaluate(4));
/// assert!(4 <= dynamic_rand.evaluate(2));
/// assert!(16 > dynamic_rand.evaluate(4));
/// ```
///
/// ```
/// use crate::parametrizer::term::parametrize_string;
/// use crate::parametrizer::ParametrizerFunction;
///
/// let sin = parametrize_string::<f64>("sin(2*t)", &[ParametrizerFunction::new("sin".to_string(), f64::sin)]).unwrap();
///
/// assert_eq!(4.0_f64.sin(), sin.evaluate(2.0));
/// assert_eq!(8.0_f64.sin(), sin.evaluate(4.0));
/// ```
pub fn parametrize_string<T: Number>(param: &str, functions: &[ParametrizerFunction]) -> Result<Box<dyn Term<T> + Send + Sync>, ParametrizerError>
{

    //Terminal case: check if the passed in string is simply "t", in which case we want a variable
    //term to use in our calculations
    if param.eq("t")
    {

        return Ok(Box::new(variableterm::VariableTerm::new()));

    }

    //Terminal case: check if the passed in string can be parsed into a number of the desired type,
    //in which case we want a constant term returning that number
    let c = param.parse();
    match c
    {

        Ok(c) => return Ok(Box::new(constantterm::ConstantTerm::new(c))),
        Err(_e) => ()

    };

    //Simplification case: If the entire string is in parentheses, slice them off and recurse
    let length = param.len();
    if param.starts_with("(") && param.ends_with(")")
    {

        return parametrize_string::<T>(&(param[1..length - 1]), functions);

    }

    //Simplification case: If the first character is a +, then remove it and recurse. Happens
    //because a leading - was replaced by +- in create_parametrization
    if param.starts_with("+")
    {

        return parametrize_string::<T>(&(param[1..]), functions);

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

                let new_term = parametrize_string(term, functions)?;

                sum_terms.push(new_term);

            }

            return Ok(Box::new(sequenceterm::SequenceTerm::new(sum_terms, sequenceterm::SequenceOperations::Addition)));

        }

    }

    //Recursive case: If there is a multiplication symbol, we may need to split. PROCESSED after
    //addition so that multiplication is PERFORMED first
    if param.contains('*')
    {

        let terms = respectful_symbol_split(param, '*', '(', ')')?;

        if terms.len() > 1 //If we actually split, then create a SequenceTerm multiplying the values. If there is no split, continue to a different case
        {

            let mut product_terms = Vec::new();

            for term in terms
            {

                let new_term = parametrize_string(term, functions)?;

                product_terms.push(new_term);

            }

            return Ok(Box::new(sequenceterm::SequenceTerm::new(product_terms, sequenceterm::SequenceOperations::Multiplication)));

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

            let numerator = parametrize_string(terms[0], functions)?;
            let denominator = parametrize_string(terms[1], functions)?;

            return Ok(Box::new(fractionterm::FractionTerm::new(numerator, denominator)));

        }

    }

    //Recursive case: Check for a negative sign leading the term. As we have remove the top level
    //of binary operations, negate the remaining term
    if param.starts_with("-")
    {

        let term = parametrize_string(&(param[1..]), functions)?;

        return Ok(Box::new(scalarterm::ScalarTerm::new(term, T::zero() - T::one())));

    }

    //Recursive case: Check for a leading "rd", which designates a dynamic random value which
    //changes each time evaluate is called. It is bounded between the first and second term.
    if param.starts_with(DYNAMIC_RANDOM_IDENTIFIER) && param.ends_with(")")
    {

        let simplified_param = &(param[DYNAMIC_RANDOM_IDENTIFIER.len()..param.len() - 1]);
        let splits : Vec<&str> = simplified_param.split("<").collect();

        if splits.len() != 2
        {

            return Err(ParametrizerError { param: param.to_string(), reason: "Random parametrization did not split into exactly two terms." });

        }

        let min = parametrize_string(splits[0], functions)?;
        let max = parametrize_string(splits[1], functions)?;

        return Ok(Box::new(randomterm::RandomTerm::new(min, max)));

    }

    //Recursive case:: Check for a leading predefined function shorthand and create a function term
    //using it and the interior term
    for function in functions
    {

        let shorthand = function.shorthand();
        if param.starts_with(shorthand) && param.ends_with(")")
        {

            let simplified_param = &(param[shorthand.len()..param.len() - 1]);

            let term = parametrize_string(simplified_param, functions)?;

            return Ok(Box::new(functionterm::FunctionTerm::new(term, function.function())));

        }

    }

    //Terminal case: Check for a leading "rc", which designates a computed random value which is
    //calculated at parametrize time and never changes.
    if param.starts_with(COMPUTED_RANDOM_IDENTIFIER) && param.ends_with(")")
    {

        let simplified_param = &(param[COMPUTED_RANDOM_IDENTIFIER.len()..param.len() - 1]);
        let splits : Vec<&str> = simplified_param.split("<").collect();

        if splits.len() != 2
        {

            return Err(ParametrizerError { param: param.to_string(), reason: "Random parametrization did not split into exactly two terms." });

        }

        let min = splits[0].parse();
        let max = splits[1].parse();

        let min = match min
        {

            Ok(m) => m,
            Err(_e) => return Err(ParametrizerError { param: param.to_string(), reason: "Could not parse the minimum value as a number for computed random generation."})

        };

        let max = match max
        {

            Ok(m) => m,
            Err(_e) => return Err(ParametrizerError { param: param.to_string(), reason: "Could not parse the maximum value as a umber for computed random generation."})

        };

        let constant = T::from_f64(rand::thread_rng().gen_range(min..max));
        let constant = match constant
        {

            Some(c) => c,
            None => return Err(ParametrizerError {param: param.to_string(), reason: "Could not convert to the generic type T from f64 for computed random generation."})

        };

        return Ok(Box::new(constantterm::ConstantTerm::new(constant)));

    }

    return Err(ParametrizerError { param: param.to_string(), reason: "Did not match any cases. Do not forget to write multiplication explicitly, i.e. 'n*t' as opposed to 'nt'." });

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
mod term_tests
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

        let division = parametrize_string::<f32>("6/(t+1)/2", &[]);

        match division
        {

            Ok(_) => panic!("Expected too many division terms error."),
            Err(e) => assert_eq!(e.reason, "More than one division symbol in a term.")

        }

    }

    #[test]
    fn test_identifiers ()
    {

        fn polynomial(t: f64) -> f64
        {

            return t*t+2.0*t+1.0;

        }

        let succeed = quick_parametrization::<f32>("poly(t)", &vec![super::ParametrizerFunction::new("poly".to_string(), polynomial)]);
        let fail = quick_parametrization::<f32>("poly(t)", &[]);

        match succeed
        {

            Ok(t) => assert_eq!(16.0, t.evaluate(3.0)),
            Err(_) => panic!("Expected successful parsing of added function.")

        };

        match fail
        {

            Ok(_) => panic!("Expected unable to parse error"),
            Err(e) => assert_eq!(e.reason, "Unexpected number of splits for piecewise part. Each part should be separated by an = sign and contain a term and a number separated by a >")

        };

    }

}
