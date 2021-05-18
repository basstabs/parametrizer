use crate::Number;
use super::Term;

///A struct assigning to each piece of the function a time after which it is applicable. The term
///will be the evluated one until t passes the next part's after value
struct PiecewisePair<T: Number>
{

    term: Box<dyn Term<T> + Send + Sync>,
    after: T //The time after which to apply the term

}

///Struct containing a list of terms and times which split the number line into intervals during
///which different terms are applied
pub struct PiecewiseTerm<T: Number>
{

    parts: Vec<PiecewisePair<T>>,
    cycle: Option<T>

}

impl<T: Number> PiecewiseTerm<T>
{

    ///Creates a PiecewiseTerm, which is initialized to contain no terms. Terms and times must be
    ///added using add_part
    pub fn new() -> PiecewiseTerm<T>
    {

        return PiecewiseTerm { parts: Vec::new(), cycle: None };

    }

    ///Creates a looping piecewise term that evaluates anything larger than the looping number to
    ///its remainder with respect to the looping number
    pub fn looping(c: T) -> PiecewiseTerm<T>
    {

        return PiecewiseTerm { parts: Vec::new(), cycle: Some(c) };

    }

    ///Adds on a term to the piecewise function.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parametrizer::term::piecewiseterm::PiecewiseTerm;
    /// use crate::parametrizer::term::constantterm::ConstantTerm;
    /// use crate::parametrizer::term::Term;
    ///
    /// let mut piecewise = PiecewiseTerm::new();
    /// let mut looping = PiecewiseTerm::looping(10);
    ///
    /// let const1 = ConstantTerm::new(3);
    /// let const2 = ConstantTerm::new(5);
    /// let const3 = ConstantTerm::new(9);
    ///
    /// let const4 = ConstantTerm::new(2);
    /// let const5 = ConstantTerm::new(4);
    /// let const6 = ConstantTerm::new(6);
    ///
    /// piecewise.add_part(Box::new(const1), 0);
    /// piecewise.add_part(Box::new(const2), 5);
    /// piecewise.add_part(Box::new(const3), 10);
    ///
    /// looping.add_part(Box::new(const4), 1);
    /// looping.add_part(Box::new(const5), 5);
    /// looping.add_part(Box::new(const6), 9);
    ///
    /// assert_eq!(3, piecewise.evaluate(2));
    /// assert_eq!(5, piecewise.evaluate(8));
    /// assert_eq!(9, piecewise.evaluate(20));
    ///
    /// assert_eq!(2, looping.evaluate(3));
    /// assert_eq!(4, looping.evaluate(16));
    /// assert_eq!(6, looping.evaluate(109));
    /// ```
    pub fn add_part(&mut self, term: Box<dyn Term<T> + Send + Sync>, after: T)
    {

        self.parts.push(PiecewisePair::<T> { term, after });

    }

}

impl<T: Number> Term<T> for PiecewiseTerm<T>
{

    ///Iterates through all of the piecewise parts, returning the evluation of the term assigned to
    ///the the interval containing t
    fn evaluate(&self, time: T) -> T
    {

        let mut iter = self.parts.iter();

        let mut current = match iter.next()
        {

            Some(t) => &t.term,
            None => return T::zero()

        };

        let mut t = time;

        if let Some(c) = self.cycle
        {

            if t > c
            {

                t = t % c;

            }

        }

        for part in iter
        {

            if t >= part.after
            {

                current = &part.term;

            }
            else
            {

                return current.evaluate(t);

            }

        }

        return current.evaluate(t);

    }

}
