// Create a 2D array with properties width, height, and data
// Using type 'T' so that the array can hold elements of any type
pub struct Array2<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Array2<T> {
    // Constructor for creating an Array2 from row-major order
    pub fn from_row_major(data: Vec<T>, width: usize, height: usize) -> Result<Self, String> {
        // Invariant: The length of the input data must match the product of width and height
        // Ensures that the data can be stored in the 2D array
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

    // Constructor for creating an Array2 from column-major order
    pub fn column_major(data: Vec<T>, width: usize, height: usize) -> Result<Self, String>
    where
        T: Clone,
    {
        // Invariant: The length of the input data must match the product of width and height
        // Ensures that the data can be stored in the 2D array
        if data.len() != width * height {
            return Err(format!(
                "Data length does not match: expected {}, but got {}",
                width * height,
                data.len()
            ));
        }

        let mut flattened_data = Vec::with_capacity(width * height); // Create a vector in row-major order
        for x in 0..width {
            for y in 0..height {
                flattened_data.push(data[y * height + x].clone()); //Copy the data into column-major order
            }
        }
        Ok(Array2 {
            width,
            height,
            data: flattened_data,
        })
    }

    // Fill the Array2 with a specific value
    pub fn fill(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        // Invariant: data vector is initialized with 'width * height' elements of the same value
        // Ensures that the array is fully initialized and ready to use
        Array2 {
            width,
            height,
            data: vec![value; width * height], // width * height represents the number of elements in the array
        }
    }

    // Get the reference to an element by x, y coordinates
    pub fn get(&self, x: usize, y: usize) -> Result<&T, String> {
        // Invariant: x and y must be within the dimensions of the array
        // Avoids out of bounds errors
        if x < self.width && y < self.height {
            Ok(&self.data[y * self.width + x])
        } else {
            Err(format!(
                "Index out of bounds: ({}, {}) is outside dimensions {}x{}",
                x, y, self.width, self.height
            ))
        }
    }

    // Set the value at a specific x, y position
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), String> {
        // Invariant: x and y must be within the dimensions of the array
        // Avoids out of bounds errors
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

    // Row-major iterator
    // (x, y) are the coordinates and &T is the reference to that element
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // Invariant: The iterator must return all elements in the array in row-major order, moving through each row sequentially
        // This ensures that all elements are visited in a predictable order
        (0..self.height).flat_map(move |y| { // clever bit of code that transforms the row index (y) into an iterator to loop through the columns in that row
            (0..self.width).map(move |x| (x, y, &self.data[y * self.width + x])) //the math works out so that: y * width moves to the start of that row, then +x moves to the column within that row
        })
    }

    // Column-major iterator
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // Invariant: The iterator must return all elements in the array in column-major order, moving through each column sequentially
        // This ensures that all elements are visited in a predictable order
        (0..self.width).flat_map(move |x| {
            (0..self.height).map(move |y| (x, y, &self.data[y * self.width + x]))
        })
    }
}
