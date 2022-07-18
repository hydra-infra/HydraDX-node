use super::*;
use crate::mock::*;

#[cfg(test)]
mod vesting;

fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext = ExtBuilder::default().build();
	ext.execute_with(|| set_block_number::<Test>(1));
	ext
}
