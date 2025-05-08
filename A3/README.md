
# CSC411 - Assignment 3: Locality
Sofia Mancini
Due: 7th March, 2025

## Part A: Implementation Overview
### What Has Been Correctly Implemented
- Image Transformations
    - Rotations: 90°,, 180°, & 270° clockwise rotations.
    - Flip: Horizontal and vertical mirroring
    - Transpose: Transposing the image across the upper-left to lower-right axis.
- Storage Models:
    - The program supports both row-major and column-major storage options using Array2.
- Command-Line Inputs:
    - The program uses the 'clap' library to parse command-line arguments.
- I/O Handling:
    - The program reads a PPM image from standard input and writes the transformed image to standard output.

## Part B: Architecture of the Solution
### Overview
The solution is structured around the Array2 data structure, which provides a flexible and efficient way to handle 2D arrays with both row-major and column-major storage models. The ppmtrans program leverages this structure to perform various image transformations.

### Key Components
1. Array2 Module:
    - Storage: The Array2 struct stores data in a Vec<T>, allowing for dynamic resizing and efficient access.
    - Iterators: The module provides both row-major and column-major iterators, enabling efficient traversal of the array based on the storage model.
    - Constructors: The module includes constructors for creating Array2 instances from row-major and column-major data, ensuring that the data is correctly mapped to the storage model.

2. Image Transformation Logic:
    - Coordinate Mapping: The map_coords function maps coordinates from the input image to the output image based on the specified transformation (rotation, flip, transpose).
    - Transformation Application: The transform_image function applies the transformation by iterating over the input image using the specified traversal order (row-major or column-major) and mapping each pixel to its new position in the output image.

3. Command-Line Interface:
    - Argument Parsing: The clap library is used to parse command-line arguments, providing a user-friendly interface for specifying transformations and storage models.
    - Input/Output Handling: The program reads PPM images using the csc411_image crate and writes the transformed images to standard output.

### Flow of Execution
1. Argument Parsing: The program starts by parsing command-line arguments to determine the desired transformation and storage model.
2. Image Reading: The program reads the input image from standard input or a specified file.
3. Image Transformation: The program applies the specified transformation by iterating over the input image using the appropriate traversal order and mapping each pixel to its new position in the output image.
4. Image Writing: The program writes the transformed image to standard output.

### Example Usage:
1. Rotate an image 90° clockwise using row-major traversal
    user % ppmtrans --rotate 90 --row-major < input.ppm > output.ppm
2. Flip an image vertically using column-major traversal

    user % ppmtrans --flip vertical --col-major < input.ppm > output.ppm

## Part C:

### Performance Predictions

|                  | Row-Major Access | Column-Major Access  |
| ---------------- |:---------------: | --------------------:|
| 90° Rotation  | Rank 3           | Rank 4               |
| 180° Rotation | Rank 1           | Rank 2               |

### Measured Execution Times

|                  | Row-Major Access | Column-Major Access  |
| ---------------- |:---------------: | --------------------:|
| 90° Rotation  | 3.832s           | 4.419s               |
| 180° Rotation | 3.074s           | 4.783s               |

### Analysis of Results
- 180° Row-Major access was predicted to be the fastest and upon execution proved to be the fastest.
- I predicted that the rotation would have a larger detriment to performance, but the execution times show the access method had a larger impact.

### Causes for Discrepancies
- Cache Effects
    - Column-major traversal may have caused excessive cache misses.
- CPU Behaviour
    - Row-major access may be favored by the CPU's prefetching algorithm.
- Compiler Bias
    - The compiler's built-in optimizations have favour row-major access patterns, leading to worse performance using column-major access.

### Conclusion:
The complexity of the coordinate mapping required for 90° rotations outweighs the cache efficiency differences between row-major and column-major access. While the access method still plays a role (as seen in the faster row-major times), the primary factor influencing performance is the type of rotation being performed.

## Part D:

### Current Layout
- Since Array2 is storing data in row-major order, when a 90-degree rotation is performed, elements are often access in a column-wise method, leading to poor spatial locality and more cache misses.

### Alternative Layout
- Tiled memory storage may improve cache performance. 

### Examples:
A00 A01 A02 A03  
A10 A11 A12 A13  
A20 A21 A22 A23  
A30 A31 A32 A33 

#### In row-major storage: 
'A00 A01 A02 A03 A10 A11 A12 A13 A20 A21 A22 A23 A30 A31 A32 A33'

#### Tiled-storage
A00 A01 A10 A11   |   A02 A03 A12 A13  
A20 A21 A30 A31   |   A22 A23 A32 A33

## Time for Completion:
This assignment took approximately 15 hours of total work to complete. The breakdown is as follows:
- Reading and Understanding the Assignment: 1 hour.
- Initial Design and Estimates: 1 hour.
- Implementing ppmtrans: 10 hours (spread over several days).
    - Learning and integrating the clap library: 3 hours.
    - Debugging and fixing implementation issues:
        - Inital issues 2 hours
        - Gradescope error: 5 hours
            - I tried a lot of complicated solutions before realising my 90° rotation was anticlockwise.
- Parts C & D: 1 hour.
- Writing and Revising the README: 1–2 hours.