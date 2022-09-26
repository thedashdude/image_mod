#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use image::{ImageBuffer,GenericImage,GenericImageView,Pixel,DynamicImage};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Too few arguments")
	}
	if args.len() == 3 
	{
		let file_name_source = &args[1];
		let file_name_dest = &args[2];
		println!("Reading stats from {}", file_name_source);

		let img_dynamic = image::open(file_name_source).unwrap();
		let img_src = img_dynamic.to_rgb8();

		let (width, height) = img_src.dimensions();
		let num_pixels: i64 = (width as i64) * (height as i64);
		/*
		let (avg, sd) = get_luma_mean_sd(&img_src);
		println!("....Average (Luma): {}", avg);
		println!("....Standard Deviation (Luma): {}", sd);

		println!("Applying to {}", file_name_dest);
		let img_dynamic = image::open(file_name_dest).unwrap();
		let mut img_dest = img_dynamic.to_rgb8();

		apply_luma_dist_functional(&mut img_dest,avg,sd);
		let (avg, sd) = get_luma_mean_sd(&img_dest);
		println!("....Set Average (Luma): {}", avg);
		println!("....Set Standard Deviation (Luma): {}", sd);


		img_dest.save("out.png");
		*/
		let (avg, sd) = get_dist_functional(&img_src, &get_chroma);
		println!("....Average: {}", avg);
		println!("....Standard Deviation: {}", sd);

		println!("Applying to {}", file_name_dest);
		let img_dynamic = image::open(file_name_dest).unwrap();
		let mut img_dest = img_dynamic.to_rgb8();

		apply_dist_functional(&mut img_dest, &get_chroma, &set_chroma, avg, sd); //(&mut img_dest,avg,sd);
		let (avg, sd) = get_dist_functional(&img_dest, &get_chroma);
		println!("....Set Average: {}", avg);
		println!("....Set Standard Deviation: {}", sd);

		img_dest.save("out.png");

		return ();
	}
	else {
		let file_name = &args[1];
		let img_dynamic = image::open(file_name).unwrap();
		let mut img = img_dynamic.to_rgb8();

		let (width, height) = img.dimensions();
		let num_pixels: i64 = (width as i64) * (height as i64);

		let (avg, sd) = get_luma_mean_sd(&img);

		println!("Width x Height: {} x {}", width,height);
		println!("Total Pixels: {}", num_pixels);
		println!("Average (Luma): {}", avg);
		println!("Standard Deviation (Luma): {}", sd);

		apply_luma_dist(&mut img,127.5,75.0);
		img.save("test.png");
	}
}


fn get_dist_functional<F>(img: & image::RgbImage, get_stat: F) -> (f32,f32) where
	F: Fn(&image::Rgb<u8>) -> f32
{
	let m = get_stat_mean(img, &get_stat);
	let s = get_stat_sd(img, &get_stat, m);
	(m,s)
}

fn apply_dist_functional<F,G>(img: &mut image::RgbImage, get_stat: F, set_stat: G, new_mean: f32, new_sd: f32) -> () where
	F: Fn(&image::Rgb<u8>) -> f32, 
	G: Fn(&mut image::Rgb<u8>, f32) -> ()
{
	let (old_mean, old_sd) = get_dist_functional(&img, &get_stat);
	//Naive approach
	for (x,y,pixel) in img.enumerate_pixels_mut()
	{
		let old_stat = get_stat(pixel);
		let new_stat = (old_stat - old_mean)/old_sd * new_sd + new_mean;
		set_stat(pixel, new_stat);
		
		//let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		//let adjust = |x| { ((x as f32 - old_mean)*new_sd/old_sd + new_mean) as u8};
		//*pixel = image::Rgb([adjust(r),adjust(g),adjust(b)]);
		////img.put_pixel(x,y,image::Rgb([1,2,3]));
	}
}

fn get_stat_mean<F>(img: &image::RgbImage, get_stat: F) -> f32 where
	F: Fn(&image::Rgb<u8>) -> f32
{
	let (width, height) = img.dimensions();
	let mut col_avg_sum: f32 = 0.0;
	for n in 0..width 
	{
		let v = img.view(n,0,1,height);
		let mut col_sum: i32 = 0;
		for (_x,_y,pixel) in v.pixels() {
			col_sum += get_stat(&pixel) as i32;
		}
		col_avg_sum += (col_sum as f32) / (height as f32);
	}
	let avg: f32 = col_avg_sum / (width as f32);
	avg
}
fn get_stat_sd<F>(img: &image::RgbImage, get_stat: F, mean: f32) -> f32 where
	F: Fn(&image::Rgb<u8>) -> f32
{
	let (width, height) = img.dimensions();
	let mut dev_avg_sum: f32 = 0.0;
	for n in 0..width 
	{
		let v = img.view(n,0,1,height);
		let mut col_dev_sum: f32 = 0.0;
		for (_x,_y,pixel) in v.pixels() {
			col_dev_sum += (mean - (get_stat(&pixel))).powf(2.0);
		}
		dev_avg_sum += (col_dev_sum as f32) / (height as f32);
	}
	let variance: f32 = dev_avg_sum / (width as f32);
	let sd: f32 = variance.sqrt();
	sd
}

fn apply_luma_dist_functional(img: &mut image::RgbImage, new_mean: f32, new_sd: f32) -> () {
	let (width, height) = img.dimensions();
	let num_pixels: i64 = (width as i64) * (height as i64);
	let (old_mean, old_sd) = get_luma_mean_sd(&img);
	//Naive approach
	for (x,y,pixel) in img.enumerate_pixels_mut()
	{
		let old_luma = get_luma(pixel);
		let new_luma = (old_luma - old_mean)/old_sd * new_sd + new_mean;
		set_luma(pixel, new_luma);
		
		//let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		//let adjust = |x| { ((x as f32 - old_mean)*new_sd/old_sd + new_mean) as u8};
		//*pixel = image::Rgb([adjust(r),adjust(g),adjust(b)]);
		////img.put_pixel(x,y,image::Rgb([1,2,3]));
	}
}

fn apply_luma_dist(img: &mut image::RgbImage, new_mean: f32, new_sd: f32) -> () {
	let (width, height) = img.dimensions();
	let num_pixels: i64 = (width as i64) * (height as i64);
	let (old_mean, old_sd) = get_luma_mean_sd(&img);
	//Naive approach
	for (x,y,pixel) in img.enumerate_pixels_mut()
	{
		let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		let adjust = |x| { ((x as f32 - old_mean)*new_sd/old_sd + new_mean) as u8};
		*pixel = image::Rgb([adjust(r),adjust(g),adjust(b)]);
		//img.put_pixel(x,y,image::Rgb([1,2,3]));
	}
}

fn get_luma_mean(img: &image::RgbImage) -> f32 {
	let (width, height) = img.dimensions();
	let mut col_avg_sum: f32 = 0.0;
	for n in 0..width 
	{
		let v = img.view(n,0,1,height);
		let mut col_sum: i32 = 0;
		for (_x,_y,pixel) in v.pixels() {
			col_sum += pixel.to_luma()[0] as i32;
		}
		col_avg_sum += (col_sum as f32) / (height as f32);
	}
	let avg: f32 = col_avg_sum / (width as f32);
	avg
}

fn get_luma_sd(img: &image::RgbImage, mean: f32) -> f32 {
	let (width, height) = img.dimensions();
	let mut dev_avg_sum: f32 = 0.0;
	for n in 0..width 
	{
		let v = img.view(n,0,1,height);
		let mut col_dev_sum: f32 = 0.0;
		for (_x,_y,pixel) in v.pixels() {
			col_dev_sum += (mean - (pixel.to_luma()[0] as f32)).powf(2.0);
		}
		dev_avg_sum += (col_dev_sum as f32) / (height as f32);
	}
	let variance: f32 = dev_avg_sum / (width as f32);
	let sd: f32 = variance.sqrt();
	sd
}

fn get_luma_mean_sd(img: &image::RgbImage) -> (f32, f32){
	let m = get_luma_mean(img);
	let s = get_luma_sd(img, m);
	(m,s)
}

fn get_luma(pixel: &image::Rgb<u8>) -> f32 {
	pixel.to_luma()[0] as f32
}
fn set_luma(pixel: &mut image::Rgb<u8>, luma: f32) -> () {
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	let old_luma = get_luma(pixel);
	let ratio = luma / old_luma;
	let adjust = |x| {x * ratio};
	*pixel = image::Rgb([adjust(r as f32) as u8,adjust(g as f32) as u8,adjust(b as f32) as u8]);
}


fn get_chroma(pixel: &image::Rgb<u8>) -> f32 {
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	(max_3_u8(r,g,b) - min_3_u8(r,g,b)) as f32
}
fn set_chroma(pixel: &mut image::Rgb<u8>, chroma: f32) -> () {
	//naive approach
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	let max = max_3_u8(r,g,b) as f32;
	let min = min_3_u8(r,g,b) as f32;
	let old_chroma = get_chroma(pixel);
	let mut add = (chroma - old_chroma) / 2.0;
	if old_chroma < 1.0 {
		add = 0.0;
	}

	let adjust = |x| {if x == max {
		x + add
	} else if x == min {
		x - add
	}
	else if old_chroma > 1.0 {
		(x-min)/old_chroma*chroma + min - add
	} else {
		0.0
	}
	};

	*pixel = image::Rgb([adjust(r as f32) as u8,adjust(g as f32) as u8,adjust(b as f32) as u8]);
}

fn max_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::max(std::cmp::max(a,b),c)
}
fn min_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::min(std::cmp::min(a,b),c)
}