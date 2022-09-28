mod stat; 

use std::env;
use stat::{Stat,Luma,Chroma};

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
