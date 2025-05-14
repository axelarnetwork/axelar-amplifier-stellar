#[repr(u64)]
pub enum CommandType {
    ScheduleTimeLockProposal = 1,
    CancelTimeLockProposal = 2,
    ApproveOperatorProposal = 3,
    CancelOperatorApproval = 4,
}
