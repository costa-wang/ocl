
extern crate futures;
extern crate ocl;
extern crate ocl_extras;

use futures::future::poll_fn;
use futures::executor::block_on;
use std::thread;
use std::time::Duration;
use std::{pin::Pin, task::Context as FutureContext, task::Poll};
use ocl::{Platform, Device, Context, Program, Queue, Kernel, Buffer, EventList, Event, MemFlags};
use ocl::r#async::{BufferSink, WriteGuard};

// Our arbitrary data set size (about a million) and coefficent:
const WORK_SIZE: usize = 1 << 20;
const COEFF: i32 = 321;

const THREAD_COUNT: usize = 32;

// Our kernel source code:
static KERNEL_SRC: &'static str = r#"
    __kernel void multiply_by_scalar(
            __private int const coeff,
            __global int const* const src,
            __global int* const res)
    {
        uint const idx = get_global_id(0);
        res[idx] = src[idx] * coeff;
    }
"#;

 fn buffer_sink() -> ocl::Result<()> {
     let platform =  Platform::list()[0];
     let device  =  Device::list_all(&platform)?[0];
     let context = Context::builder().platform(platform).devices(device).build()?;
    //  let queue = Queue::new(&context, device, Some(ocl::core::QUEUE_PROFILING_ENABLE))?;    
    let queue = Queue::new(&context, device, Some(ocl::core::QUEUE_PROFILING_ENABLE))?;
    let source_buffer = Buffer::<i32>::builder()
            .queue(queue.clone())
            .flags(MemFlags::new().read_write().alloc_host_ptr())
            .len(WORK_SIZE)
            .build()?;
    
    let result_buffer = Buffer::<i32>::builder()
            .queue(queue.clone())
            .flags(MemFlags::new().read_write().alloc_host_ptr())
            .len(WORK_SIZE)
            .build()?;
    
    let program = Program::builder()
            .src(KERNEL_SRC)
            .devices(device)
            .build(&context)?;

    let kernel = Kernel::builder()
            .name("multiply_by_scalar")
            .program(&program)
            .queue(queue.clone())
            .global_work_size(WORK_SIZE)
            .arg(COEFF)
            .arg(&source_buffer)
            .arg(&result_buffer)
            .build()?;
    
    let mut event_list = EventList::new();
    unsafe { kernel.cmd().enew(&mut event_list).enq()?; }
    event_list.wait_for()?;

    // let mut event = Event::empty();
    // source_buffer.cmd().write(&vec![0; WORK_SIZE]).enew(&mut event).enq()?;
    // event.wait_for()?;

    assert_eq!(kernel.default_global_work_size().to_len(), WORK_SIZE);

    let buffer_sink = unsafe {
        BufferSink::from_buffer(source_buffer.clone(), Some(queue.clone()), 0,
            WORK_SIZE)?
    };
    let source_data = ocl_extras::scrambled_vec((0, 20), WORK_SIZE);
    // let source_datas: Vec<_> = (0..THREAD_COUNT).map(|_| {
    //     ocl_extras::scrambled_vec((0, 20), ocl_pq.dims().to_len())
    // }).collect();
    let mut vec_result = vec![0i32; WORK_SIZE];
    buffer_sink.buffer().read(&mut vec_result).enq()?;
    let mut writer_0 = buffer_sink.clone().write();
    // writer_0.set_lock_wait_events(event_list);

    // if let Some(event_list) = writer_0.wait_events(){
        
    // }else{
    //     println!("1");
    // }
    // thread::sleep(Duration::from_millis(2000));
    // let mut write_guard = block_on(async{ writer_0.await });
    // print_type_name(&write_guard);

    

    // print_type_name(&write_guard);
    // write_guard.copy_from_slice(&[0i32; WORK_SIZE]);
    // let buffer_sink: BufferSink<_> = WriteGuard::release(write_guard).into();
    // block_on(async{buffer_sink.flush().enq().unwrap()});
    

    // let source_data = source_datas[i].clone();
    let writer_1 = buffer_sink.clone().write();
    // let mut write_guard = block_on(async{writer_1.await});
    // write_guard.copy_from_slice(&source_data);
    // let buffer_sink: BufferSink<_> = WriteGuard::release(write_guard).into();
    // block_on(async{buffer_sink.flush().enq().unwrap()});

// for (&src, &res) in source_data.iter().zip(vec_result.iter()) {
//     assert_eq!(src * COEFF, res);
// }
    Ok(())
}

pub fn main() {
    match buffer_sink() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
    // thread::sleep(Duration::from_millis(1000));
}


fn print_type_name<T>(_: &T) {
    println!("{}", std::any::type_name::<T>() );
}