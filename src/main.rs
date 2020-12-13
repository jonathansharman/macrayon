use std::collections::HashMap;
use std::env;
use std::error;
use std::io::{BufWriter, prelude::*};
use std::fmt::Display;
use std::fs::{self, File};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

type Macros = HashMap<String, Macro>;

#[derive(Debug)]
struct Macro {
	params: Vec<String>,
	body: String,
}

#[derive(Debug)]
struct MacroError;

impl Display for MacroError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Bad macro definition")
	}
}

impl error::Error for MacroError {}

fn load_macros(path: &Path) -> Result<Macros> {
	let source = std::fs::read_to_string(path)?;
	let mut macros = Macros::new();
	// Slice into the source.
	let mut source: &str = &source;
	// Read macros.
	'reading_macros: while let Some(macro_start) = source.find("##") {
		// Skip over macro start marker ("##").
		source = &source[macro_start + 2..];
		// Read macro name, which ends at the next "#".
		let name_end = source.find("#").ok_or(MacroError)?;
		let name = source[..name_end].trim().to_string();
		// Read parameters and body.
		let mut params: Vec<String> = Vec::new();
		loop {
			let body_start_marker = source.find("##").ok_or(MacroError)?;
			let parameter_start_marker = source.find("#").ok_or(MacroError)?;
			// If these markers are the same, then there's a "##" before the first single "#".
			if body_start_marker == parameter_start_marker {
				// Body. Skip the body start marker ("##").
				source = &source[body_start_marker + 2..];
				let macro_end_marker = source.find("##").ok_or(MacroError)?;
				// Read body.
				let body = source[..macro_end_marker].trim().to_string();
				// Advance past the end of the macro.
				source = &source[macro_end_marker + 2..];
				// Assemble and insert the macro.
				macros.insert(name, Macro { params, body });
				// Continue reading macros.
				continue 'reading_macros;
			} else {
				// Parameter. Skip the parameter separator ("#").
				source = &source[parameter_start_marker + 1..];
				let param_end = source.find("#").ok_or(MacroError)?;
				// Read parameter and insert into the parameter list.
				let param = source[..param_end].trim().to_string();
				params.push(param);
				// Advance past the parameter.
				source = &source[param_end..];
			}
		}
	}
	Ok(macros)
}

fn instantiate_macros(macros: &Macros, source: &Path, target: &Path) -> Result<()> {
	println!("Transforming {:?} to {:?}.", source, target);
	let source = std::fs::read_to_string(source)?;
	// Slice into the source.
	let mut source: &str = &source;
	// Open target file for writing.
	let mut target = BufWriter::new(File::create(target)?);
	// Instantiate macro calls.
	'instantiating_macros: while let Some(macro_start) = source.find("##") {
		// Write source text up to the macro call.
		target.write(source[..macro_start].as_bytes())?;
		// Skip over macro start marker ("##").
		source = &source[macro_start + 2..];
		// Read macro name, which ends at the next "#".
		let name_end = source.find("#").ok_or(MacroError)?;
		let name = source[..name_end].trim().to_string();
		// Read arguments.
		let mut args: Vec<String> = Vec::new();
		loop {
			let macro_end_marker = source.find("##").ok_or(MacroError)?;
			let argument_start_marker = source.find("#").ok_or(MacroError)?;
			// If these markers are the same, then there's a "##" before the first single "#".
			if macro_end_marker == argument_start_marker {
				// End of macro call. Skip the macro end marker ("##").
				source = &source[macro_end_marker + 2..];
				// Substitute arguments into the macro body.
				let the_macro = macros.get(&name).ok_or(MacroError)?;
				let mut body = the_macro.body.clone();
				// Make sure all parameters are supplied an argument.
				if args.len() != the_macro.params.len() {
					return Err(MacroError.into());
				}
				for (arg, param) in args.iter().zip(the_macro.params.iter()) {
					body = body.replace(param, arg);
				}
				// Write the instantiated macro body to the target.
				target.write(body.as_bytes())?;
				// Continue instantiating macros.
				continue 'instantiating_macros;
			} else {
				// Argument. Skip the argument separator ("#").
				source = &source[argument_start_marker + 1..];
				let arg_end = source.find("#").ok_or(MacroError)?;
				// Read argument and insert into the argument list.
				let arg = source[..arg_end].trim().to_string();
				args.push(arg);
				// Advance past the argument.
				source = &source[arg_end..];
			}
		}
	}
	// Write the rest of the source to the target.
	target.write(source.as_bytes())?;
	Ok(())
}

fn instantiate_macros_in_path(macros: &Macros, path: &Path) -> Result<()> {
	if path.is_dir() {
		for entry in fs::read_dir(path)? {
			let path = entry?.path();
			if path.is_dir() {
				instantiate_macros_in_path(&macros, &path)?;
			} else if let Some(extension) = path.extension() {
				if extension == "macry" {
					let mut target = path.clone();
					target.set_extension("cry");
					instantiate_macros(&macros, path.as_path(), target.as_path())?;
				}
			}
		}
	}
	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => {
			// Use MACROS as the macros file and recursively transform all *.macry files.
			let macros = load_macros(Path::new("MACROS")).unwrap();
			instantiate_macros_in_path(&macros, Path::new(".")).unwrap();
		},
		4 => {
			// Get macros file, macrayon source, and crayon source from args.
			let macros = load_macros(Path::new(&args[1])).unwrap();
			let source = Path::new(&args[2]);
			let target = Path::new(&args[3]);
			instantiate_macros(&macros, source, target).unwrap();
		},
		_ => {
			println!("Usage: macrayon [<macros file> <macrayon source> <crayon source>]")
		},
	}
}
