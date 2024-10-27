/// Primitive representation of a Blockchain block.
pub struct Block<Header, Extrinsic> {
    /// Contians metadata about the block
    pub header: Header,
    /// Represent the state transitions to be executed in this block.
    pub extrinsics: Vec<Extrinsic>,
}

/// Extremely simplified header which only contains the current block number.
/// Also need:
/// - parent block hash
/// - state root
/// - extrinsics root
/// - etc...
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

/// An external message from outside of the blockchain.
/// This simplified version of an extrinsic tells us who is making the call, and which
/// that making.
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

/// The result type of runtime
pub type DispatchResult = Result<(), &'static str>;

/// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
/// function call
pub trait Dispatch {
    /// The type used to identify the caller of the function.
    type Caller;
    /// The state transition function call the caller is trying to access.
    type Call;

    /// This function takes a 'caller' and the 'call' they want to make, and returns a 'Result'
    /// based on the outcome of that function call.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
} 
