
# Design Document: Sudoku Validator

## 1. Overview
This project consists of two main components:
    A. Array2: a polymorphic two-dimensional array implementation using Rust.
    B. Sudoku Validator: A program that checks whether a given Sudoku solution is valid.

The Array2 abstraction will be used to store and manipulate the input data. The Sudoku validator will utilize Array2 to check for validity.

## 2. Array2 Design

### 2.1. Data Structure
    Array2 will be created using a Vec<T> that stores elements in row-major order.

    pub struct Array2<T> {
        width: usize,
        height: usize,
        data: Vec<T>,
    }

### 2.2. Functions
    a. Creates Array2 from row-major ordered vector:
        - row_major(data: Vec<T>, width: usize, height: usize) -> Array2<T> 
    b. Creates Array2 from column-major ordered vector:
        - column_major(data: Vec<T>, width: usize, height: usize) -> Array2<T>
    c. Adds values into Array2
        - fill(value: T, width: usize, height: usize) -> Array2<T>
    d. Get the reference to an element in Array2
        - get(&self, x: usize, y: usize) -> Option<&T>
    e. Updates the value at a location within Array2
        - set(&mut self, x: usize, y: usize, value: T)

## 3. Sudoku Validator

### 3.1. Input Handling
        - The program will read in a .pgm file from csc411_image
        - let img = csc411_image::GrayImage::read(filename).unwrap();
### 3.2. Validation Steps
        a. first check input properties:
            - width == 9 && height == 9
            - denomimator == 9
            - No pixel value 0
        b. Row validation
        c. Column validation
        d. 3x3 grid validation
### 3.3 Exit codes for checking success
        - exit(0): valid solution
        - exit(1): invalid solution

## 4. Design Checklist

### 1. What is the abstract thing you are trying to represent? Often the answer will be in terms of sets, sequences, and finite maps. 
        - A polymorphic two-dimensional matrix of numbers.
### 2. What functions will you offer, and what are the contracts of that those functions must meet? 
        a. is_valid(board) -> bool
            - checks if the given 9x9 matrix is valid
            - contract: input must be a 9x9 grid containing only integers
        b. check_row(board, row) -> bool
            - checks if the given row contains 9 unique integers from 1-9
            contract: row must contain valid integers
        c. check_col(board, col) -> bool
            - checks if the given column contains 9 unique integers from 1-9
            contract: column must contain valid integers
        d. check_grid(board, row, col) -> bool
            - checks if the given 3x3 grid contains unique integers 1-9
            contract: row && col must contain a valid subgrid 
### 3. What examples do you have of what the functions are supposed to do? 
        board = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [4, 5, 6, 7, 8, 9, 1, 2, 3],
            [7, 8, 9, 1, 2, 3, 4, 5, 6],
            [2, 3, 4, 5, 6, 7, 8, 9, 1],
            [5, 6, 7, 8, 9, 1, 2, 3, 4],
            [8, 9, 1, 2, 3, 4, 5, 6, 7],
            [3, 4, 5, 6, 7, 8, 9, 1, 2],
            [6, 7, 8, 9, 1, 2, 3, 4, 5],
            [9, 1, 2, 3, 4, 5, 6, 7, 8]
            ]
        - is_valid(board) expected output: True
        - check_row(board, row)  expected output: True
        - check_col(board, col) expected output: True
        - check_grid(board, row, col)
### 4. What representation will you use, and what invariants will it satisfy? (This question is especially important to answer precisely.) 
        a. Representations: 
            - To store the matrix: Vec<Vec<u8>> 
        b. Invariants:
            - The Board will always be a 9x9 matrix.
            - Each row contains exactly one instance of numbers 1 to 9.
            - Each column contains exactly one instance of numbers 1 to 9.
            - Each subgrid contains exactly one instance of numbers 1 to 9.
            - All values on the board must be within [1,9]

### 5. When a representation satisfies all invariants, what abstract thing from step 1 does it represent? (This question is also especially important to answer precisely.) 
        - When all invariants are satisfied the result will represent a valid Sudoku solution.
### 6. What test cases have you devised? 
        - A valid solution -> should return 0 upon echo$?
        - Duplicate value in row -> should return 1
        - Duplicate value in column -> should return 1
        - Duplicate value in subgrid -> should return 1
        - Invalid value in board -> should return 1
        - Invalid input -> should return 1
### 7. What programming idioms will you need?
        - The idiom for allocating and initializing pointers.
        - The idiom for creating an abstract type using incomplete structures.
        - The idiom of recursive functions for recursive types.