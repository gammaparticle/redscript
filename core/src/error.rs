use std::fmt::{self, Display};
use std::{io, usize};

use crate::ast::Pos;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DecompileError(String),
    SyntaxError(String, Pos),
    CompileError(String, Pos),
    TypeError(String, Pos),
    ResolutionError(String, Pos),
    PoolError(String),
    FormatError(fmt::Error),
    MultipleErrors(Vec<Pos>),
}

impl Error {
    pub fn eof(hint: String) -> Error {
        Error::IoError(io::Error::new(io::ErrorKind::UnexpectedEof, hint))
    }

    pub fn function_not_found<F: Display>(fun_name: F, pos: Pos) -> Error {
        let error = format!("Function {} not found", fun_name);
        Error::CompileError(error, pos)
    }

    pub fn member_not_found<M: Display, C: Display>(member: M, context: C, pos: Pos) -> Error {
        let error = format!("Member {} not found on {}", member, context);
        Error::CompileError(error, pos)
    }

    pub fn class_not_found<N: Display>(class_name: N, pos: Pos) -> Error {
        let error = format!("Can't find class {}", class_name);
        Error::CompileError(error, pos)
    }

    pub fn class_is_abstract<N: Display>(class_name: N, pos: Pos) -> Error {
        let error = format!("Cannot instantiate abstract class {}", class_name);
        Error::CompileError(error, pos)
    }

    pub fn unresolved_reference<N: Display>(name: N, pos: Pos) -> Error {
        let error = format!("Unresolved reference {}", name);
        Error::CompileError(error, pos)
    }

    pub fn unresolved_type<N: Display>(name: N, pos: Pos) -> Error {
        let error = format!("Unresolved type {}", name);
        Error::CompileError(error, pos)
    }

    pub fn unresolved_import<N: Display>(import: N, pos: Pos) -> Error {
        Error::CompileError(format!("Unresolved import {}", import), pos)
    }

    pub fn unresolved_module<N: Display>(import: N, pos: Pos) -> Error {
        Error::CompileError(format!("Module {} has no members or does not exist", import), pos)
    }

    pub fn invalid_annotation_args(pos: Pos) -> Error {
        Error::CompileError("Invalid arguments for annotation".to_owned(), pos)
    }

    pub fn type_annotation_required(pos: Pos) -> Error {
        Error::CompileError("Type annotation required".to_owned(), pos)
    }

    pub fn invalid_context<N: Display>(type_: N, pos: Pos) -> Error {
        let error = format!("{} doesn't have members", type_);
        Error::CompileError(error, pos)
    }

    pub fn invalid_op<N: Display>(type_: N, op: &str, pos: Pos) -> Error {
        let error = format!("{} is not supported on {}", op, type_);
        Error::CompileError(error, pos)
    }

    pub fn invalid_arg_count<N: Display>(name: N, expected: usize, pos: Pos) -> Error {
        let error = format!("Expected {} parameters for {}", expected, name);
        Error::CompileError(error, pos)
    }

    pub fn void_cannot_be_used(pos: Pos) -> Error {
        Error::CompileError("Void value cannot be used".to_owned(), pos)
    }

    pub fn value_expected<N: Display>(found: N, pos: Pos) -> Error {
        Error::CompileError(format!("Expected a value, found {}", found), pos)
    }

    pub fn return_type_mismatch<N: Display>(type_: N, pos: Pos) -> Error {
        let error = format!("Function should return {}", type_);
        Error::CompileError(error, pos)
    }

    pub fn type_error<F: Display, T: Display>(from: F, to: T, pos: Pos) -> Error {
        let error = format!("Can't coerce {} to {}", from, to);
        Error::TypeError(error, pos)
    }

    pub fn no_matching_overload<N: Display>(name: N, errors: &[FunctionResolutionError], pos: Pos) -> Error {
        let max_errors = 10;
        let messages = errors
            .iter()
            .take(max_errors)
            .fold(String::new(), |acc, str| acc + "\n " + &str.0);

        let detail = if errors.len() > max_errors {
            format!("{}\n...and more", messages)
        } else {
            messages
        };
        let error = format!(
            "Arguments passed to {} do not match any of the overloads:{}",
            name, detail
        );
        Error::ResolutionError(error, pos)
    }

    pub fn invalid_intrinsic<N: Display, T: Display>(name: N, type_: T, pos: Pos) -> Error {
        let err = format!("Invalid intrinsic {} call: unexpected {}", name, type_);
        Error::CompileError(err, pos)
    }

    pub fn expected_static_method<N: Display>(name: N, pos: Pos) -> Error {
        let err = format!("Method {} is not static", name);
        Error::CompileError(err, pos)
    }

    pub fn expected_non_static_method<N: Display>(name: N, pos: Pos) -> Error {
        let err = format!("Method {} is static", name);
        Error::CompileError(err, pos)
    }

    pub fn no_this_in_static_context(pos: Pos) -> Error {
        Error::CompileError("No 'this' in static context".to_owned(), pos)
    }

    pub fn unsupported<N: Display>(name: N, pos: Pos) -> Error {
        let err = format!("{} is unsupported", name);
        Error::CompileError(err, pos)
    }

    pub fn class_redefinition(pos: Pos) -> Error {
        let err = "Class with this name is already defined elsewhere".to_owned();
        Error::CompileError(err, pos)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FormatError(err)
    }
}

#[derive(Debug)]
pub struct FunctionResolutionError(String);

impl FunctionResolutionError {
    pub fn parameter_mismatch(cause: &str, index: usize) -> FunctionResolutionError {
        let message = format!("Invalid parameter at position {}: {}", index, cause);
        FunctionResolutionError(message)
    }

    pub fn return_mismatch<N: Display>(expected: N, given: N) -> FunctionResolutionError {
        let message = format!("Return type {} does not match expected {}", given, expected);
        FunctionResolutionError(message)
    }

    pub fn too_many_args(expected: usize, got: usize) -> FunctionResolutionError {
        let error = format!("Too many arguments, expected {} but got {}", expected, got);
        FunctionResolutionError(error)
    }

    pub fn invalid_arg_count(received: usize, min: usize, max: usize) -> FunctionResolutionError {
        let message = format!("Expected {}-{} parameters, given {}", min, max, received);
        FunctionResolutionError(message)
    }
}
