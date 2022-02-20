//MIT License
//Copyright (c) 2021 Ivan (GitHub: <ivanl-exe>, E-Mail: <ivan.exe@pm.me>)

use read_input::prelude::*;
use std::{
  env::args,
  io::BufReader,
  fs::File
};
use image::{
  imageops::FilterType::Triangle,
  io::Reader,
  DynamicImage,
  ImageFormat
};

//Bourke, P., 1997. Character representation of grey scale images. Paulbourke.net. Available at: <http://paulbourke.net/dataformats/asciiart/>
const GRADIENT: &str = r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>|i!lI;:,^`'. ";

fn main() {
    let (resolution, scaled_gradient, file_path) = unwrap_args();
    let (_image_format, dynamic_image) = open_image(file_path);
    let image_data = resize_to_vec(dynamic_image, resolution);

    let mut image_vector = ImageVector::new();
    image_vector.set(image_data);
    let ascii_image: Vec<char> = image_vector.convert_to_ascii(
      GRADIENT,
      scaled_gradient
    );

    print_ascii_image(
      &ascii_image,
      &resolution
    );
}

//Constant array of input guides in the case of an incorrect number of arguements passed to the file:
const GUIDE_LOG: [&str; 4] = [
  "\nASCII Width: ",
  "\nASCII Height: ",
  "\nScaled Gradient (bool): ",
  "\nFile Path: "
];

fn unwrap_args() -> ([usize; 2], bool, String){
  //Mutable string vector containing CLI passed arguements:
  let mut cli_args: Vec<String> = args().collect();
  cli_args.remove(0);
  //If an incorrect amount of arguements is passed to the file acquire input:
  if cli_args.len() != 4 {
    cli_args = Vec::new();
      for guide in GUIDE_LOG {
        print!("{}", guide);
        cli_args.push(input().get());
      }
  }

  //Return a tuple of parsed and unwrapped arguements:
  ([cli_args[0].parse::<usize>().unwrap(),
  cli_args[1].parse::<usize>().unwrap()],
  cli_args[2].parse::<bool>().unwrap(),
  cli_args[3].to_string())
}

fn open_image(file_path: String) -> (ImageFormat, DynamicImage) {
  //Open the image located in the 'file_path',
  //Return the format, and decoded image:
  let image_reader: Reader<BufReader<File>> = Reader::open(file_path).unwrap();
  (image_reader.format().unwrap(), image_reader.decode().unwrap())
}

fn resize_to_vec(dynamic_image: DynamicImage, resolution: [usize; 2]) -> Vec<u8> {
  //Resize the dynamic image object to the given usize array arguement,
  //Return as a dynamic heap u8 vector:
  dynamic_image.resize_exact(
    resolution[0] as u32,
    resolution[1] as u32,
    Triangle
  ).to_rgba8().to_vec()
}

fn print_ascii_image(ascii_image: &Vec<char>, resolution: &[usize; 2]) -> () {
  //Iterate through the ASCII image, printing each char:
  for index in 0..ascii_image.len() {
    print!("{}", ascii_image[index]);
    //For the end of each row (otherwise a multiple of the width), print a newline special character '\n':
    if (index + 1) % resolution[0] == 0 {
      println!();
    }
  }
}

fn unifrom_gradient(byte: u8, scale: &[u8; 2], gradient: &str) -> char {
  //Linear conversion, maintaing the range ratio for a value:
  //Translate 'byte' from range scale[0], scale[1] to range 0, gradient.len() for indexing into the gradient,
  //scale[0] <= byte <= scale[1] ---> 0 <= index < gradient.len()
  gradient.chars().nth((
    (byte - scale[0]) as usize * (gradient.len() - 1)
    / (scale[1] - scale[0]) as usize
  ).into()).unwrap()
}

fn read_pixel(image_data: &Vec<u8>, index: usize) -> u8 {
  //Mutable unsigned size (architecture dependant) 'pixel':
  let mut pixel: usize = 0;
  //Sum RGBA values for one pixel, as indexed by index:
  for element in index..index + 4 {
    pixel += image_data[element] as usize;
  }
  //Subtract 255 (the maximum u8 value) to effectively reverse the alpha value to correlate to the gradient,
  //Return the average (mean) by dividing by 4 as an 8-bit unsigned interger:
  ((pixel - 255) / 4) as u8
}

/*
enum ImageVectorError {
  BufferInsufficient
}
*/

struct ImageVector {
  image_data: Vec<u8>,
}

impl ImageVector {
  fn new() -> Self {
    //Initalize 'ImageVector' struct with a dynamic vector:
    ImageVector {
      image_data: vec![]
    }
  }

  fn set(&mut self, image_data: Vec<u8>) -> () {
    //Set the interal struct 'image_data' to be that of the passed arguement:
    self.image_data = image_data;
  }

  fn convert_to_ascii(&mut self, gradient: &str, scaled_gradient: bool) -> Vec<char> {
    //An array of the minimum and maximum 8-bit unsigned interger values within the image:
    let mut scale: [u8; 2] = [255, 0];

    //Adjust the image to a vector of 8-bit pixel values:
    let mut adjusted_image: Vec<u8> = vec![];
    let mut index: usize = 0;
    while index < self.image_data.len() {
      adjusted_image.push(
        read_pixel(&self.image_data, index)
      );
      //If a scaled gradient is wanted then proceed into the nested conditions:
      if scaled_gradient == true {
        //If the latest pixel value is below the prior minimum then replace: 
        if adjusted_image[adjusted_image.len() - 1] < scale[0] {
          scale[0] = adjusted_image[adjusted_image.len() - 1];
        }
        //If the latest pixel value is above the prior maximum then replace: 
        else if adjusted_image[adjusted_image.len() - 1] > scale[1] {
          scale[1] = adjusted_image[adjusted_image.len() - 1];
        }
      }
      //Increment by one pixel, otherwise one cycle of RGBA values (4):
      index += 4;
    }
    //If a scaled gradient is unwanted set the scale to the entire 8-bit unsigned interger range:
    if scaled_gradient == false {
      scale = [0, 255];
    }

    let mut ascii_image: Vec<char> = vec![];
    //Unifrom gradient conversion into ASCII chars:
    for index in 0..adjusted_image.len() {
      ascii_image.push(unifrom_gradient(
        adjusted_image[index],
        &scale,
        &gradient
      ));
    }

    ascii_image
  }
}