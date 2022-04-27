#![deny(missing_docs)]
//! Creates a png image from text.
//! Useful to protect sensitive information from bots and scrapers,
//! such as email addresses and telephone numbers in web pages,
//! and even passwords sent via email
//!
//! - [x] Png Support 8 bit rgba
//! - [x] Font Weight implemented as Random pixels
//! - [ ] Png Support 1 bit monochrome -  to reduce image size
//! - [ ] Poisson Disk rendering

use clap::Parser;
use std::process;
// use glyph_brush_layout::{ab_glyph::*, *};
use text2img::text2img_internal;


/// Command Line interface to text2img
fn main() //-> anyhow::Result<()>
{
	let args = Args::parse();

	match text2img_internal(args.text, args.weight,args.size)
	{
		Ok(dt) => dt.write_png(args.output).expect("Error writing image file"),
		Err(msg) => {
			eprintln!("{}",msg);
			process::exit(exitcode::USAGE);
		}
	}
// 	Ok(())
	//Not needed: process::exit(exitcode::OK);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Point size of text
	#[clap(short, long, default_value_t = 20)]
	size: u8,

	/// Weight 1 to 100
	#[clap(short, long, default_value_t = 100)]
	weight: u8,

	/// Output filename. Currently only png format is supported
	#[clap(short, long)]
	output: String,

	/// Text to render
	text: String,
}

