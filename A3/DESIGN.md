# Design Document: ppmtrans

## 1. Abstract Representation

The ppmtrans program will manipulate an RGB image using the csc411_image library. These images will be represented as two-dimensional arrays using the array2 library. The abstract representation of this image is a finite map from coordinates (x, y) to pixel values (r, g, b) where:
    - (x, y) are the coordinates of a pixel in the image
    - (r, g, b) are the red, green, and blue components of the pixel.

The program will support such transformations as rotation, flipping, and transposition, all of which modify the mapping coordinates.

## 2. Functions and Contracts

### Core Functions
    1. read_ppm(input: &str) -> Array2<pixel>
        - Precondition: input is a valid file path or stdin containing a ppm image.
        - Postcondition: returns an Array2<pixel> representation of the image
    2. write_ppm(image: Array2<pixel>, output: &str)
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: writes the image to output (file or stdout)
    3. transform_ppm(imapge: Array2<pixel>, operation: TransformOperation) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>, and operation is a valid transformation
        - Postcondition: returns a new Array2<pixel> representation of the transformed image.

### Transformation Functions
    1. rotate_90(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image rotated 90 degrees (clockwise)
    2. rotate_180(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image rotated 180 degrees
    3. rotate_270(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image rotated 270 degrees (clockwise) (or 90 degrees anticlockwise -> might be easier)
    4. rotate_0(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image unchanged
    5. flip_horizontal(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image mirrored horizontally
    6. flip_vertical(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image mirrored vertically
    7. transpose(image: Array2<pixel>) -> Array2<pixel>
        - Precondition: image is a valid Array2<pixel>
        - Postcondition: returns the image transposed across the upper-left to lower-right diagonal

## 3. Examples of Function Behavior

1. Rotate 90 Degrees Clockwise
    - input: a 2x2 image:
        (0,0): (1, 2, 3) (0,1): (4, 5, 6)
        (1, 0): (7, 8, 9) (1, 1): (10, 11, 12)
    - output: a 2x2 image:
        (0,0): (7, 8, 9) (0,1): (1, 2, 3)
        (1, 0): (10, 11, 12) (1,1): (4, 5, 6)
2. Flip Horizontally
    - input: a 2x2 image:
        (0,0): (1, 2, 3) (0,1): (4, 5, 6)
        (1, 0): (7, 8, 9) (1, 1): (10, 11, 12)
    - output: a 2x2 image:
        (0,0): (4, 5, 6) (0,1): (1, 2, 3)
        (1, 0): (10, 11, 12) (1, 1): (7, 8, 9)

## 4. Representation and Invariants

### Representation
    - The image is represented using the Array2 data structure, which supports row-major and column-major storage.
    - The storage model (row vs column) is determined at initialization and remains consistent throughout.

### Invariants
    - Image Invariant:
        - For any pixel at (x, y) in the input image, the transformed image will have the pixel value at a new coordinate (x', y') determined by the transformation function.
        -> ex: For a 90-degree rotation, (x, y) in the input maps to (y, width - x -1) in the output.
    - Storage Invariant:
        - The Array2 storage model (row vs column) is preserved during the transformation.
    - Iterator Invariant: 
        - The program uses iter_row_major or iter_col_major to traverse the image, ensuring that the traversal order matches the storage model.

## 5. Abstract Thing Represented

When the representation satifies all invariants, it represents a valid RGB image with pixel values mapped to coordinates according to the specified transformation.

## 6. Test Cases

### Test Case 1: Rotate 90 Degrees
    - input: a 2x2 image:
        (0,0): (1, 2, 3) (0,1): (4, 5, 6)
        (1, 0): (7, 8, 9) (1, 1): (10, 11, 12)
    - expected output: a 2x2 image:
        (0,0): (7, 8, 9) (0,1): (1, 2, 3)
        (1, 0): (10, 11, 12) (1,1): (4, 5, 6)

### Test Case 2: Flip Horizontally
    - input: a 2x2 image:
        (0,0): (1, 2, 3) (0,1): (4, 5, 6)
        (1, 0): (7, 8, 9) (1, 1): (10, 11, 12)
    - expected output: a 2x2 image:
        (0,0): (4, 5, 6) (0,1): (1, 2, 3)
        (1, 0): (10, 11, 12) (1, 1): (7, 8, 9)

### Test Case 3: Nonexistent Operation
    - input: --rotate 45
    - expected output: message to stderr and exit

## 7. Programming Idioms

1. Iterators:
    - Use iter_row_major and iter_col_major for traversal, avoiding nested loops.
2. Procedural Abstraction:
    - Encapsulate transformations into separate functions
3. Error Handling:
    - Use assert! and explicit checks to ensure preconditions and invariants are satisfied.

## 8. Modular Architecture

The program is divided into the following modules:
    1. ppm_io: Handles the reading/writing of ppm images
    2. transformations: Handles the image transformations
    3. array2: Library that holds the Array2 data structure and iterators.
    4. main: Handles the command-line arguments and facilities the program flow.

## 9. Subproblems and Reusable Components

### Reusable Components
    1. Coordinate Mapping:
        - A reusable function map_coordinates(x: usize, y: usize, operation: TransformOperation) -> (usize, usize) will abstract the logic for determining new coordinates after a transformation. This function will be used by all transformations.
        - Ex: 
            fn map_coordinates(x: usize, y: usize, operation: TransformOperation) -> (usize, usize) {
                match operation {
                    TransformOperation::Rotate90 => (y, width - x - 1),
                    TransformOperation::FlipHorizontal => (width - x - 1, y),
                    // other transformations...
                }
            }
    2. Pixel Copying:
        - A reusable function copy_pixel(src: &Array2<pixel>, dest: &mut Array2<pixel>, src_x: usize, src_y: usize, dest_x: usize, dest_y: usize) will handle all pixel copying logic from source image to destination. This will avoid redundancy in the transformation functions.
    3. Image Traversal:
        - a transform_image(image: Array2<pixel>, operation: TransformOperation, iterator: IteratorType) will abstract the traversal method, allowing the same function to work with either row-major or column-major iterators.
    4. Transformation Validation:
        - A reusable function validate_transformation(image: &Array2<Pixel>, operation: TransformOperation) -> Result<(), TransformationError> will check for issues with the given input (i.e. checking dimensions)

### Abstracted Subproblems
    1. Rotate 270 Degrees:
        - It can reuse rotate_90 by calling it three times or by adjusting the rotation to move instead anticlockwise.
        - ex:
        fn rotate_270(image: Array2<Pixel>) -> Array2<Pixel> {
            let mut result = image;
            for _ in 0..3 {
                result = rotate_90(result);
            }
            result
        }
    2. Flip and Transpose:
        - transpose function can reuse logic from flip_horizontal and flip_vertical to simplify implementation.

## 10. Example Input and Output

Input:
P3 -- rotate_90
2 2
255
1 2 3   4 5 6
7 8 9   10 11 12

Output:
P3
2 2
255
7 8 9   1 2 3
10 11 12   4 5 6
