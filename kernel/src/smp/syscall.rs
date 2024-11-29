use bitmap::traits::BitMapOps;
use log::debug;
use system_error::SystemError;

use crate::{
    process::{Pid, ProcessManager},
    syscall::Syscall,
};

impl Syscall {
    pub fn getaffinity(pid: usize, set: &mut [u8]) -> Result<usize, SystemError> {
        let pcb = ProcessManager::find(Pid::from(pid)).unwrap_or(ProcessManager::current_pcb());
        let binding = pcb.sched_info().cpumask();
        let mask = binding.read();
        let src = unsafe { mask.inner().as_bytes() };
        debug!("{:?}", src);
        set[0..src.len()].copy_from_slice(src);
        Ok(0)
    }
}
