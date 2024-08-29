mod u31;
pub use u31::*;

mod u31_ext;
pub use u31_ext::*;

use bitcoin_script::{script,define_pushable};
define_pushable!();
use bitcoin::ScriptBuf as Script;
use bitcoin_scriptexec::{Exec, ExecCtx, ExecutionResult, Options, TxTemplate};
use bitcoin::blockdata::transaction;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::hashes::Hash;
// use bitcoin::TapLeafHash;
use bitcoin::taproot::TapLeafHash;

pub fn unroll<F, T>(count: u32, mut closure: F) -> Vec<T>
where
    F: FnMut(u32) -> T,
    T: pushable::Pushable,
{
    let mut result = vec![];

    for i in 0..count {
        result.push(closure(i))
    }
    result
}

pub fn execute_script(script: Script) -> ExecutionResult {
    let mut exec = Exec::new(
        ExecCtx::Tapscript,
        Options::default(),
        TxTemplate {
            tx: Transaction {
                version: transaction::Version::TWO,
                lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
                input: vec![],
                output: vec![],
            },
            prevouts: vec![],
            input_idx: 0,
            taproot_annex_scriptleaf: Some((TapLeafHash::all_zeros(), None)),
        },
        script,
        vec![],
    )
    .expect("error creating exec");

    loop {
        if exec.exec_next().is_err() {
            break;
        }
    }
    let res = exec.result().unwrap();
    res.clone()
}