mod stat; 

use std::env;
use stat::{StandardDistribution,Stat,Luma,Chroma,ColorIndex,Red,Green,Blue};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 || args.len() > 3 {
		panic!("Wrong number of arguments. Expected 3, received {}", args.len());
	}
	if args.len() == 3 {
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
		println!("\n\nReading stats from {}", file_name_source);

		let dist_r: StandardDistribution = Red::get_image_distribution(&img_src);
		println!("....Average (R): {}", dist_r.mean);
		println!("....Standard Deviation (R): {}", dist_r.sd);
		let dist_g: StandardDistribution = Green::get_image_distribution(&img_src);
		println!("....Average (G): {}", dist_g.mean);
		println!("....Standard Deviation (G): {}", dist_g.sd);
		let dist_b: StandardDistribution = Blue::get_image_distribution(&img_src);
		println!("....Average (B): {}", dist_b.mean);
		println!("....Standard Deviation (B): {}", dist_b.sd);

		//Setting Stats on the destination image
		println!("\n\nReading stats from {}", file_name_dest);

		let dist_r_old: StandardDistribution = Red::get_image_distribution(&img_dest);
		println!("....Old Average (R): {}", dist_r_old.mean);
		println!("....Old Standard Deviation (R): {}", dist_r_old.sd);
		let dist_g_old: StandardDistribution = Green::get_image_distribution(&img_dest);
		println!("....Old Average (G): {}", dist_g_old.mean);
		println!("....Old Standard Deviation (G): {}", dist_g_old.sd);
		let dist_b_old: StandardDistribution = Blue::get_image_distribution(&img_dest);
		println!("....Old Average (B): {}", dist_b_old.mean);
		println!("....Old Standard Deviation (B): {}", dist_b_old.sd);

		println!("\n\nApplying Colors to {}", file_name_dest);
		
		Red::set_image_distribution(&mut img_dest, &dist_r);
		Green::set_image_distribution(&mut img_dest, &dist_g);
		Blue::set_image_distribution(&mut img_dest, &dist_b);

		let dist_r_new: StandardDistribution = Red::get_image_distribution(&img_dest);
		println!("....New Average (R): {}", dist_r_new.mean);
		println!("....New Standard Deviation (R): {}", dist_r_new.sd);
		let dist_g_new: StandardDistribution = Green::get_image_distribution(&img_dest);
		println!("....New Average (G): {}", dist_g_new.mean);
		println!("....New Standard Deviation (G): {}", dist_g_new.sd);
		let dist_b_new: StandardDistribution = Blue::get_image_distribution(&img_dest);
		println!("....New Average (B): {}", dist_b_new.mean);
		println!("....New Standard Deviation (B): {}", dist_b_new.sd);


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
