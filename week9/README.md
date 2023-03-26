# Image Recognition Project

This Rust project loads an image file and identifies the objects in the image, returning their labels and confidence scores.

## Prerequisites

Before you can run this project, you'll need to install Rust and its package manager, Cargo. You can find instructions on how to install Rust and Cargo on the [official Rust website](https://www.rust-lang.org/tools/install).

## Getting Started

To build the project with Cargo, you could type:

```
cargo build
```
This will download any required dependencies and compile the project. If the build succeeds, you can run the project with the following command:
```
cargo run cat.jpg
```
the result would be:
```
"cat": 0.8
"chair": 0.6
```

Yout can replace cat.jpg with the path to the image file you want to analyze.

The program will output the labels and confidence scores for each identified object in the image.

## Customizing the Object Detection Algorithm
The get_object_labels function in main.rs currently returns a placeholder result. To implement your own object detection algorithm, you can replace the get_object_labels function with your own implementation.

The get_object_labels function takes an RgbImage object as input and returns a vector of Object structs, where each struct represents an identified object in the image. Each Object struct has a label field that contains the name of the object and a confidence field that represents the algorithm's confidence in the object's identification.

## Contributing
If you find any issues with this project or have suggestions for improvements, feel free to open an issue or submit a pull request on the GitHub repository.
