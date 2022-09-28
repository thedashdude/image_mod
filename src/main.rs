use std::env;
use image::{GenericImageView,Pixel};




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

		let (avg_l, sd_l) = get_dist_functional(&img_src, &get_luma);
		println!("....Average (Luma): {}", avg_l);
		println!("....Standard Deviation (Luma): {}", sd_l);

		let (avg_c, sd_c) = get_dist_functional(&img_src, &get_chroma);
		println!("....Average (Chroma): {}", avg_c);
		println!("....Standard Deviation (Chroma): {}", sd_c);

		//Setting Stats on the destination image
		println!("\n\nApplying Luma to {}", file_name_dest);

		let (old_avg_l, old_sd_l) = get_dist_functional(&img_dest, &get_luma);
		println!("....Old Average (Luma): {}", old_avg_l);
		println!("....Old Standard Deviation (Luma): {}", old_sd_l);

		apply_dist_functional(&mut img_dest, &get_luma, &set_luma, avg_l, sd_l); //(&mut img_dest,avg,sd);
		let (new_avg_l, new_sd_l) = get_dist_functional(&img_dest, &get_luma);
		println!("....Set Average (Luma): {}", new_avg_l);
		println!("....Set Standard Deviation (Luma): {}", new_sd_l);

		println!("Applying Chroma to {}", file_name_dest);
		let (old_avg_c, old_sd_c) = get_dist_functional(&img_dest, &get_chroma);
		println!("....Old Average (Chroma): {}", old_avg_c);
		println!("....Old Standard Deviation (Chroma): {}", old_sd_c);

		apply_dist_functional(&mut img_dest, &get_chroma, &set_chroma, avg_c, sd_c); //(&mut img_dest,avg,sd);
		let (new_avg_c, new_sd_c) = get_dist_functional(&img_dest, &get_chroma);
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

//Returns the mean and standard distribution of the statistic requested from an image 
fn get_dist_functional<F>(img: & image::RgbImage, get_stat: F) -> (f32,f32) where
	F: Fn(&image::Rgb<u8>) -> f32
{
	let m = get_stat_mean(img, &get_stat);
	let s = get_stat_sd(img, &get_stat, m);
	(m,s)
}

//Applys the mean and standard distribution of the statistic given on an image (imperfect due to the forced bounding on (0..=255) of u8)
fn apply_dist_functional<F,G>(img: &mut image::RgbImage, get_stat: F, set_stat: G, new_mean: f32, new_sd: f32) -> () where
	F: Fn(&image::Rgb<u8>) -> f32, 
	G: Fn(&mut image::Rgb<u8>, f32) -> ()
{
	let (old_mean, old_sd) = get_dist_functional(&img, &get_stat);
	//Naive approach
	for (_x,_y,pixel) in img.enumerate_pixels_mut()
	{
		let old_stat = get_stat(pixel);
		let new_stat: f32 = (old_stat - old_mean)/old_sd * new_sd + new_mean;
		set_stat(pixel, new_stat);
		
		//let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
		//let adjust = |x| { ((x as f32 - old_mean)*new_sd/old_sd + new_mean) as u8};
		//*pixel = image::Rgb([adjust(r),adjust(g),adjust(b)]);
		////img.put_pixel(x,y,image::Rgb([1,2,3]));
	}
}

//Returns the mean of the statistic requested from an image 
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

//Returns the standard deviation of the statistic requested from an image 
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


//Get the Luma statistic of a pixel
fn get_luma(pixel: &image::Rgb<u8>) -> f32 {
	pixel.to_luma()[0] as f32
}
//Set the Luma statistic of a pixel
fn set_luma(pixel: &mut image::Rgb<u8>, luma: f32) -> () {
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	let old_luma = get_luma(pixel);
	let ratio = luma / old_luma;
	let adjust = |x| {x * ratio};
	*pixel = image::Rgb([adjust(r as f32) as u8,adjust(g as f32) as u8,adjust(b as f32) as u8]);
}

//Get the Chroma statistic of a pixel
fn get_chroma(pixel: &image::Rgb<u8>) -> f32 {
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	(max_3_u8(r,g,b) - min_3_u8(r,g,b)) as f32
}
//Set the Chroma statistic of a pixel
fn set_chroma(pixel: &mut image::Rgb<u8>, new_chroma: f32) -> () {
	//Keeps the ratio (255-MAX : MIN - 0) the same
	let (r,g,b) = (pixel[0],pixel[1],pixel[2]);
	let mut max = max_3_u8(r,g,b) as f32;
	let mut min = min_3_u8(r,g,b) as f32;
	let old_chroma = get_chroma(pixel);
	let mut old_gap = 255.0-old_chroma;
	

	
	let mut new_gap = 255.0-new_chroma;
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


//Return the max of 3 u8s
fn max_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::max(std::cmp::max(a,b),c)
}
//Return the min of 3 u8s
fn min_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::min(std::cmp::min(a,b),c)
}