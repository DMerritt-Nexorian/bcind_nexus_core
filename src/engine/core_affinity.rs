use libc::{CPU_SET, CPU_ZERO, cpu_set_t, sched_setaffinity};

/// Pins the current thread strictly to a specified CPU core.
pub fn pin_to_core(core_id: usize) -> Result<(), i32> {
    unsafe {
        let mut set: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut set);
        CPU_SET(core_id, &mut set);
        let res = sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &set);
        if res != 0 {
            return Err(*libc::__errno_location());
        }
    }
    Ok(())
}

/// Fixed-capacity, zero-allocation ring buffer for deterministic latency.
pub struct StaticRingBuffer<T, const N: usize> {
    storage: [T; N],
    head: usize,
    tail: usize,
}

impl<T: Copy + Default, const N: usize> StaticRingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            storage: [T::default(); N],
            head: 0,
            tail: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        let next_head = (self.head + 1) % N;
        if next_head == self.tail {
            return Err("Buffer overflow: Hard boundary reached");
        }
        self.storage[self.head] = item;
        self.head = next_head;
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let item = self.storage[self.tail];
        self.tail = (self.tail + 1) % N;
        Some(item)
    }
}

impl<T: Copy + Default, const N: usize> Default for StaticRingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
