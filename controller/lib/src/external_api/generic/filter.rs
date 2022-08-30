use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterServiceError {
    #[error("Offset out of range")]
    OutOfRange,
}

/// `FilterService` is a struct that can be used as a services to filter result.
pub struct FilterService {}

impl Default for FilterService {
    fn default() -> Self {
        Self::new()
    }
}

impl FilterService {
    pub fn new() -> Self {
        FilterService {}
    }

    /// It takes a vector of <T> and a limit, and returns a vector that is limited to the
    /// number specified by the limit
    ///
    /// # Arguments:
    ///
    /// * `vector`: A vector.
    /// * `limit`: The number of elements in the vector to return.
    ///
    /// # Returns:
    ///
    /// A vector.

    pub fn limit<T: Clone>(&mut self, vector: &Vec<T>, mut limit: u32) -> Vec<T> {
        if limit > vector.len() as u32 {
            limit = vector.len() as u32;
        }
        vector[0..limit as usize].to_vec()
    }

    /// "Return a subset of the vector, starting at the offset index."
    ///
    /// The first thing we do is check if the offset is greater than the length of the vector. If
    /// it is, we return an error
    ///
    /// # Arguments:
    ///
    /// * `vector`: A vector.
    /// * `offset`: The offset to start from.
    ///
    /// # Returns:
    ///
    /// A vector.

    pub fn offset<T: Clone>(
        &mut self,
        vector: &Vec<T>,
        offset: u32,
    ) -> Result<Vec<T>, FilterServiceError> {
        if offset > vector.len() as u32 {
            return Err(FilterServiceError::OutOfRange);
        }
        Ok(vector[offset as usize..].to_vec())
    }
}
