use std::{error::Error, fs::{self, File}, io::{BufRead, BufReader, Seek, SeekFrom}, sync::{Arc, atomic::{AtomicUsize, Ordering}}, thread};

fn process_chunk(reader: &mut BufReader<File>, start: u64, chunk_size: u64) -> usize{
    
    reader.seek(SeekFrom::Start(start)).expect("Can't read file");

    let mut current_pos = start;
    let mut count = 0;
    let mut line = String::new();

    // Reads some bytes until the end of the line at the initial chunk so that we can start from
    // the new line.
    // IDK HOW I CAME UP WITH THIS
    if start!=0 {
        let bytes_read = reader.read_line(&mut line).expect("Can't read the file") as u64;
        current_pos += bytes_read;
    }

    let end = start + chunk_size; 
    
    while current_pos<=end {
        line.clear();
        let bytes_read = reader.read_line(&mut line).expect("Can't read file") as u64;

        if bytes_read == 0 {
            break;
        }

        current_pos += bytes_read;

        count += line.to_lowercase().matches("error").count();
    }

    return count;
}

fn main() -> Result<(), Box<dyn Error>>{
    let file_size  = fs::metadata("test.txt")
        .expect("Cant  read the metadata of the file")
        .len();



    let threads = num_cpus::get() as u64;

    let base_chunk = file_size / threads;
    let remainder = file_size % threads;

    let mut handles = Vec::new();
   
    let total_count = Arc::new(AtomicUsize::new(0));

    for count in 0..threads {
        let file = File::open("test.txt").expect("Can't read the file");        
        let main_counter = Arc::clone(&total_count);

        let handle = thread::spawn(move|| {
            let mut start : u64 = 0;

            start += count * base_chunk;

            let mut reader = BufReader::new(file);

            let chunk_size = if count == 0 {
                    remainder + base_chunk
                } else {
                    base_chunk
                };

            let word_counter = process_chunk(&mut reader, start, chunk_size);

            main_counter.fetch_add(word_counter, Ordering::SeqCst);
        
            
        });

        handles.push(handle);
    } 

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total Error Counter: {}", total_count.load(Ordering::Relaxed));

    Ok(())
}
