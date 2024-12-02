use metal::{
    Buffer, CommandBuffer, ComputeCommandEncoderRef, ComputePassDescriptor,
    ComputePassDescriptorRef, CounterSampleBuffer, CounterSampleBufferDescriptor,
    CounterSampleBufferRef, Device, MTLResourceOptions, NSRange,
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

const NUM_SAMPLES: u64 = 2;

fn handle_compute_pass_sample_buffer_attachment(
    compute_pass_descriptor: &ComputePassDescriptorRef,
    counter_sample_buffer: &CounterSampleBufferRef,
) {
    let sample_buffer_attachment_descriptor =
        compute_pass_descriptor.sample_buffer_attachments().object_at(0).unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_encoder_sample_index(0);
    sample_buffer_attachment_descriptor.set_end_of_encoder_sample_index(1);
}

#[derive(Debug, Clone)]
pub struct MetalProfiler {
    device: Device,
    profile_buffers: HashMap<usize, Vec<Buffer>>,
    current_node_id: Option<usize>,
    counter_sample_buffer: CounterSampleBuffer,
    compute_pass_descriptor: ComputePassDescriptor,
}

impl MetalProfiler {
    pub fn new(device: Device) -> Self {
        let compute_pass_descriptor = ComputePassDescriptor::new();

        let counter_sample_buffer_desc = CounterSampleBufferDescriptor::new();
        counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
        counter_sample_buffer_desc.set_sample_count(NUM_SAMPLES);
        let counter_sets = device.counter_sets();
        let timestamp_counter = counter_sets.iter().find(|cs| cs.name() == "timestamp");

        counter_sample_buffer_desc
            .set_counter_set(timestamp_counter.expect("No timestamp counter found"));

        let counter_sample_buffer =
            device.new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc).unwrap();

        handle_compute_pass_sample_buffer_attachment(
            compute_pass_descriptor,
            &counter_sample_buffer,
        );

        Self {
            device: device.to_owned(),
            profile_buffers: HashMap::new(),
            current_node_id: None,
            counter_sample_buffer,
            compute_pass_descriptor: compute_pass_descriptor.to_owned(),
        }
    }

    pub fn add_node_entry(&mut self, node_id: usize) {
        self.profile_buffers.insert(node_id, vec![]);
        self.current_node_id = Some(node_id);
    }

    pub fn add_buffer(&mut self, buffer: Buffer) {
        let current_node_id = &self.current_node_id.unwrap();
        let node_values = self
            .profile_buffers
            .get_mut(current_node_id)
            .unwrap_or_else(|| panic!("No buffer found for node ID: {}", &current_node_id));
        node_values.push(buffer);
    }

    pub fn get_profile_data(&self) -> HashMap<usize, u64> {
        let mut formatted_hashmap: HashMap<usize, u64> = HashMap::new();
        self.profile_buffers.borrow().iter().for_each(|(key, v)| {
            let mut node_duration_ns = 0;
            v.iter().for_each(|buffer| unsafe {
                let slice = std::slice::from_raw_parts(
                    buffer.contents() as *const u64,
                    NUM_SAMPLES as usize,
                );
                node_duration_ns += slice[1] - slice[0];
            });
            formatted_hashmap.insert(*key, node_duration_ns);
        });
        formatted_hashmap
    }
}

#[derive(Debug, Clone)]
pub struct TCommandBuffer {
    inner: CommandBuffer,
    profiler: Option<Rc<RefCell<MetalProfiler>>>,
}

impl TCommandBuffer {
    pub fn new(
        command_buffer: CommandBuffer,
        profiler: Option<Rc<RefCell<MetalProfiler>>>,
    ) -> Self {
        TCommandBuffer { inner: command_buffer, profiler }
    }

    pub fn encode<EncodeCallback>(&self, encode_cb: EncodeCallback)
    where
        EncodeCallback: Fn(&ComputeCommandEncoderRef),
    {
        if let Some(profiler) = &self.profiler {
            let mut profiler = profiler.borrow_mut();

            let destination_buffer = profiler.device.new_buffer(
                (std::mem::size_of::<u64>() * NUM_SAMPLES as usize) as u64,
                MTLResourceOptions::StorageModeShared,
            );

            let encoder = self
                .inner
                .compute_command_encoder_with_descriptor(&profiler.compute_pass_descriptor);

            encode_cb(encoder);

            let blit_encoder = self.inner.new_blit_command_encoder();
            blit_encoder.resolve_counters(
                &profiler.counter_sample_buffer,
                NSRange::new(0_u64, NUM_SAMPLES),
                &destination_buffer,
                0_u64,
            );
            blit_encoder.end_encoding();

            profiler.add_buffer(destination_buffer);
        } else {
            let encoder = self.inner.new_compute_command_encoder();
            encode_cb(encoder);
        };
    }
}

impl Deref for TCommandBuffer {
    type Target = CommandBuffer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TCommandBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
