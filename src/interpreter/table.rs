use std::u32;
use parking_lot::RwLock;
use elements::{TableType, ResizableLimits};
use interpreter::Error;
use interpreter::module::check_limits;
use interpreter::variable::VariableType;
use interpreter::store::FuncId;

/// Table instance.
pub struct TableInstance {
	/// Table limits.
	limits: ResizableLimits,
	/// Table memory buffer.
	buffer: RwLock<Vec<Option<FuncId>>>,

}

impl TableInstance {
	/// New instance of the table
	pub fn new(table_type: &TableType) -> Result<Self, Error> {
		check_limits(table_type.limits())?;
		Ok(TableInstance {
			limits: table_type.limits().clone(),
			buffer: RwLock::new(
				vec![None; table_type.limits().initial() as usize]
			),
		})
	}

	/// Return table limits.
	pub fn limits(&self) -> &ResizableLimits {
		&self.limits
	}

	/// Get variable type for this table.
	pub fn variable_type(&self) -> VariableType {
		panic!("TODO")
	}

	/// Get the specific value in the table
	pub fn get(&self, offset: u32) -> Result<FuncId, Error> {
		let buffer = self.buffer.read();
		let buffer_len = buffer.len();
		let table_elem = buffer.get(offset as usize).ok_or(Error::Table(format!(
			"trying to read table item with index {} when there are only {} items",
			offset,
			buffer_len
		)))?;
		Ok(table_elem.ok_or(Error::Table(format!(
			"trying to read uninitialized element on index {}",
			offset
		)))?)
	}

	/// Set the table element to the specified function.
	pub fn set(&self, offset: u32, value: FuncId) -> Result<(), Error> {
		let mut buffer = self.buffer.write();
		let buffer_len = buffer.len();
		let table_elem = buffer.get_mut(offset as usize).ok_or(Error::Table(format!(
			"trying to update table item with index {} when there are only {} items",
			offset,
			buffer_len
		)))?;
		*table_elem = Some(value);
		Ok(())
	}
}
