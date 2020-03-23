use whitenoise_validator::errors::*;

use crate::base::NodeArguments;
use whitenoise_validator::base::{Value, get_argument, ArrayND};
use crate::components::Evaluable;
use whitenoise_validator::proto;
use ndarray::{ArrayD, Array};
use std::ops::Add;
use crate::utilities::utilities::get_num_columns;
use num::Zero;

impl Evaluable for proto::Sum {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<Value> {
        match get_argument(&arguments, "data")?.get_arraynd()? {
            ArrayND::F64(data) => Ok(sum(&data)?.into()),
            ArrayND::I64(data) => Ok(sum(&data)?.into()),
            _ => return Err("data must be either f64 or i64".into())
        }
    }
}

/// Calculates sum for each column of the data.
///
/// # Arguments
/// * `data` - Data for which you would like the sum of each column.
///
/// # Return
/// Sum of each column of the data.
///
/// # Example
/// ```
/// use ndarray::prelude::*;
/// use whitenoise_runtime::components::sum::sum;
/// let data = arr2(&[ [1.,10.], [2., 20.], [3., 30.] ]).into_dyn();
/// let sums = sum(&data).unwrap();
/// assert!(sums == arr2(&[[6., 60.]]).into_dyn());
/// ```
pub fn sum<T: Add<T, Output=T> + Zero + Copy>(data: &ArrayD<T>) -> Result<ArrayD<T>> {
    let data = data.clone();

    // iterate over the generalized columns
    let means = data.gencolumns().into_iter()
        .map(|column| column.fold(T::zero(), |sum, i| sum + *i)).collect::<Vec<T>>();

    let array = match data.ndim() {
        1 => Array::from_shape_vec(vec![], means),
        2 => Array::from_shape_vec(vec![1 as usize, get_num_columns(&data)? as usize], means),
        _ => return Err("invalid data shape for Sum".into())
    };

    match array {
        Ok(array) => Ok(array),
        Err(_) => Err("unable to package Sum result into an array".into())
    }
}
