use std::sync::mpsc;
use std::{fs, thread};

fn main() {
    println!("Hello, world!");
}

fn start_file_reader_thread(
    documents: Vec<PathBuf>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;

            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(());
    });

    (receiver, handle)
}

fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>)
{
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);
            if sender.send(index).is_err() {
                break;
            }
        }
    });
}



// "The remaining three stages are similar in design. Each one consumes a Receiver cre‐
// ated by the previous stage. We won’t show the code here, just the type signatures of these three
// functions. The full source is online."

fn start_in_memory_merge_thread(file_indexes: mpsc::Receiver<InMemoryIndex>)
                                -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>);

fn start_index_writer_thread(big_indexes: mpsc::Receiver<InMemoryIndex>,
                             output_dir: &Path)
                             -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<io::Result<()>>);

fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path)
                     -> io::Result<()>;

