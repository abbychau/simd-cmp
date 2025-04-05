#![feature(portable_simd)]

use clap::Parser;
use std::simd::u8x32; // Proper import for std::simd
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::process;

#[derive(Parser)]
#[command(about = "Compare two files using SIMD acceleration")]
struct Args {
    /// First file to compare
    file1: String,
    
    /// Second file to compare
    file2: String,
    
    /// Quiet mode - only return exit code
    #[arg(short = 's', long = "silent")]
    silent: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.file1 == args.file2 {
        if !args.silent {
            println!("Files are identical");
        }
        process::exit(0);
    }
    
    match compare_files(&args.file1, &args.file2, args.silent) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn compare_files(file1_path: &str, file2_path: &str, silent: bool) -> io::Result<()> {
    let mut file1 = File::open(file1_path)?;
    let mut file2 = File::open(file2_path)?;
    
    // Use 4KB buffers (multiple of SIMD width)
    const BUFFER_SIZE: usize = 4096;
    let mut buffer1 = [0u8; BUFFER_SIZE];
    let mut buffer2 = [0u8; BUFFER_SIZE];
    
    let mut offset: u64 = 0;
    
    loop {
        let bytes_read1 = read_chunk(&mut file1, &mut buffer1)?;
        let bytes_read2 = read_chunk(&mut file2, &mut buffer2)?;
        
        // If we've reached EOF for both files and they match so far, files are identical
        if bytes_read1 == 0 && bytes_read2 == 0 {
            return Ok(());
        }
        
        // If files have different lengths but match up to the shorter one
        if bytes_read1 != bytes_read2 {
            if !silent {
                println!("cmp: EOF on {}", if bytes_read1 < bytes_read2 { file1_path } else { file2_path });
            }
            return Err(io::Error::new(ErrorKind::Other, "Files have different sizes"));
        }
        
        // Compare the buffers using SIMD to find the first difference
        match compare_buffers_simd(&buffer1[..bytes_read1], &buffer2[..bytes_read2]) {
            Some(pos) => {
                if !silent {
                    let byte_pos = offset + pos as u64;
                    println!("{} {} {} differ: {:o} {:o}", 
                        file1_path, file2_path, byte_pos + 1, buffer1[pos], buffer2[pos]);
                }
                return Err(io::Error::new(ErrorKind::Other, "Files differ"));
            }
            None => {
                // No difference found in this chunk, continue with next chunk
                offset += bytes_read1 as u64;
            }
        }
    }
}

fn read_chunk(file: &mut File, buffer: &mut [u8]) -> io::Result<usize> {
    let mut total_read = 0;
    
    while total_read < buffer.len() {
        match file.read(&mut buffer[total_read..]) {
            Ok(0) => break, // EOF reached
            Ok(n) => total_read += n,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(total_read)
}

fn compare_buffers_simd(buf1: &[u8], buf2: &[u8]) -> Option<usize> {
    let len = buf1.len();
    let mut i = 0;
    
    // Process 32 bytes at a time using SIMD
    while i + 32 <= len {
        // std::simd doesn't have from_slice_unaligned, use from_slice
        let v1 = u8x32::from_slice(&buf1[i..i+32]);
        let v2 = u8x32::from_slice(&buf2[i..i+32]);
        
        // Compare 32 bytes at once
        let mask = v1 != v2; // This produces a mask where true = bytes differ
        if mask {
            // At least one byte differs - find which one
            for j in 0..32 {
                if v1[j] != v2[j] {
                    return Some(i + j);
                }
            }
        }
        
        i += 32;
    }
    
    // Handle remaining bytes individually
    for j in i..len {
        if buf1[j] != buf2[j] {
            return Some(j);
        }
    }
    
    None
}
