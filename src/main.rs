mod stat; 

use std::env;
use stat::{StandardDistribution,Stat,Luma,Chroma,Red,Green,Blue};

const MIN_ARGS: usize = 4;
const MAX_ARGS: usize = 5;

fn main() {
	let args: Vec<String> = env::args().collect();
	if (args.len() == 2 && args[1].to_uppercase().eq("HELP")) || args.len() < 2 {
		println!("\nHELP\n----");
		println!("  Format");
		println!("  ------");
		println!("    file1 file2 filters [file3=out.png]");
		println!("\n  Arguments");
		println!("  ---------");
		println!("    file1   - location of image to read filters from.");
		println!("    file2   - location of image to alter.");
		println!("    filters - String composed using characters from the set (R,G,B,L,C), where case doesn't matter.");
		println!("    file3   - OPTIONAL location to save altered image. Defaults to 'out.png'.");
		println!("\n  Filters");
		println!("  -------");
		println!("    R - Red: Amount of red");
		println!("    G - Green: Amount of green");
		println!("    B - Blue: Amount of blue");
		println!("    L - Luma: Brightness of color");
		println!("    C - Chroma: Vibrancy of color");
		return;
	}
	else if args.len() < MIN_ARGS || args.len() > MAX_ARGS {
		println!("Wrong number of arguments. Expected {} to {}, received {}.\nTry `cargo run help` or `cargo run`.", MIN_ARGS, MAX_ARGS, args.len());
		return;
	}
	else if args.len() >= MIN_ARGS && args.len() <= MAX_ARGS {
		println!("\n\n\n");
		//Load the source and destination images safely
		let file_name_source: &String = &args[1];
		let file_name_dest: &String = &args[2];
		let commands: &String = &args[3].to_uppercase();
		let mut file_name_save: &String = &String::from("out.png");
		if args.len() == 5 {
			file_name_save = &args[4];
		}

		println!("Loading files...");
		println!("  Source: {}",file_name_source);
		let img_dynamic_result = image::open(file_name_source);
		if let Err(_) = img_dynamic_result {
			println!("Failed to open file '{}', terminating...", file_name_source);
			return;
		}
		let img_src = img_dynamic_result.unwrap().to_rgb8();


		println!("  Destination: {}",file_name_dest);
		let img_dynamic_result = image::open(file_name_dest);
		if let Err(_) = img_dynamic_result {
			println!("Failed to open file '{}', terminating...", file_name_dest);
			return;
		}
		let mut img_dest = img_dynamic_result.unwrap().to_rgb8();

		println!("\n\n");
		println!("Running filters - {}...", commands);
		for (i,c) in commands.chars().enumerate() {
			println!("Filter {}/{} - {}",i+1,commands.len(),c);
			match c {
				'R' => user_apply_stat::<Red>(&img_src,&mut img_dest),
				'G' => user_apply_stat::<Green>(&img_src,&mut img_dest),
				'B' => user_apply_stat::<Blue>(&img_src,&mut img_dest),
				'L' => user_apply_stat::<Luma>(&img_src,&mut img_dest),
				'C' => user_apply_stat::<Chroma>(&img_src,&mut img_dest),
				_ => println!("Unrecognized command {}",c),
			}
		} 

		//Save altered destination image to a new file
		println!("Saving to '{}'...", file_name_save);
		let save_result = img_dest.save(file_name_save);
		if let Err(_) = save_result {
			println!("  Failed to save file, terminating...");
			return;
		}
		println!("  Done");


		return;
	}
}

fn user_apply_stat<T: Stat>(src: & image::RgbImage, img: &mut image::RgbImage) -> () {
	println!("Applying stat <{}>",T::NAME);
	println!("  Source:");
	let src_dist: StandardDistribution = T::get_image_distribution(src);
	println!("    Src Average <{}>: {}",T::NAME, src_dist.mean);
	println!("    Src Std.Dev <{}>: {}",T::NAME, src_dist.sd);
	println!("  Destination:");
	let old_dist: StandardDistribution = T::get_image_distribution(img);
	println!("    Old Average <{}>: {}",T::NAME, old_dist.mean);
	println!("    Old Std.Dev <{}>: {}",T::NAME, old_dist.sd);

	T::set_image_distribution(img, &src_dist);

	let new_dist: StandardDistribution = T::get_image_distribution(img);
	println!("    New Average <{}>: {}",T::NAME, new_dist.mean);
	println!("    New Std.Dev <{}>: {}",T::NAME, new_dist.sd);
	println!("\n\n");
}