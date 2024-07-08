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
    InjectedCall = 1337,
    CustomPatchPoint = 1338,
}

impl From<LLVMInstruction> for usize {
    fn from(value: LLVMInstruction) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for LLVMInstruction {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let ret = match value {
            v if v == LLVMInstruction::Ret as usize => LLVMInstruction::Ret,
            v if v == LLVMInstruction::Ret as usize => LLVMInstruction::Ret,
            v if v == LLVMInstruction::Br as usize => LLVMInstruction::Br,
            v if v == LLVMInstruction::Switch as usize => LLVMInstruction::Switch,
            v if v == LLVMInstruction::IndirectBr as usize => LLVMInstruction::IndirectBr,
            v if v == LLVMInstruction::Invoke as usize => LLVMInstruction::Invoke,
            v if v == LLVMInstruction::Resume as usize => LLVMInstruction::Resume,
            v if v == LLVMInstruction::Unreachable as usize => LLVMInstruction::Unreachable,
            v if v == LLVMInstruction::CleanupRet as usize => LLVMInstruction::CleanupRet,
            v if v == LLVMInstruction::CatchRet as usize => LLVMInstruction::CatchRet,
            v if v == LLVMInstruction::CatchSwitch as usize => LLVMInstruction::CatchSwitch,
            v if v == LLVMInstruction::CallBr as usize => LLVMInstruction::CallBr,
            v if v == LLVMInstruction::FNeg as usize => LLVMInstruction::FNeg,
            v if v == LLVMInstruction::Add as usize => LLVMInstruction::Add,
            v if v == LLVMInstruction::FAdd as usize => LLVMInstruction::FAdd,
            v if v == LLVMInstruction::Sub as usize => LLVMInstruction::Sub,
            v if v == LLVMInstruction::FSub as usize => LLVMInstruction::FSub,
            v if v == LLVMInstruction::Mul as usize => LLVMInstruction::Mul,
            v if v == LLVMInstruction::FMul as usize => LLVMInstruction::FMul,
            v if v == LLVMInstruction::UDiv as usize => LLVMInstruction::UDiv,
            v if v == LLVMInstruction::SDiv as usize => LLVMInstruction::SDiv,
            v if v == LLVMInstruction::FDiv as usize => LLVMInstruction::FDiv,
            v if v == LLVMInstruction::URem as usize => LLVMInstruction::URem,
            v if v == LLVMInstruction::SRem as usize => LLVMInstruction::SRem,
            v if v == LLVMInstruction::FRem as usize => LLVMInstruction::FRem,
            v if v == LLVMInstruction::Shl as usize => LLVMInstruction::Shl,
            v if v == LLVMInstruction::LShr as usize => LLVMInstruction::LShr,
            v if v == LLVMInstruction::AShr as usize => LLVMInstruction::AShr,
            v if v == LLVMInstruction::And as usize => LLVMInstruction::And,
            v if v == LLVMInstruction::Or as usize => LLVMInstruction::Or,
            v if v == LLVMInstruction::Xor as usize => LLVMInstruction::Xor,
            v if v == LLVMInstruction::Alloca as usize => LLVMInstruction::Alloca,
            v if v == LLVMInstruction::Load as usize => LLVMInstruction::Load,
            v if v == LLVMInstruction::Store as usize => LLVMInstruction::Store,
            v if v == LLVMInstruction::GetElementPtr as usize => LLVMInstruction::GetElementPtr,
            v if v == LLVMInstruction::Fence as usize => LLVMInstruction::Fence,
            v if v == LLVMInstruction::AtomicCmpXchg as usize => LLVMInstruction::AtomicCmpXchg,
            v if v == LLVMInstruction::AtomicRMW as usize => LLVMInstruction::AtomicRMW,
            v if v == LLVMInstruction::Trunc as usize => LLVMInstruction::Trunc,
            v if v == LLVMInstruction::ZExt as usize => LLVMInstruction::ZExt,
            v if v == LLVMInstruction::SExt as usize => LLVMInstruction::SExt,
            v if v == LLVMInstruction::FPToUI as usize => LLVMInstruction::FPToUI,
            v if v == LLVMInstruction::FPToSI as usize => LLVMInstruction::FPToSI,
            v if v == LLVMInstruction::UIToFP as usize => LLVMInstruction::UIToFP,
            v if v == LLVMInstruction::SIToFP as usize => LLVMInstruction::SIToFP,
            v if v == LLVMInstruction::FPTrunc as usize => LLVMInstruction::FPTrunc,
            v if v == LLVMInstruction::FPExt as usize => LLVMInstruction::FPExt,
            v if v == LLVMInstruction::PtrToInt as usize => LLVMInstruction::PtrToInt,
            v if v == LLVMInstruction::IntToPtr as usize => LLVMInstruction::IntToPtr,
            v if v == LLVMInstruction::BitCast as usize => LLVMInstruction::BitCast,
            v if v == LLVMInstruction::AddrSpaceCast as usize => LLVMInstruction::AddrSpaceCast,
            v if v == LLVMInstruction::CleanupPad as usize => LLVMInstruction::CleanupPad,
            v if v == LLVMInstruction::CatchPad as usize => LLVMInstruction::CatchPad,
            v if v == LLVMInstruction::ICmp as usize => LLVMInstruction::ICmp,
            v if v == LLVMInstruction::FCmp as usize => LLVMInstruction::FCmp,
            v if v == LLVMInstruction::PHI as usize => LLVMInstruction::PHI,
            v if v == LLVMInstruction::Call as usize => LLVMInstruction::Call,
            v if v == LLVMInstruction::Select as usize => LLVMInstruction::Select,
            v if v == LLVMInstruction::UserOp1 as usize => LLVMInstruction::UserOp1,
            v if v == LLVMInstruction::UserOp2 as usize => LLVMInstruction::UserOp2,
            v if v == LLVMInstruction::VAArg as usize => LLVMInstruction::VAArg,
            v if v == LLVMInstruction::ExtractElement as usize => LLVMInstruction::ExtractElement,
            v if v == LLVMInstruction::InsertElement as usize => LLVMInstruction::InsertElement,
            v if v == LLVMInstruction::ShuffleVector as usize => LLVMInstruction::ShuffleVector,
            v if v == LLVMInstruction::ExtractValue as usize => LLVMInstruction::ExtractValue,
            v if v == LLVMInstruction::InsertValue as usize => LLVMInstruction::InsertValue,
            v if v == LLVMInstruction::LandingPad as usize => LLVMInstruction::LandingPad,
            v if v == LLVMInstruction::Freeze as usize => LLVMInstruction::Freeze,
            v if v == LLVMInstruction::CustomPatchPoint as usize => LLVMInstruction::CustomPatchPoint,
            v if v == LLVMInstruction::InjectedCall as usize => LLVMInstruction::InjectedCall,
            _ => return Err(format!("Unknown LLVM instructions with id {}", value)),
        };
        Ok(ret)
    }
}

impl TryFrom<u16> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let valueu8 = usize::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}

impl TryFrom<u32> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let valueu8 = usize::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}

impl TryFrom<u64> for LLVMInstruction {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let valueu8 = usize::try_from(value);
        match valueu8 {
            Ok(valueu8) => valueu8.try_into(),
            Err(_) => return Err(format!("Unknown LLVM instructions with id {}", value)),
        }
    }
}