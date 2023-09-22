use std::error::Error;
use std::fs::DirEntry;
use std::u32;
use std::fs;
use std::fmt;

use image::{self, open, ImageError, Pixel};
use rayon::prelude::*;

pub mod config;
pub mod benchmark;


#[derive(Debug, Clone)]
pub struct MinMaxValues {
    pub min: u32,
    pub max: u32,
}

#[derive(Clone)]
pub struct PixelDescription {
    pub r_avg: f32,
    pub g_avg: f32,
    pub b_avg: f32,
    pub r_val: MinMaxValues,
    pub g_val: MinMaxValues,
    pub b_val: MinMaxValues,
}

#[derive(Debug, Clone)]
pub struct DatasetPixelDescription {
    pub r_avg: f32,
    pub g_avg: f32,
    pub b_avg: f32,
    pub r_std: f32,
    pub g_std: f32,
    pub b_std: f32,
}

#[derive(Debug, Clone)]
pub struct DatasetDescription {
    pub pixels_description: DatasetPixelDescription,
    pub images_height: MinMaxValues,
    pub images_length: MinMaxValues,
    pub size: usize,
}


impl fmt::Display for DatasetDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "
    - pixels description:
        - red average: {}
        - green average: {}
        - blue average: {}

        - red std: {}
        - green std: {}
        - blue std: {}
    
    - images height min/max: {}/{}
    - images length min/max: {}/{}
    - dataset size: {}

",
            self.pixels_description.r_avg,
            self.pixels_description.g_avg,
            self.pixels_description.b_avg,
            self.pixels_description.r_std,
            self.pixels_description.g_std,
            self.pixels_description.b_std,
            self.images_height.min,
            self.images_height.max,
            self.images_length.min,
            self.images_length.max,
            self.size,
        )
    }
}


impl PixelDescription {
    fn empty() -> PixelDescription {
        PixelDescription { 
            r_avg: 0., 
            g_avg: 0., 
            b_avg: 0., 
            r_val: MinMaxValues { min: 0, max: 0 }, 
            g_val: MinMaxValues { min: 0, max: 0 }, 
            b_val: MinMaxValues { min: 0, max: 0 },
        }
    }
}


impl MinMaxValues {
    fn empty() -> MinMaxValues {
        MinMaxValues { min: 0, max: 0 }
    }
}


pub fn get_pixels_description(image_path: &str) -> Result<PixelDescription, ImageError> {
    let image_rgb = match open(image_path) {
        Ok(dynamic_image) => dynamic_image.to_rgb8(),
        Err(err) => return Err(err),
    };

    let (height, length) =  image_rgb.dimensions();
    let (mut r_val_sum, mut g_val_sum, mut b_val_sum) = (0, 0, 0);
    let (mut max_r_val, mut max_g_val, mut max_b_val) = (0, 0, 0);
    let (mut min_r_val, mut min_g_val, mut min_b_val) = (u32::MAX, u32::MAX, u32::MAX);

    for row in 0..height {
        for col in 0..length {
            let r_val = image_rgb.get_pixel(row, col).channels()[0] as u32;
            let g_val = image_rgb.get_pixel(row, col).channels()[1] as u32;
            let b_val = image_rgb.get_pixel(row, col).channels()[2] as u32;

            r_val_sum += r_val;
            g_val_sum += g_val;
            b_val_sum += b_val;
            
            max_r_val = if max_r_val < r_val {r_val} else {max_r_val};
            max_g_val = if max_g_val < g_val {g_val} else {max_g_val};
            max_b_val = if max_b_val < b_val {b_val} else {max_b_val};

            min_r_val = if min_r_val > r_val {r_val} else {min_r_val};
            min_g_val = if min_g_val > g_val {g_val} else {min_g_val};
            min_b_val = if min_b_val > b_val {b_val} else {min_b_val};
        }
    }

    let num_pixels_per_channel = height * length;

    Ok(PixelDescription{
        r_avg: r_val_sum as f32 / num_pixels_per_channel as f32,
        g_avg: g_val_sum as f32 / num_pixels_per_channel as f32,
        b_avg: b_val_sum as f32 / num_pixels_per_channel as f32,
        r_val: MinMaxValues { min: min_r_val, max: max_r_val },
        g_val: MinMaxValues { min: min_g_val, max: max_g_val },
        b_val: MinMaxValues { min: min_b_val, max: max_b_val },
    })
}


pub fn get_dataset_description(root_dir: String, trackit: bool)  -> Result<DatasetDescription, Box<dyn Error>> {
    let images_paths = visit_dirs(
        root_dir, 
        vec![
            String::from("jpg"),
            String::from("png"),
        ]
    )?;
    
    let mut height_info = MinMaxValues{min: u32::MAX, max: 0};
    let mut length_info = MinMaxValues{min: u32::MAX, max: 0};

    if trackit {
        println!("Collecting images sizes information...");
    }
    
    for image_path in images_paths.iter() {
        let image_dimensions_result = image::image_dimensions(image_path.path());

        match image_dimensions_result {
            Ok((image_height, image_length)) => {
                height_info.min = if height_info.min < image_height {height_info.min} else {image_height};
                height_info.max = if height_info.max > image_height {height_info.max} else {image_height};
                length_info.min = if length_info.min < image_length {length_info.min} else {image_length};
                length_info.max = if length_info.max > image_length {length_info.max} else {image_length};
            },
            Err(err) => eprintln!("Problem with extracting the image dimensions {err}"),
        }
    }

    if trackit {
        println!("Collecting images pixels information...");
    }
    let pixels_description_collection: Vec<PixelDescription> = images_paths
        .into_par_iter()
        .map(|image_path| get_pixels_description(image_path.path().to_str().unwrap()).unwrap())
        .collect();
    

    let reduce_result = pixels_description_collection
        .clone()
        .into_par_iter()
        .reduce(
            || PixelDescription::empty(), 
            |a, b| {
                PixelDescription { 
                    r_avg: a.r_avg + b.r_avg, 
                    g_avg: a.g_avg + b.g_avg, 
                    b_avg: a.b_avg + b.b_avg, 
                    r_val: MinMaxValues::empty(),  
                    g_val: MinMaxValues::empty(), 
                    b_val: MinMaxValues::empty(),
                }
            } 
        );

        let (r_sum, g_sum, b_sum) = (reduce_result.r_avg, reduce_result.g_avg, reduce_result.b_avg);

    let r_dataset_avg = r_sum / pixels_description_collection.len() as f32;
    let g_dataset_avg = g_sum / pixels_description_collection.len() as f32;
    let b_dataset_avg = b_sum / pixels_description_collection.len() as f32;
    

    if trackit {
        println!("Aggregating pixel information...");
    }
    let folded_result = pixels_description_collection
        .clone()
        .into_par_iter()
        .fold(
            || (0_f32, 0_f32, 0_f32), 
            |acc, elem: PixelDescription| {
                (
                    acc.0 + f32::powf(elem.r_avg - r_dataset_avg, 2.0),
                    acc.1 + f32::powf(elem.g_avg - g_dataset_avg, 2.0),
                    acc.2 + f32::powf(elem.b_avg - b_dataset_avg, 2.0),
                )
            }
        );

    let (mut r_std, mut g_std, mut b_std) = folded_result.reduce(
        || (0_f32, 0_f32, 0_f32), 
        |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b. 2) 
    );

    r_std = (r_std / pixels_description_collection.len() as f32).sqrt();
    g_std = (g_std / pixels_description_collection.len() as f32).sqrt();
    b_std = (b_std / pixels_description_collection.len() as f32).sqrt();


    return Ok(
        DatasetDescription { 
            pixels_description: DatasetPixelDescription{
                r_avg: r_dataset_avg,
                g_avg: g_dataset_avg,
                b_avg: b_dataset_avg,
                r_std,
                g_std,
                b_std,
            }, 
            images_height: height_info, 
            images_length: length_info,
            size: pixels_description_collection.len(),
        }
    );
}


fn visit_dirs(root_dir: String, image_formats: Vec<String>) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut images_paths = vec![];
    let mut unvisited_directories = vec![root_dir];

    while let Some(directory) = unvisited_directories.pop() {
        let paths: fs::ReadDir = match fs::read_dir(directory.clone()) {
            Ok(paths) => paths,
            Err(err) => {return Err(format!("{err}. Root directory: \"{}\"", directory).into())},
        };
        
        for entry_result in paths {
            let entry = entry_result?;
            let path = entry.path();
            if path.is_dir() {
                unvisited_directories.push(path.into_os_string().into_string().unwrap());
            } else {
                let file_name_osstring = entry.file_name();
                let file_name = file_name_osstring.to_str().unwrap();
                if image_formats.iter().any(|image_format| file_name.ends_with(image_format)) {
                    images_paths.push(entry);
                }
            }
        }
    }

    Ok(images_paths)
}


#[cfg(test)]#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_correct_pixel_averages() {
		let image_path = "./assets/test_dataset/tiger.jpg";

		let pixels_info = get_pixels_description(image_path).unwrap();
		
		assert_eq!(pixels_info.r_avg, 63.89522);
		assert_eq!(pixels_info.g_avg, 48.269913);
		assert_eq!(pixels_info.b_avg, 43.222286);	
    }

    #[test]
    fn get_correct_dataset_pixel_description() {
        let dataset_description = get_dataset_description("assets/test_dataset".to_owned(), false).unwrap();

        assert_eq!(dataset_description.images_height.max, 2667);
        assert_eq!(dataset_description.images_height.min, 720);
        assert_eq!(dataset_description.images_length.max, 4000);
        assert_eq!(dataset_description.images_length.min, 896);

        assert_eq!(dataset_description.size, 2);
        
        assert_eq!(dataset_description.pixels_description.r_avg, 51.43689);
        assert_eq!(dataset_description.pixels_description.g_avg, 40.53276);
        assert_eq!(dataset_description.pixels_description.b_avg, 41.67968);	
        
        assert_eq!(dataset_description.pixels_description.r_std, 12.45833);
        assert_eq!(dataset_description.pixels_description.g_std, 7.737152);
        assert_eq!(dataset_description.pixels_description.b_std, 1.5426064);
    }
}
