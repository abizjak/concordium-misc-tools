//! Smart contract test bench that is used for testing various interaction
//! scenarios with wallets via the given front end.
#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;

/// The different errors the contract can produce.
#[derive(Serialize, Debug, PartialEq, Eq, Reject, SchemaType)]
enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// This smart contract reverts.
    SmartContractReverts,
    /// Failed to invoke a contract.
    InvokeError,
}

/// Mapping errors related to contract invocations to ContractError.
impl<T> From<CallContractError<T>> for ContractError {
    fn from(_cce: CallContractError<T>) -> Self { Self::InvokeError }
}

const PUBLIC_KEY: PublicKeyEd25519 = PublicKeyEd25519([
    55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165, 234,
    138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153,
]);

const SIGNATURE: SignatureEd25519 = SignatureEd25519([
    99, 47, 86, 124, 147, 33, 64, 92, 226, 1, 160, 163, 134, 21, 218, 65, 239, 226, 89, 237, 225,
    84, 255, 69, 173, 150, 205, 248, 96, 113, 142, 121, 189, 224, 124, 255, 114, 196, 209, 25, 198,
    68, 85, 42, 140, 127, 12, 65, 63, 92, 245, 57, 11, 14, 160, 69, 137, 147, 214, 214, 55, 75,
    217, 4,
]);

const HASH: HashSha2256 = concordium_std::HashSha2256([2u8; 32]);

const STRING: &str = "abc";

/// The contract state.
#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    u8_value:               u8,
    u16_value:              u16,
    address_array:          StateBox<Vec<Address>, S>,
    address_value:          Address,
    account_address_value:  AccountAddress,
    contract_address_value: ContractAddress,
    hash_value:             HashSha2256,
    signature_value:        SignatureEd25519,
    public_key_value:       PublicKeyEd25519,
    timestamp_value:        Timestamp,
    option_value:           Option<u8>,
    string_value:           StateBox<String, S>,
}

#[derive(Serial, Deserial, SchemaType)]
struct ReturnState {
    u8_value:               u8,
    u16_value:              u16,
    address_array:          Vec<Address>,
    address_value:          Address,
    account_address_value:  AccountAddress,
    contract_address_value: ContractAddress,
    hash_value:             HashSha2256,
    signature_value:        SignatureEd25519,
    public_key_value:       PublicKeyEd25519,
    timestamp_value:        Timestamp,
    option_value:           Option<u8>,
    #[concordium(size_length = 2)]
    string_value:           String,
}

impl<'a, S: HasStateApi> From<&'a State<S>> for ReturnState {
    fn from(
        State {
            u8_value,
            u16_value,
            address_array,
            address_value,
            account_address_value,
            contract_address_value,
            hash_value,
            signature_value,
            public_key_value,
            timestamp_value,
            option_value,
            string_value,
        }: &'a State<S>,
    ) -> Self {
        Self {
            u8_value:               *u8_value,
            u16_value:              *u16_value,
            address_array:          address_array.to_vec(),
            address_value:          *address_value,
            account_address_value:  *account_address_value,
            contract_address_value: *contract_address_value,
            hash_value:             *hash_value,
            signature_value:        *signature_value,
            public_key_value:       *public_key_value,
            timestamp_value:        *timestamp_value,
            option_value:           *option_value,
            string_value:           String::from(string_value.as_str()),
        }
    }
}

/// Init function that creates this smart_contract_test_bench.
#[init(contract = "smart_contract_test_bench", parameter = "u16", payable)]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
    _amount: Amount,
) -> InitResult<State<S>> {
    let size = ctx.parameter_cursor().size();
    let mut u16_value: u16 = 0u16;

    if size > 0 {
        u16_value = ctx.parameter_cursor().get()?;
    }

    Ok(State {
        u8_value: 0u8,
        u16_value,
        address_array: state_builder.new_box(vec![]),
        address_value: Address::Account(AccountAddress([0u8; 32])),
        account_address_value: AccountAddress([0u8; 32]),
        contract_address_value: ContractAddress {
            index:    0,
            subindex: 0,
        },
        hash_value: HASH,
        signature_value: SIGNATURE,
        public_key_value: PUBLIC_KEY,
        timestamp_value: Timestamp::from_timestamp_millis(11),
        option_value: None,
        string_value: state_builder.new_box(STRING.to_string()),
    })
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_u8",
    parameter = "u8",
    error = "ContractError",
    mutable
)]
fn set_u8<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: u8 = ctx.parameter_cursor().get()?;
    host.state_mut().u8_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_u8_payable",
    parameter = "u8",
    error = "ContractError",
    mutable,
    payable
)]
fn set_u8_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: u8 = ctx.parameter_cursor().get()?;
    host.state_mut().u8_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_u8",
    return_value = "u8",
    error = "ContractError",
    mutable
)]
fn get_u8<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<u8, ContractError> {
    Ok(host.state_mut().u8_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_u16",
    parameter = "u16",
    error = "ContractError",
    mutable
)]
fn set_u16<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: u16 = ctx.parameter_cursor().get()?;
    host.state_mut().u16_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_u16_payable",
    parameter = "u16",
    error = "ContractError",
    mutable,
    payable
)]
fn set_u16_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: u16 = ctx.parameter_cursor().get()?;
    host.state_mut().u16_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_u16",
    return_value = "u16",
    error = "ContractError",
    mutable
)]
fn get_u16<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<u16, ContractError> {
    Ok(host.state_mut().u16_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_address",
    parameter = "Address",
    error = "ContractError",
    mutable
)]
fn set_address<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: Address = ctx.parameter_cursor().get()?;
    host.state_mut().address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_address_payable",
    parameter = "Address",
    error = "ContractError",
    mutable,
    payable
)]
fn set_address_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: Address = ctx.parameter_cursor().get()?;
    host.state_mut().address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_address",
    return_value = "Address",
    error = "ContractError",
    mutable
)]
fn get_address<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<Address, ContractError> {
    Ok(host.state_mut().address_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_contract_address",
    parameter = "ContractAddress",
    error = "ContractError",
    mutable
)]
fn set_contract_address<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().contract_address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_contract_address_payable",
    parameter = "ContractAddress",
    error = "ContractError",
    mutable,
    payable
)]
fn set_contract_address_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().contract_address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_contract_address",
    return_value = "ContractAddress",
    error = "ContractError",
    mutable
)]
fn get_contract_address<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<ContractAddress, ContractError> {
    Ok(host.state_mut().contract_address_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_account_address",
    parameter = "AccountAddress",
    error = "ContractError",
    mutable
)]
fn set_account_address<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: AccountAddress = ctx.parameter_cursor().get()?;
    host.state_mut().account_address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_account_address_payable",
    parameter = "AccountAddress",
    error = "ContractError",
    mutable,
    payable
)]
fn set_account_address_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: AccountAddress = ctx.parameter_cursor().get()?;
    host.state_mut().account_address_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_account_address",
    return_value = "AccountAddress",
    error = "ContractError",
    mutable
)]
fn get_account_address<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<AccountAddress, ContractError> {
    Ok(host.state_mut().account_address_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_hash",
    parameter = "HashSha2256",
    error = "ContractError",
    mutable
)]
fn set_hash<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: HashSha2256 = ctx.parameter_cursor().get()?;
    host.state_mut().hash_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_hash_payable",
    parameter = "HashSha2256",
    error = "ContractError",
    mutable,
    payable
)]
fn set_hash_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: HashSha2256 = ctx.parameter_cursor().get()?;
    host.state_mut().hash_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_hash",
    return_value = "HashSha2256",
    error = "ContractError",
    mutable
)]
fn get_hash<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<HashSha2256, ContractError> {
    Ok(host.state_mut().hash_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_public_key",
    parameter = "PublicKeyEd25519",
    error = "ContractError",
    mutable
)]
fn set_public_key<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: PublicKeyEd25519 = ctx.parameter_cursor().get()?;
    host.state_mut().public_key_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_public_key_payable",
    parameter = "PublicKeyEd25519",
    error = "ContractError",
    mutable,
    payable
)]
fn set_public_key_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: PublicKeyEd25519 = ctx.parameter_cursor().get()?;
    host.state_mut().public_key_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_public_key",
    return_value = "PublicKeyEd25519",
    error = "ContractError",
    mutable
)]
fn get_public_key<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<PublicKeyEd25519, ContractError> {
    Ok(host.state_mut().public_key_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_signature",
    parameter = "SignatureEd25519",
    error = "ContractError",
    mutable
)]
fn set_signature<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: SignatureEd25519 = ctx.parameter_cursor().get()?;
    host.state_mut().signature_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_signature_payable",
    parameter = "SignatureEd25519",
    error = "ContractError",
    mutable,
    payable
)]
fn set_signature_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: SignatureEd25519 = ctx.parameter_cursor().get()?;
    host.state_mut().signature_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_signature",
    return_value = "SignatureEd25519",
    error = "ContractError",
    mutable
)]
fn get_signature<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<SignatureEd25519, ContractError> {
    Ok(host.state_mut().signature_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_timestamp",
    parameter = "Timestamp",
    error = "ContractError",
    mutable
)]
fn set_timestamp<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: Timestamp = ctx.parameter_cursor().get()?;
    host.state_mut().timestamp_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_timestamp_payable",
    parameter = "Timestamp",
    error = "ContractError",
    mutable,
    payable
)]
fn set_timestamp_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: Timestamp = ctx.parameter_cursor().get()?;
    host.state_mut().timestamp_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_timestamp",
    return_value = "Timestamp",
    error = "ContractError",
    mutable
)]
fn get_timestamp<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<Timestamp, ContractError> {
    Ok(host.state_mut().timestamp_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_string",
    parameter = "String",
    error = "ContractError",
    mutable
)]
fn set_string<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: String = ctx.parameter_cursor().get()?;
    drop(host.state_mut().string_value.replace(value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_string_payable",
    parameter = "String",
    error = "ContractError",
    mutable,
    payable
)]
fn set_string_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: String = ctx.parameter_cursor().get()?;
    drop(host.state_mut().string_value.replace(value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_string",
    return_value = "String",
    error = "ContractError",
    mutable
)]
fn get_string<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<String, ContractError> {
    Ok(host.state_mut().string_value.clone())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_option_u8",
    parameter = "Option<u8>",
    error = "ContractError",
    mutable
)]
fn set_option_u8<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: Option<u8> = ctx.parameter_cursor().get()?;
    host.state_mut().option_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_option_u8_payable",
    parameter = "Option<u8>",
    error = "ContractError",
    mutable,
    payable
)]
fn set_option_u8_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: Option<u8> = ctx.parameter_cursor().get()?;
    host.state_mut().option_value = value;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_option_u8",
    return_value = "Option<u8>",
    error = "ContractError",
    mutable
)]
fn get_option_u8<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<Option<u8>, ContractError> {
    Ok(host.state_mut().option_value)
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_address_array",
    parameter = "Vec<Address>",
    error = "ContractError",
    mutable
)]
fn set_address_array<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: Vec<Address> = ctx.parameter_cursor().get()?;
    drop(host.state_mut().address_array.replace(value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_address_array_payable",
    parameter = "Vec<Address>",
    error = "ContractError",
    mutable,
    payable
)]
fn set_address_array_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: Vec<Address> = ctx.parameter_cursor().get()?;
    drop(host.state_mut().address_array.replace(value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_address_array",
    return_value = "Vec<Address>",
    error = "ContractError",
    mutable
)]
fn get_address_array<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<Vec<Address>, ContractError> {
    Ok(host.state_mut().address_array.clone())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_object",
    parameter = "ReturnState",
    error = "ContractError",
    mutable
)]
fn set_object<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let value: ReturnState = ctx.parameter_cursor().get()?;
    host.state_mut().account_address_value = value.account_address_value;
    host.state_mut().contract_address_value = value.contract_address_value;
    host.state_mut().address_value = value.address_value;
    host.state_mut().u8_value = value.u8_value;
    host.state_mut().u16_value = value.u16_value;
    drop(host.state_mut().address_array.replace(value.address_array));
    host.state_mut().hash_value = value.hash_value;
    host.state_mut().signature_value = value.signature_value;
    host.state_mut().public_key_value = value.public_key_value;
    host.state_mut().timestamp_value = value.timestamp_value;
    host.state_mut().option_value = value.option_value;
    drop(host.state_mut().string_value.replace(value.string_value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "set_object_payable",
    parameter = "ReturnState",
    error = "ContractError",
    mutable,
    payable
)]
fn set_object_payable<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    _amount: Amount,
) -> Result<(), ContractError> {
    let value: ReturnState = ctx.parameter_cursor().get()?;
    host.state_mut().account_address_value = value.account_address_value;
    host.state_mut().contract_address_value = value.contract_address_value;
    host.state_mut().address_value = value.address_value;
    host.state_mut().u8_value = value.u8_value;
    host.state_mut().u16_value = value.u16_value;
    drop(host.state_mut().address_array.replace(value.address_array));
    host.state_mut().hash_value = value.hash_value;
    host.state_mut().signature_value = value.signature_value;
    host.state_mut().public_key_value = value.public_key_value;
    host.state_mut().timestamp_value = value.timestamp_value;
    host.state_mut().option_value = value.option_value;
    drop(host.state_mut().string_value.replace(value.string_value));

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "get_object",
    return_value = "ReturnState",
    error = "ContractError",
    mutable
)]
fn get_object<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<ReturnState, ContractError> {
    Ok(host.state().into())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "success",
    error = "ContractError",
    mutable
)]
fn success<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "reverts",
    error = "ContractError",
    mutable
)]
fn reverts<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    bail!(ContractError::SmartContractReverts);
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "internal_call_reverts",
    error = "ContractError",
    mutable
)]
fn internal_call_reverts<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    host.invoke_contract(
        &ctx.self_address(),
        &Parameter::empty(),
        EntrypointName::new_unchecked("reverts"),
        Amount::zero(),
    )?;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "internal_call_success",
    error = "ContractError",
    mutable
)]
fn internal_call_success<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    host.invoke_contract(
        &ctx.self_address(),
        &Parameter::empty(),
        EntrypointName::new_unchecked("success"),
        Amount::zero(),
    )?;

    Ok(())
}

#[receive(
    contract = "smart_contract_test_bench",
    name = "view",
    parameter = "HashSha2256",
    error = "ContractError",
    return_value = "ReturnState"
)]
fn view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ReturnState> {
    Ok(host.state().into())
}
