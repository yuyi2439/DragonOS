use bitmap::{traits::BitMapOps, AllocBitmap};
use system_error::SystemError;

use crate::{
    libs::cpumask::CpuMask,
    process::{Pid, ProcessFlags, ProcessManager},
    syscall::Syscall,
};

impl Syscall {
    pub fn getaffinity(pid: usize, set: &mut [u8]) -> Result<usize, SystemError> {
        let pid = Pid::from(pid);
        let pcb = ProcessManager::find(pid);
        if let None = pcb {
            return Err(SystemError::ESRCH);
        }
        let pcb = pcb.unwrap();

        let binding = pcb.sched_info().inner_lock_read_irqsave();
        let src = unsafe { binding.cpu_mask.inner().as_bytes() };
        set[0..src.len()].copy_from_slice(src);
        Ok(0)
    }

    pub fn setaffinity(pid: usize, set: &[u8]) -> Result<usize, SystemError> {
        let pid = Pid::from(pid);
        let pcb = ProcessManager::find(pid);
        if let None = pcb {
            return Err(SystemError::ESRCH);
        }
        let pcb = pcb.unwrap();
        if pcb.flags().contains(ProcessFlags::NO_SETAFFINITY) {
            return Err(SystemError::EINVAL);
        }

        let bmp = AllocBitmap::from_bytes(set);
        let mask = CpuMask::from_bitmap(bmp);
        pcb.sched_info().inner_lock_write_irqsave().cpu_mask = mask;

        Ok(0)
    }
}
