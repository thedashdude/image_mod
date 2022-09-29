use image::{GenericImageView,Pixel};





pub struct StandardDistribution {
	pub mean: f32,
	pub sd: f32,
}


//Return the max of 3 u8s
fn max_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::max(std::cmp::max(a,b),c)
}
//Return the min of 3 u8s
fn min_3_u8(a: u8,b: u8,c: u8) -> u8 {
	std::cmp::min(std::cmp::min(a,b),c)
}



pub trait Stat {
	const NAME: &'static str;

	fn get(pixel: &image::Rgb<u8>) -> f32;
	fn set(pixel: &mut image::Rgb<u8>, value: f32) -> ();
	//Returns the mean and standard distribution of the statistic requested from an image 
	fn get_image_distribution(img: & image::RgbImage) -> StandardDistribution
	{
		let m = Self::get_image_mean(img);
		let s = Self::get_image_sd(img, m);
		StandardDistribution{ mean:m, sd: s}
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
	fn set_image_distribution(img: &mut image::RgbImage, new_distribution: &StandardDistribution) -> ()
	{

		let old_distribution: StandardDistribution = Self::get_image_distribution(&img);
		//Naive approach
		for (_x,_y,pixel) in img.enumerate_pixels_mut()
		{
			let old_stat = Self::get(pixel);
			let new_stat: f32 = (old_stat - old_distribution.mean)/old_distribution.sd * new_distribution.sd + new_distribution.mean;
			Self::set(pixel, new_stat);
		}
	}
}

pub struct Chroma;
impl Stat for Chroma {
	const NAME: &'static str = "CHROMA";
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
pub struct Luma;
impl Stat for Luma {
	const NAME: &'static str = "LUMA";
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

pub trait ColorIndex : Stat { 
	const INDEX:u8 = 0;
	const COLOR_NAME: &'static str;
}

pub struct Red;
impl ColorIndex for Red {
	const INDEX: u8 = 0;
	const COLOR_NAME: &'static str = "RED";
}

pub struct Green;
impl ColorIndex for Green {
	const INDEX: u8 = 1;
	const COLOR_NAME: &'static str = "GREEN";
}

pub struct Blue;
impl ColorIndex for Blue {
	const INDEX: u8 = 2;
	const COLOR_NAME: &'static str = "BLUE";
}

impl<T> Stat for T where T: ColorIndex {
	const NAME: &'static str = T::COLOR_NAME;
	//Get the Luma statistic of a pixel
	fn get(pixel: &image::Rgb<u8>) -> f32 {
		pixel[Self::INDEX as usize] as f32
	}
	//Set the Luma statistic of a pixel
	fn set(pixel: &mut image::Rgb<u8>, value: f32) -> () {
		pixel[Self::INDEX as usize] = value as u8;
	}
}
