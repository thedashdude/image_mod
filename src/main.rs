use std::env;
use image::{GenericImageView,Pixel};


//Return the max of 3 u8s
fn max_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::max(std::cmp::max(a,b),c)
}
//Return the min of 3 u8s
fn min_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::min(std::cmp::min(a,b),c)
}

trait Stat {
	fn get(pixel: &image::Rgb<u8>) -> f32;
	fn set(pixel: &mut image::Rgb<u8>, value: f32) -> ();
	//Returns the mean and standard distribution of the statistic requested from an image 
	fn get_image_distribution(img: & image::RgbImage) -> (f32,f32)
	{
		let m = Self::get_image_mean(img);
		let s = Self::get_image_sd(img, m);
		(m,s)
	}
	fn get_image_mean(img: &image::RgbImage) -> f32 {
		let (width, height) = img.dimensions();
		let mut col_avg_sum: f32 = 0.0;
		for n in 0..width 
		{
			let v = img.view(n,0,1,height);
			let mut col_sum: i32 = 0;
			for (_x,_y,pixel) in v.pixels() {
				col_sum += Self::get(&pixel) as i32;
			}
			col_avg_sum += (col_sum as f32) / (height as f32);
		}
		let avg: f32 = col_avg_sum / (width as f32);
		avg
	}
	fn get_image_sd(img: &image::RgbImage, mean: f32) -> f32 {
		let (width, height) = img.dimensions();
		let mut dev_avg_sum: f32 = 0.0;
		for n in 0..width 
		{
			let v = img.view(n,0,1,height);
			let mut col_dev_sum: f32 = 0.0;
			for (_x,_y,pixel) in v.pixels() {
				col_dev_sum += (mean - (Self::get(&pixel))).powf(2.0);
			}
			dev_avg_sum += (col_dev_sum as f32) / (height as f32);
		}
		let variance: f32 = dev_avg_sum / (width as f32);
		let sd: f32 = variance.sqrt();
		sd
	}
	fn set_image_distribution(img: &mut image::RgbImage, new_mean: f32, new_sd: f32) -> ()
	{
		let (old_mean, old_sd) = Self::get_image_distribution(&img);
		//Naive approach
		for (_x,_y,pixel) in img.enumerate_pixels_mut()
		{
			let old_stat = Self::get(pixel);
			let new_stat: f32 = (old_stat - old_mean)/old_sd * new_sd + new_mean;
			Self::set(pixel, new_stat);
		}
	}
}

struct Chroma;
impl Stat for Chroma {
	//Get the Chroma statistic of a pixel
	fn get(pixel: &image::Rgb<u8>) -> f32 {
		let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		(max_3_u8(r,g,b) - min_3_u8(r,g,b)) as f32
	}
	//Set the Chroma statistic of a pixel
	fn set(pixel: &mut image::Rgb<u8>, value: f32) -> () {
		//Keeps the ratio (255-MAX : MIN - 0) the same
		let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		let mut max = max_3_u8(r,g,b) as f32;
		let mut min = min_3_u8(r,g,b) as f32;
		let old_chroma = Self::get(pixel);
		let mut old_gap = 255.0-old_chroma;
		

		
		let mut new_gap = 255.0-value;
		//If the ratio is undefined, just fudge it a bit
		if old_gap == 0.0 {
			max -= 1.0;
			min += 1.0;
			old_gap = 2.0;
		}
		//If 0 chroma, keep it that way
		if max == min {
			new_gap = old_gap;
		}

		let new_max = max + (255.0 - max)/old_gap * (old_gap - new_gap);
		let new_min = min - (min - 0.0)/old_gap * (old_gap - new_gap);


		let adjust = |x| {if x == max {
			new_max
		} else if x == min {
			new_min
		} else {
			(x-min)/(max-min) * (new_max - new_min) + new_min
		}
		};

		*pixel = image::Rgb([adjust(r as f32) as u8,adjust(g as f32) as u8,adjust(b as f32) as u8]);
	}

}
struct Luma;
impl Stat for Luma {
	//Get the Luma statistic of a pixel
	fn get(pixel: &image::Rgb<u8>) -> f32 {
		pixel.to_luma()[0] as f32
	}
	//Set the Luma statistic of a pixel
	fn set(pixel: &mut image::Rgb<u8>, value: f32) -> () {
		let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		let old_luma = Self::get(pixel);
		let ratio = value / old_luma;
		let adjust = |x| {x * ratio};
		*pixel = image::Rgb([adjust(r as f32) as u8,adjust(g as f32) as u8,adjust(b as f32) as u8]);
	}
}



fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 || args.len() > 3 {
		panic!("Wrong number of arguments. Expected 3, received {}", args.len());
	}
	if args.len() == 3 {
		println!("\n\n\n");

		//Load the source and destination images safely
		println!("Loading files...");
		let file_name_source = &args[1];
		let file_name_dest = &args[2];

		let img_dynamic_result = image::open(file_name_source);
		if let Err(_) = img_dynamic_result {
			println!("Failed to open file '{}', terminating...", file_name_source);
			return;
		}
		let img_src = img_dynamic_result.unwrap().to_rgb8();



		let img_dynamic_result = image::open(file_name_dest);
		if let Err(_) = img_dynamic_result {
			println!("Failed to open file '{}', terminating...", file_name_dest);
			return;
		}
		let mut img_dest = img_dynamic_result.unwrap().to_rgb8();

		println!("Complete");

		//Reading Stats from the source image
		println!("\n\nReading stats from {}", file_name_source);

		let (avg_l, sd_l) = Luma::get_image_distribution(&img_src);
		println!("....Average (Luma): {}", avg_l);
		println!("....Standard Deviation (Luma): {}", sd_l);

		let (avg_c, sd_c) = Chroma::get_image_distribution(&img_src);
		println!("....Average (Chroma): {}", avg_c);
		println!("....Standard Deviation (Chroma): {}", sd_c);

		//Setting Stats on the destination image
		println!("\n\nApplying Luma to {}", file_name_dest);

		let (old_avg_l, old_sd_l) = Luma::get_image_distribution(&img_dest);
		println!("....Old Average (Luma): {}", old_avg_l);
		println!("....Old Standard Deviation (Luma): {}", old_sd_l);

		Luma::set_image_distribution(&mut img_dest, avg_l, sd_l); //(&mut img_dest,avg,sd);
		let (new_avg_l, new_sd_l) = Luma::get_image_distribution(&img_dest);
		println!("....Set Average (Luma): {}", new_avg_l);
		println!("....Set Standard Deviation (Luma): {}", new_sd_l);

		println!("Applying Chroma to {}", file_name_dest);
		let (old_avg_c, old_sd_c) = Chroma::get_image_distribution(&img_dest);
		println!("....Old Average (Chroma): {}", old_avg_c);
		println!("....Old Standard Deviation (Chroma): {}", old_sd_c);

		Chroma::set_image_distribution(&mut img_dest, avg_c, sd_c); //(&mut img_dest,avg,sd);
		let (new_avg_c, new_sd_c) = Chroma::get_image_distribution(&img_dest);
		println!("....Set Average (Chroma): {}", new_avg_c);
		println!("....Set Standard Deviation (Chroma): {}", new_sd_c);


		//Save altered destination image to a new file
		println!("\n\nSaving to 'out.png'");
		let save_result = img_dest.save("out.png");
		if let Err(_) = save_result {
			println!("Failed to save file, terminating...");
			return;
		}

		return;
	}
}
