use colored::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{PathBuf, *};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
//
// GEI FILES
fn get_filenames(reverse: bool) -> Vec<String> {
    let mut filenames = vec![];
    let range = 0..100;
    filenames.reserve(range.len());

    if reverse {
        for i in range.rev() {
            filenames.push(format!("file_{}.txt", i));
        }
    } else {
        for i in range {
            filenames.push(format!("file_{}.txt", i));
        }
    }

    filenames
}
//

// WIP  with crash need to deal how to handle error
pub fn read_file(filename: &str, thread_id: usize) -> Result<String, io::Error> {
    return Ok(format!("Thread:  {}, File: {}", thread_id, filename));
}

pub fn process_file(filename: &str, thread_id: usize) -> io::Result<()> {
    println!("{}", read_file(filename, thread_id)?);
    // println!("Thread:  {}, File: {}", thread_id, filename);
    // if thread_id == 5 {
    //     return Err(io::Error::new(
    //         io::ErrorKind::Other,
    //         format!("Error, thread: {}, file: {}", thread_id, filename),
    //     ));
    // }
    Ok(())
}

// nothing to return ( no communcation between threads)
fn process_worklist(worklist: Vec<String>, thread_id: usize) -> io::Result<()> {
    for filename in worklist {
        process_file(&filename, thread_id)?;
    }
    Ok(())
}

// nothing to return ( no communcation between threads)
pub fn process_files_handles(filenames: Vec<String>) -> io::Result<()> {
    // Divide the work into threads
    const NTHREADS: usize = 8;
    let chunk_size = (filenames.len() + NTHREADS - 1) / NTHREADS;
    let worklists = filenames.chunks(chunk_size);
    //
    // Split
    let mut thread_handles = vec![];
    for (i, worklist) in worklists.enumerate() {
        let worklist = worklist.to_owned();
        thread_handles.push(thread::spawn(move || {
            process_worklist(worklist, i).expect("Error processing worklist");
        }));
    }

    // join
    for handle in thread_handles {
        match handle.join() {
            Ok(_) => (),
            Err(_) => println!("Thread panicked"),
        };
    }
    Ok(())
}

//
// RAYON
//
fn process_files_rayon(filenames: Vec<String>) -> io::Result<()> {
    // Divide the work into threads
    // Split
    filenames.par_iter().for_each(|file| {
        process_file(file, 0).unwrap();
        return ();
    });
    Ok(())
}

// Other version using map reduce (similar timing as for_each
fn process_files_rayon_map_reduce(filenames: Vec<String>) -> io::Result<()> {
    // Divide the work into threads
    // Split
    filenames
        .par_iter()
        .map(|file| process_file(file, 0))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 });
    Ok(())
}

// Channels sender received
// Creation of thread that will communication with a receiver
// Will generate strings (will only pass the 3 u64 bytes to the receiver)
fn start_file_read_thread(
    filenames: Vec<String>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    // Generate a thread that reas all the files
    let handle = thread::spawn(move || {
        for filename in filenames {
            let text = read_file(&filename, 0)?;

            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}

// no need to have a returned value as no error during processing
fn start_processing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        for (index, text) in texts.into_iter().enumerate() {
            let new_text = format!("proc: {} -> {}", index, text).to_owned();
            if sender.send(new_text).is_err() {
                break;
            }
        }
    });

    (receiver, handle)
}

fn merge_processed_texts(texts: mpsc::Receiver<String>) -> io::Result<String> {
    let merged = texts.into_iter().collect::<Vec<String>>().join("\n");

    Ok(merged)
}

fn main() {
    let mut summary = HashMap::<String, Duration>::new();
    {
        println!("{:=^49}", " FORK-JOIN ".green());

        let reverse = false;
        let filenames = get_filenames(reverse);
        let start = Instant::now();
        process_files_handles(filenames).unwrap();
        let duration = start.elapsed();
        summary.insert("fork-join".to_string(), duration);
    }

    {
        println!("{:=^49}", " FORK RAYON ".green());
        let reverse = false;
        let filenames = get_filenames(reverse);

        let start = Instant::now();
        process_files_rayon(filenames).unwrap();
        let duration = start.elapsed();
        summary.insert("fork-rayon".to_string(), duration);
    }

    {
        println!("{:=^49}", " FORK RAYON MAP REDUCE ".green());
        let reverse = false;
        let filenames = get_filenames(reverse);

        let start = Instant::now();
        process_files_rayon_map_reduce(filenames).unwrap();
        let duration = start.elapsed();
        summary.insert("fork-rayon-map-reduce".to_string(), duration);
    }
    println!("{::^49}", "Summary ".yellow());
    println!("{}", format!("{:#?}", summary).yellow());
    println!("{::^49}", "".yellow());

    {
        println!("{:=^49}", " CHANNELS SEQUENTIAL ".green());
        let reverse = true;
        let filenames = get_filenames(reverse);

        // Create pipeline
        let (file_text_recvr, h1) = start_file_read_thread(filenames);
        let (proc_text_recvr, h2) = start_processing_thread(file_text_recvr);
        let result = merge_processed_texts(proc_text_recvr);

        // Wait for threads to finish
        let r1 = h1.join().unwrap();
        h2.join().unwrap();

        // check that error is encountered
        // r1?;

        println!("{}", result.expect("BAD MERGE").yellow());
    }
}
