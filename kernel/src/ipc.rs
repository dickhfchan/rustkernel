// Port-based asynchronous IPC for microkernel

use spin::Mutex;

pub type PortId = u32;
pub type ProcessId = u32;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: ProcessId,
    pub data: [u8; 256], // Fixed size for now
    pub len: usize,
}

pub struct Port {
    id: PortId,
    owner: ProcessId,
    // TODO: Replace with proper queue once we have heap allocator
    message_buffer: Mutex<Option<Message>>,
}

pub fn init() {
    crate::println!("Initializing IPC system...");
    
    // TODO: Initialize port table
    // TODO: Set up message queues
    // TODO: Initialize async notification system
    
    crate::println!("IPC system initialized");
}

impl Port {
    pub fn new(id: PortId, owner: ProcessId) -> Self {
        Self {
            id,
            owner,
            message_buffer: Mutex::new(None),
        }
    }
    
    pub fn send_message(&self, message: Message) -> Result<(), &'static str> {
        let mut buffer = self.message_buffer.lock();
        if buffer.is_some() {
            return Err("Port buffer full");
        }
        *buffer = Some(message);
        Ok(())
    }
    
    pub fn receive_message(&self) -> Option<Message> {
        self.message_buffer.lock().take()
    }
}
