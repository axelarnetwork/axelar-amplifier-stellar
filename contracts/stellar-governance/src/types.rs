#[repr(u64)]
pub enum CommandType {
    ScheduleTimeLockProposal = 0,
    CancelTimeLockProposal = 1,
    ApproveOperatorProposal = 2,
    CancelOperatorApproval = 3,
}
