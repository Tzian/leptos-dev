use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Eq, PartialEq, EnumString, strum::Display, Clone, Serialize, Deserialize)]
pub enum ServicesError
{
	#[strum(to_string = "ConnectionError {con_err}")]
	ConnectionError
	{
		con_err: String
	},

	#[strum(to_string = "InputError {in_err}")]
	InputError
	{
		in_err: String
	},

	#[strum(to_string = "QueryError {q_err}")]
	QueryError
	{
		q_err: String
	},

	#[strum(to_string = "ConversionError {conv_err}")]
	ConversionError
	{
		conv_err: String
	},

	#[strum(to_string = "DbError {db_err}")]
	DbError
	{
		db_err: String
	},

	#[strum(to_string = "OtherError {o_err}")]
	OtherError
	{
		o_err: String
	}
}
