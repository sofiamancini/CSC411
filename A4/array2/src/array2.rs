/// A 2D array data structure with generic type elements.
///
/// This provides efficient row-major storage and access to 2D data with bounds checking.
/// Supports both row-major and column-major iteration patterns.
///
/// # Examples
/// ```
/// use array2::Array2;
///
/// // Create from row-major data
/// let data = vec![1, 2, 3, 4];
/// let array = Array2::from_row_major(data, 2, 2).unwrap();
///
/// // Access elements
/// assert_eq!(*array.get(1, 0).unwrap(), 2);
/// ```
pub struct Array2<T> {
    /// Width of the 2D array (number of columns)
    pub width: usize,
    /// Height of the 2D array (number of rows)
    pub height: usize,
    /// Data storage in row-major order (row 0, row 1, etc.)
    pub data: Vec<T>,
}

impl<T> Array2<T> {
    /// Creates a new Array2 from row-major ordered data.
    ///
    /// # Arguments
    /// * `data` - Vector of elements in row-major order
    /// * `width` - Number of columns in the array
    /// * `height` - Number of rows in the array
    ///
    /// # Returns
    /// `Result<Self, String>` where:
    /// - `Ok` contains the new Array2 if dimensions match data length
    /// - `Err` contains an error message if dimensions don't match
    ///
    /// # Errors
    /// Returns an error if `data.len() != width * height`
    pub fn from_row_major(data: Vec<T>, width: usize, height: usize) -> Result<Self, String> {
        if data.len() != width * height {
            return Err(format!(
                "Data length does not match: expected {}, but got {}",
                width * height,
                data.len()
            ));
        }
        Ok(Array2 {
            width,
            height,
            data,
        })
    }

    /// Creates a new Array2 from column-major ordered data.
    ///
    /// # Arguments
    /// * `data` - Vector of elements in column-major order
    /// * `width` - Number of columns in the array
    /// * `height` - Number of rows in the array
    ///
    /// # Returns
    /// `Result<Self, String>` where:
    /// - `Ok` contains the new Array2 if dimensions match data length
    /// - `Err` contains an error message if dimensions don't match
    ///
    /// # Errors
    /// Returns an error if `data.len() != width * height`
    ///
    /// # Notes
    /// Requires `T: Clone` to reorganize the data
    pub fn from_column_major(data: Vec<T>, width: usize, height: usize) -> Result<Self, String>
    where
        T: Clone,
    {
        if data.len() != width * height {
            return Err(format!(
                "Data length does not match: expected {}, but got {}",
                width * height,
                data.len()
            ));
        }

        let mut flattened_data = Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                flattened_data.push(data[y * height + x].clone());
            }
        }
        Ok(Array2 {
            width,
            height,
            data: flattened_data,
        })
    }

    /// Creates a new Array2 filled with a single value.
    ///
    /// # Arguments
    /// * `value` - Value to fill the array with
    /// * `width` - Number of columns in the array
    /// * `height` - Number of rows in the array
    ///
    /// # Returns
    /// A new Array2 filled with `value`
    ///
    /// # Notes
    /// Requires `T: Clone` to duplicate the value
    pub fn fill(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Array2 {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    /// Gets a reference to an element at (x, y) coordinates.
    ///
    /// # Arguments
    /// * `x` - Column index (0-based)
    /// * `y` - Row index (0-based)
    ///
    /// # Returns
    /// `Result<&T, String>` where:
    /// - `Ok` contains a reference to the element if in bounds
    /// - `Err` contains an error message if out of bounds
    ///
    /// # Errors
    /// Returns an error if `x >= width` or `y >= height`
    pub fn get(&self, x: usize, y: usize) -> Result<&T, String> {
        if x < self.width && y < self.height {
            Ok(&self.data[y * self.width + x])
        } else {
            Err(format!(
                "Index out of bounds: ({}, {}) is outside dimensions {}x{}",
                x, y, self.width, self.height
            ))
        }
    }

    /// Sets the value at (x, y) coordinates.
    ///
    /// # Arguments
    /// * `x` - Column index (0-based)
    /// * `y` - Row index (0-based)
    /// * `value` - New value to set
    ///
    /// # Returns
    /// `Result<(), String>` where:
    /// - `Ok` if the coordinates were in bounds
    /// - `Err` contains an error message if out of bounds
    ///
    /// # Errors
    /// Returns an error if `x >= width` or `y >= height`
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), String> {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            Ok(())
        } else {
            Err(format!(
                "Index out of bounds: ({}, {}) is outside dimensions {}x{}",
                x, y, self.width, self.height
            ))
        }
    }

    /// Returns an iterator over elements in row-major order.
    ///
    /// The iterator yields tuples of `(x, y, &T)` where:
    /// - `x` is the column index
    /// - `y` is the row index
    /// - `&T` is a reference to the element
    ///
    /// # Examples
    /// ```
    /// use array2::Array2;
    ///
    /// let array = Array2::fill(0, 2, 2);
    /// for (x, y, val) in array.iter_row_major() {
    ///     println!("({}, {}): {}", x, y, val);
    /// }
    /// ```
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| (x, y, &self.data[y * self.width + x]))
        })
    }

    /// Returns an iterator over elements in column-major order.
    ///
    /// The iterator yields tuples of `(x, y, &T)` where:
    /// - `x` is the column index
    /// - `y` is the row index
    /// - `&T` is a reference to the element
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width).flat_map(move |x| {
            (0..self.height).map(move |y| (x, y, &self.data[y * self.width + x]))
        })
    }

    /// Gets a mutable reference to an element at (x, y) coordinates.
    ///
    /// # Arguments
    /// * `x` - Column index (0-based)
    /// * `y` - Row index (0-based)
    ///
    /// # Returns
    /// `Result<&mut T, String>` where:
    /// - `Ok` contains a mutable reference to the element if in bounds
    /// - `Err` contains an error message if out of bounds
    ///
    /// # Errors
    /// Returns an error if `x >= width` or `y >= height`
    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T, String> {
        if x < self.width && y < self.height {
            Ok(&mut self.data[y * self.width + x])
        } else {
            Err(format!(
                "Index out of bounds: ({}, {}) is outside dimensions {}x{}",
                x, y, self.width, self.height
            ))
        }
    }

    /// Returns a mutable iterator over elements in row-major order.
    ///
    /// The iterator yields tuples of `(x, y, &mut T)` where:
    /// - `x` is the column index
    /// - `y` is the row index
    /// - `&mut T` is a mutable reference to the element
    pub fn iter_row_major_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let width = self.width;
        self.data.iter_mut().enumerate().map(move |(i, v)| {
            let x = i % width;
            let y = i / width;
            (x, y, v)
        })
    }
}