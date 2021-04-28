use rand::Rng;
use crate::Number;
use super::Term;

///A term which computes a random value each time it is called
pub struct RandomTerm<T: Number>
{

    min: Box<dyn Term<T> + Send + Sync>,
    max: Box<dyn Term<T> + Send + Sync>

}

impl<T: Number> RandomTerm<T>
{

    ///A term which randomly generates values between the given min and max terms
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::parametrizer::term::randomterm::RandomTerm;
    /// use crate::parametrizer::term::variableterm::VariableTerm;
    /// use crate::parametrizer::term::constantterm::ConstantTerm;
    /// use crate::parametrizer::term::Term;
    ///
    /// let const1 = ConstantTerm::new(6);
    /// let const2 = ConstantTerm::new(10);
    /// let const3 = ConstantTerm::new(2.5);
    /// let variable = VariableTerm::new();
    ///
    /// let rand1 = RandomTerm::new(Box::new(const1), Box::new(const2));
    /// let rand2 = RandomTerm::new(Box::new(const3), Box::new(variable));
    ///
    /// assert!(rand1.evaluate(2) < 10);
    /// assert!(rand1.evaluate(9) >= 6);
    /// assert!(rand2.evaluate(3.0) >= 2.5);
    /// assert!(rand2.evaluate(15.0) < 15.0);
    /// ```
    pub fn new(min: Box<dyn Term<T> + Send + Sync>, max: Box<dyn Term<T> + Send + Sync>) -> RandomTerm<T>
    {

        return RandomTerm {min, max};

    }

}

impl<T: Number> Term<T> for RandomTerm<T>
{

    ///Generates a random value between the min and max terms
    ///
    /// # Panics
    /// Panics if min is not less than max
    fn evaluate(&self, t: T) -> T
    {

        let mut rng = rand::thread_rng();

        let minimum = self.min.evaluate(t).to_f64().expect("Unable to convert generic type to f64 for random generation.");
        let maximum = self.max.evaluate(t).to_f64().expect("Unable to convert generic type to f64 for random generation.");

        if minimum >= maximum
        {

            panic!("Minimum is not smaller than maximum when attempting to generate a random value in parametrized RandomTerm.");

        }

        let random = rng.gen_range(minimum..maximum);

        return T::from_f64(random).expect("Unable to convert f64 to generic type after random generation.");

    }

}
