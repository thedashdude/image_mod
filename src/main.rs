mod stat; 

use std::env;
use stat::{StandardDistribution,Stat,Luma,Chroma,Red,Green,Blue};

const NUM_ARGS: usize = 4;

fn user_apply_stat<T: Stat>(src: & image::RgbImage, img: &mut image::RgbImage) -> () {
	println!("Applying stat <{}>",T::NAME);
	println!("  Source:");
	let src_dist: StandardDistribution = T::get_image_distribution(src);
	println!("    Src Average ({}): {}",T::NAME, src_dist.mean);
	println!("    Src Std.Dev ({}): {}",T::NAME, src_dist.sd);
	println!("  Modified:");
	let old_dist: StandardDistribution = T::get_image_distribution(img);
	println!("    Old Average ({}): {}",T::NAME, old_dist.mean);
	println!("    Old Std.Dev ({}): {}",T::NAME, old_dist.sd);

	T::set_image_distribution(img, &src_dist);

	let new_dist: StandardDistribution = T::get_image_distribution(img);
	println!("    New Average ({}): {}",T::NAME, new_dist.mean);
	println!("    New Std.Dev ({}): {}",T::NAME, new_dist.sd);
	println!("\n\n");
}


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < NUM_ARGS || args.len() > NUM_ARGS {
		panic!("Wrong number of arguments. Expected {}, received {}.", NUM_ARGS, args.len());
	}
	if args.len() == NUM_ARGS {
		// RGB adjust???
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
		user_apply_stat::<Red>(&img_src,&mut img_dest);
		user_apply_stat::<Green>(&img_src,&mut img_dest);
		user_apply_stat::<Blue>(&img_src,&mut img_dest);

		//Save altered destination image to a new file
		println!("\n\nSaving to 'out.png'");
		let save_result = img_dest.save("out.png");
		if let Err(_) = save_result {
			println!("Failed to save file, terminating...");
			return;
		}

		return;
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

		let dist_luma: StandardDistribution = Luma::get_image_distribution(&img_src);
		println!("....Average (Luma): {}", dist_luma.mean);
		println!("....Standard Deviation (Luma): {}", dist_luma.sd);

		let dist_chroma: StandardDistribution = Chroma::get_image_distribution(&img_src);
		println!("....Average (Chroma): {}", dist_chroma.mean);
		println!("....Standard Deviation (Chroma): {}", dist_chroma.sd);

		//Setting Stats on the destination image
		println!("\n\nApplying Luma to {}", file_name_dest);

		let dist_luma_old: StandardDistribution = Luma::get_image_distribution(&img_dest);
		println!("....Old Average (Luma): {}", dist_luma_old.mean);
		println!("....Old Standard Deviation (Luma): {}", dist_luma_old.sd);

		Luma::set_image_distribution(&mut img_dest, &dist_luma); //(&mut img_dest,avg,sd);
		let dist_luma_new: StandardDistribution = Luma::get_image_distribution(&img_dest);
		println!("....Set Average (Luma): {}", dist_luma_new.mean);
		println!("....Set Standard Deviation (Luma): {}", dist_luma_new.sd);

		println!("Applying Chroma to {}", file_name_dest);
		let dist_chroma_old: StandardDistribution = Chroma::get_image_distribution(&img_dest);
		println!("....Old Average (Chroma): {}", dist_chroma_old.mean);
		println!("....Old Standard Deviation (Chroma): {}", dist_chroma_old.sd);

		Chroma::set_image_distribution(&mut img_dest, &dist_chroma); //(&mut img_dest,avg,sd);
		let dist_chroma_new: StandardDistribution = Chroma::get_image_distribution(&img_dest);
		println!("....Set Average (Chroma): {}", dist_chroma_new.mean);
		println!("....Set Standard Deviation (Chroma): {}", dist_chroma_new.sd);


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
