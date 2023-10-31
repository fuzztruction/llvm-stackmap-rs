#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LLVMInstruction {
    Ret = 1,
    Br = 2,
    Switch = 3,
    IndirectBr = 4,
    Invoke = 5,
    Resume = 6,
    Unreachable = 7,
    CleanupRet = 8,
    CatchRet = 9,
    CatchSwitch = 10,
    CallBr = 11,
    FNeg = 12,
    Add = 13,
    FAdd = 14,
    Sub = 15,
    FSub = 16,
    Mul = 17,
    FMul = 18,
    UDiv = 19,
    SDiv = 20,
    FDiv = 21,
    URem = 22,
    SRem = 23,
    FRem = 24,
    Shl = 25,
    LShr = 26,
    AShr = 27,
    And = 28,
    Or = 29,
    Xor = 30,
    Alloca = 31,
    Load = 32,
    Store = 33,
    GetElementPtr = 34,
    Fence = 35,
    AtomicCmpXchg = 36,
    AtomicRMW = 37,
    Trunc = 38,
    ZExt = 39,
    SExt = 40,
    FPToUI = 41,
    FPToSI = 42,
    UIToFP = 43,
    SIToFP = 44,
    FPTrunc = 45,
    FPExt = 46,
    PtrToInt = 47,
    IntToPtr = 48,
    BitCast = 49,
    AddrSpaceCast = 50,
    CleanupPad = 51,
    CatchPad = 52,
    ICmp = 53,
    FCmp = 54,
    PHI = 55,
    Call = 56,
    Select = 57,
    UserOp1 = 58,
    UserOp2 = 59,
    VAArg = 60,
    ExtractElement = 61,
    InsertElement = 62,
    ShuffleVector = 63,
    ExtractValue = 64,
    InsertValue = 65,
    LandingPad = 66,
    Freeze = 67,
}

impl From<LLVMInstruction> for u8 {
    fn from(value: LLVMInstruction) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let ret = match value {
            v if v == LLVMInstruction::Ret as u8 => LLVMInstruction::Ret,
            v if v == LLVMInstruction::Ret as u8 => LLVMInstruction::Ret,
            v if v == LLVMInstruction::Br as u8 => LLVMInstruction::Br,
            v if v == LLVMInstruction::Switch as u8 => LLVMInstruction::Switch,
            v if v == LLVMInstruction::IndirectBr as u8 => LLVMInstruction::IndirectBr,
            v if v == LLVMInstruction::Invoke as u8 => LLVMInstruction::Invoke,
            v if v == LLVMInstruction::Resume as u8 => LLVMInstruction::Resume,
            v if v == LLVMInstruction::Unreachable as u8 => LLVMInstruction::Unreachable,
            v if v == LLVMInstruction::CleanupRet as u8 => LLVMInstruction::CleanupRet,
            v if v == LLVMInstruction::CatchRet as u8 => LLVMInstruction::CatchRet,
            v if v == LLVMInstruction::CatchSwitch as u8 => LLVMInstruction::CatchSwitch,
            v if v == LLVMInstruction::CallBr as u8 => LLVMInstruction::CallBr,
            v if v == LLVMInstruction::FNeg as u8 => LLVMInstruction::FNeg,
            v if v == LLVMInstruction::Add as u8 => LLVMInstruction::Add,
            v if v == LLVMInstruction::FAdd as u8 => LLVMInstruction::FAdd,
            v if v == LLVMInstruction::Sub as u8 => LLVMInstruction::Sub,
            v if v == LLVMInstruction::FSub as u8 => LLVMInstruction::FSub,
            v if v == LLVMInstruction::Mul as u8 => LLVMInstruction::Mul,
            v if v == LLVMInstruction::FMul as u8 => LLVMInstruction::FMul,
            v if v == LLVMInstruction::UDiv as u8 => LLVMInstruction::UDiv,
            v if v == LLVMInstruction::SDiv as u8 => LLVMInstruction::SDiv,
            v if v == LLVMInstruction::FDiv as u8 => LLVMInstruction::FDiv,
            v if v == LLVMInstruction::URem as u8 => LLVMInstruction::URem,
            v if v == LLVMInstruction::SRem as u8 => LLVMInstruction::SRem,
            v if v == LLVMInstruction::FRem as u8 => LLVMInstruction::FRem,
            v if v == LLVMInstruction::Shl as u8 => LLVMInstruction::Shl,
            v if v == LLVMInstruction::LShr as u8 => LLVMInstruction::LShr,
            v if v == LLVMInstruction::AShr as u8 => LLVMInstruction::AShr,
            v if v == LLVMInstruction::And as u8 => LLVMInstruction::And,
            v if v == LLVMInstruction::Or as u8 => LLVMInstruction::Or,
            v if v == LLVMInstruction::Xor as u8 => LLVMInstruction::Xor,
            v if v == LLVMInstruction::Alloca as u8 => LLVMInstruction::Alloca,
            v if v == LLVMInstruction::Load as u8 => LLVMInstruction::Load,
            v if v == LLVMInstruction::Store as u8 => LLVMInstruction::Store,
            v if v == LLVMInstruction::GetElementPtr as u8 => LLVMInstruction::GetElementPtr,
            v if v == LLVMInstruction::Fence as u8 => LLVMInstruction::Fence,
            v if v == LLVMInstruction::AtomicCmpXchg as u8 => LLVMInstruction::AtomicCmpXchg,
            v if v == LLVMInstruction::AtomicRMW as u8 => LLVMInstruction::AtomicRMW,
            v if v == LLVMInstruction::Trunc as u8 => LLVMInstruction::Trunc,
            v if v == LLVMInstruction::ZExt as u8 => LLVMInstruction::ZExt,
            v if v == LLVMInstruction::SExt as u8 => LLVMInstruction::SExt,
            v if v == LLVMInstruction::FPToUI as u8 => LLVMInstruction::FPToUI,
            v if v == LLVMInstruction::FPToSI as u8 => LLVMInstruction::FPToSI,
            v if v == LLVMInstruction::UIToFP as u8 => LLVMInstruction::UIToFP,
            v if v == LLVMInstruction::SIToFP as u8 => LLVMInstruction::SIToFP,
            v if v == LLVMInstruction::FPTrunc as u8 => LLVMInstruction::FPTrunc,
            v if v == LLVMInstruction::FPExt as u8 => LLVMInstruction::FPExt,
            v if v == LLVMInstruction::PtrToInt as u8 => LLVMInstruction::PtrToInt,
            v if v == LLVMInstruction::IntToPtr as u8 => LLVMInstruction::IntToPtr,
            v if v == LLVMInstruction::BitCast as u8 => LLVMInstruction::BitCast,
            v if v == LLVMInstruction::AddrSpaceCast as u8 => LLVMInstruction::AddrSpaceCast,
            v if v == LLVMInstruction::CleanupPad as u8 => LLVMInstruction::CleanupPad,
            v if v == LLVMInstruction::CatchPad as u8 => LLVMInstruction::CatchPad,
            v if v == LLVMInstruction::ICmp as u8 => LLVMInstruction::ICmp,
            v if v == LLVMInstruction::FCmp as u8 => LLVMInstruction::FCmp,
            v if v == LLVMInstruction::PHI as u8 => LLVMInstruction::PHI,
            v if v == LLVMInstruction::Call as u8 => LLVMInstruction::Call,
            v if v == LLVMInstruction::Select as u8 => LLVMInstruction::Select,
            v if v == LLVMInstruction::UserOp1 as u8 => LLVMInstruction::UserOp1,
            v if v == LLVMInstruction::UserOp2 as u8 => LLVMInstruction::UserOp2,
            v if v == LLVMInstruction::VAArg as u8 => LLVMInstruction::VAArg,
            v if v == LLVMInstruction::ExtractElement as u8 => LLVMInstruction::ExtractElement,
            v if v == LLVMInstruction::InsertElement as u8 => LLVMInstruction::InsertElement,
            v if v == LLVMInstruction::ShuffleVector as u8 => LLVMInstruction::ShuffleVector,
            v if v == LLVMInstruction::ExtractValue as u8 => LLVMInstruction::ExtractValue,
            v if v == LLVMInstruction::InsertValue as u8 => LLVMInstruction::InsertValue,
            v if v == LLVMInstruction::LandingPad as u8 => LLVMInstruction::LandingPad,
            v if v == LLVMInstruction::Freeze as u8 => LLVMInstruction::Freeze,
            _ => return Err(format!("Unknown LLVM instructions with id {}", value)),
        };
        Ok(ret)
    }
}

impl TryFrom<u16> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let valueu8 = u8::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}

impl TryFrom<u32> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let valueu8 = u8::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}

impl TryFrom<u64> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let valueu8 = u8::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}