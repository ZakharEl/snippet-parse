use std::rc::{Weak, Rc};
use std::fmt;

/// Part of the snippet that is fashioned from user input.
#[derive(Debug)]
pub enum Field {
	/// Typed in text.
	Placeholder(Vec<Segment>),
	/// 1 choice selected from menu of choices.
	Choice(usize, Vec<Vec<Segment>>),
}

/// Part of the snippet produced by replacing matching patterns within the snippet's specified fields and variables (transformables).
#[derive(Debug)]
pub struct Transformation {
	/// Regex for part of the transformable to match.
	pub section: String,
	/// Modification or replacement to be done upon pattern.
	pub format: String,
	/// Changes how pattern and/or format is applied.
	pub flags: String,
	/// Result of performing the transformation.
	pub result: String
}

/// Defines where a variable comes from.
#[derive(Debug)]
pub enum VariableSource {
	/// Comes from the program using this library - likely from environment variables.
	Daemon,
	/// Comes from external program - likely being passed in via a socket.
	Client
}

/// Part of the snippet that is filled in by program variables (ie environment variables).
#[derive(Debug)]
pub struct Variable {
	/// Name of the variable.
	pub name: String,
	/// Value of the variable.
	pub value: String,
	/// Where a variable comes from.
	pub source: VariableSource
}

/// Part of the snippet that is filled in with the output of an external program - likely some type of shell code.
#[derive(Debug)]
pub struct Code {
	/// The code to run.
	pub code: String,
	/// The output resulting from the code being run.
	pub output: String,
	/// The program that runs the code (identical to the shebang format on unix platforms).
	pub shebang: String
}

/// Essentially the snippet segmented into normal text (text variant) and non normal text
/// (all the other variants). Non normal text is text that will be filled in by a user or program.
#[derive(Debug)]
pub enum Segment {
	/// Just normal text.
	Text(String),
	/// Input from user.
	Field(Rc<Field>),
	/// Result of replacing matching patterns within specified fields and variables (transformables).
	Transformation(Rc<Transformation>),
	/// Program variable.
	Variable(Rc<Variable>),
	/// Output of an external program.
	Code(Rc<Code>),
	/// A expanded snippet nested within this segment's snippet.
	/// Not able to be set through this segment's snippet's initialization string.
	/// Only able to be set after initialization to idicate that the user expanded a snippet within this segment's snippet.
	Snippet(Rc<Snippet>)
}

/// Segments that otherwise wouldn't have a name (not variables or fields) but are given 1 so they can be reused without having to retype them in full.
#[derive(Debug)]
pub enum NamedSegment {
	///Result of replacing matching patterns within another field.
	Transformation(String, Weak<Transformation>),
	/// Output of an external program.
	Code(String, Weak<Code>)
}

/// Selections within the snippet that are cycled through in order to be filled in (or in the case of choice selected) by user.
#[derive(Debug)]
pub struct Tab {
	/// Indicates the order in which this tab is selected in the cycle.
	/// Should be unique to each tab.
	/// Otherwise this module might not function as intended.
	pub num: u8,
	/// Part of the snippet that will be selected when this tab is selected.
	pub field: Weak<Field>,
	/// All transformations that act upon this variable.
	/// Empty transformations means there are no transformations that are acting upon this variable.
	pub transformations: Vec<Weak<Transformation>>
}

///Represents text filled in by a program.
#[derive(Debug)]
pub struct Expansion<E> {
	/// The text that is actually filled in by program.
	pub expansion: Weak<E>,
	/// Text that is filled in by transformations upon the expansion field above.
	pub transformations: Vec<Weak<Transformation>>
}

/// A portion of text that represents often typed out text.
/// May or may not have portions within it that are meant to be fashioned from user input.
/// Typically to be expanded from a much smaller text within a coding environment like a text editor to reduce lengthy repetitive typing.
#[derive(Debug)]
pub struct Snippet {
	/// Segments of normal text and non normal text.
	/// Non normal text is text that will be filled in by a user or program.
	body: Vec<Segment>,
	/// Selections within this snippet that are cycled through in order to be fashioned from user input.
	tabs: Vec<Tab>,
	/// Program variables.
	variables: Vec<Expansion<Variable>>,
	/// Output of a program.
	code_expansions: Vec<Expansion<Code>>,
	/// Segments that otherwise wouldn't have a name (not variables or fields) but are given 1 so they can be reused without having to retype them in full.
	named_segments: Vec<NamedSegment>
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}

impl fmt::Display for Code {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.output)
	}
}

impl fmt::Display for Transformation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.result)
	}
}

impl fmt::Display for Segment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Segment::Text(text) => write!(f, "{}", text),
			Segment::Variable(variable) => write!(f, "{}", variable),
			Segment::Code(code) => write!(f, "{}", code),
			Segment::Snippet(snippet) => write!(f, "{}", snippet),
			Segment::Field(field) => write!(f, "{}", field),
			Segment::Transformation(transformation) => write!(f, "{}", transformation)
		}
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut body = match self {
			Field::Placeholder(child_body) => child_body,
			Field::Choice(choice, child_body) => if let Some(child_body) = child_body.get(*choice) {
				child_body
			} else {
				return Ok(())
			}
		}.iter();
		while let Some(seg) = body.next() {
			write!(f, "{}", seg)?;
		}
		Ok(())
	}
}

impl fmt::Display for Snippet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut body = self.body.iter();
		while let Some(seg) = body.next() {
			write!(f, "{}", seg)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construct_snippet() {
		let mut result = Snippet {
			body: vec![Segment::Field(Rc::new(Field::Choice(1, vec![vec![Segment::Text(String::from("Hi"))], vec![Segment::Text(String::from("Hello"))], vec![Segment::Text(String::from("Howdee"))]]))), Segment::Text(String::from(" there ")), Segment::Field(Rc::new(Field::Placeholder(vec![Segment::Text(String::from("John"))])))],
			tabs: Vec::new(),
			variables: Vec::new(),
			code_expansions: Vec::new(),
			named_segments: Vec::new()
		};
		println!("{}", result);
		println!("{:?}", result);
	}
}
