# A Rust-Based Text Processing System

This task involves building a proof-of-concept Rust application capable of loading, analyzing and manipulating batch text data. The intention of the task is to assess your understand of Rust's memory safety features, data handling capabilities and software design principles.

As part of this task, you are provided with over 10,000 works of literature, provided in `.txt` format within an AWS S3 bucket. These files are provided in separate `.txt` files, but in order to complete the task, you will be required to demonstrate a system which can handle and parse them from within a single large `.txt` file which will not fit entirely in system memory.

This assignment is split into four separate subtasks, and you will be assessed on how well you solve each of the requirements in each task.

## Task 1: Data Loading and Manipulation

#### Objective
Load a text file, process its content to count the frequency of each word, and store the results in a suitable data structure.

#### Focus
Collections, Borrowing, Lifetimes, and Pointers.

#### Requirements
Efficiently handle large text files without running out of memory.

Use borrowing and lifetimes to manipulate data within collections without unnecessary cloning.

#### Data
- Can be queried from S3 directly through the following links as 

    ```https://diffusion-corpus.s3.eu-west-2.amazonaws.com/1.txt```

    ```https://diffusion-corpus.s3.eu-west-2.amazonaws.com/2.txt```

    ```...```

You should load this data, merge it into a single `.txt` file and work from the single file. There are over 10,000 files in this bucket for processing.

## Task 2: Text Analysis Engine

#### Objective: 
Implement an analysis engine that can perform various analyses on the text, such as finding the n most common words, identifying unique words, and calculating sentence complexity.

#### Focus: 
Functional Language Features, Traits, and Pattern Matching:

- Leverage Rust's functional programming aspects, such as iterators and closures.
- Use traits to define common behavior for text analysis.
- Employ pattern matching to simplify logic for different analysis criteria.

#### Requirements:
- Find the `n` most common words for variable positive integers `n`
- Find the unique words which occur inside the text corpus
- Implement algorithms to determine the complexity of sentences within the corpus, see the following for more details:
https://datascience.stackexchange.com/questions/19452/how-to-determine-the-complexity-of-an-english-sentence

## Task 3: Concurrency in Text Processing

### Objective: 

Modify the analysis engine to process multiple text files concurrently, demonstrating efficient use of Rust's concurrency features.

#### Focus: 
Concurrency and Multithreading.

#### Requirements:

- Implement multithreading to allow processing of multiple files at the same time.
- Ensure thread safety and data consistency without compromising performance.

For the concurrency task, you may work with different files in the S3 bucket concurrently.

## Task 4: Finalizing

#### Objective: 

Design the application with extensibility in mind, allowing for new types of analyses to be easily added.

#### Focus: 
Tooling.

#### Requirements:

- Implement a CLI (Command Line Interface) to interact with the system.
- Include unit and integration tests to cover critical functionalities and concurrency logic.
