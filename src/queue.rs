use com::WeakPtr;
use command_list::CommandList;
use sync::Fence;
use winapi::um::d3d12;
use HRESULT;

#[repr(u32)]
pub enum Priority {
    Normal = d3d12::D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
    High = d3d12::D3D12_COMMAND_QUEUE_PRIORITY_HIGH,
    GlobalRealtime = d3d12::D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME,
}

bitflags! {
    pub struct CommandQueueFlags: u32 {
        const DISABLE_GPU_TIMEOUT = d3d12::D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT;
    }
}

pub type CommandQueue = WeakPtr<d3d12::ID3D12CommandQueue>;

impl CommandQueue {
    pub fn signal(&self, fence: Fence, value: u64) -> HRESULT {
        unsafe { self.Signal(fence.as_mut_ptr(), value) }
    }

    pub fn execute_command_lists(&self, command_lists: &[CommandList]) {
        unsafe { self.ExecuteCommandLists(command_lists.len() as _, command_lists as *const _ as *const *mut _); }
    }
}
