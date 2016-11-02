use client::{Rpc, RpcError};
use rpc::v1::types::{ConfirmationRequest,
					 TransactionModification,
					 U256};
use serde_json::{Value as JsonValue, to_value};
use std::path::PathBuf;
use futures::{BoxFuture, Canceled};

pub struct SignerRpc {
	rpc: Rpc,
}

impl SignerRpc {
	pub fn new(url: &str, authfile: &PathBuf) -> Result<Self, RpcError> {
		Ok(SignerRpc { rpc: try!(Rpc::new(&url, authfile)) })
	}
	pub fn requests_to_confirm(&mut self) ->
		BoxFuture<Result<Vec<ConfirmationRequest>, RpcError>, Canceled>
	{
		self.rpc.request("personal_requestsToConfirm", vec![])
	}
	pub fn confirm_request(
		&mut self,
		id: U256,
		new_gas_price: Option<U256>,
		pwd: &str
	) -> BoxFuture<Result<U256, RpcError>, Canceled>
	{
		self.rpc.request("personal_confirmRequest", vec![
			to_value(&format!("{:#x}", id)),
			to_value(&TransactionModification { gas_price: new_gas_price }),
			to_value(&pwd),
		])
	}
	pub fn reject_request(&mut self, id: U256) ->
		BoxFuture<Result<bool, RpcError>, Canceled>
	{
		self.rpc.request("personal_rejectRequest", vec![
			JsonValue::String(format!("{:#x}", id))
		])
	}
}
