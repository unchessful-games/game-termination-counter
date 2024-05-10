use std::io::{self, Read};

use futures::stream::TryStreamExt;
use tokio::io::AsyncReadExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::{generic_stats::GameCountingContainer, visitor};
struct BytesStreamReader {
    pub data_recv: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

impl Read for BytesStreamReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let data = self.data_recv.blocking_recv();
            match data {
                Some(d) => {
                    for (src_byte, dst_byte) in d.iter().zip(buf.iter_mut()) {
                        *dst_byte = *src_byte;
                    }
                    return Ok(d.len());
                }
                None => {
                    println!("Decompression thread finished!");
                    return Ok(0);
                }
            }
        }
    }
}
pub async fn download_url<T: GameCountingContainer + 'static>(url: String) -> anyhow::Result<T> {
    let response = reqwest::get(url).await.unwrap();
    let total_len = response.content_length().unwrap_or(1);
    let mut data = response
        .bytes_stream()
        .map_ok(|v| v.to_vec())
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();

    let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u8>>(512 * 1024);
    tokio::spawn(async move {
        let mut buf = [0u8; 1 * 1024];
        let mut downloaded_so_far = 0;
        let total_len_float = total_len as f64;
        loop {
            let len = data.read(&mut buf).await;
            match len {
                Ok(v) => {
                    downloaded_so_far += v;
                    if v > 0 {
                        println!(
                            "Read compressed data so far: {downloaded_so_far} \t/\t{total_len}\t{} %",
                            ((downloaded_so_far as f64) / total_len_float) * 100.0
                        );
                    }
                    let data = Vec::from_iter(buf[..v].iter().map(|v| *v));
                    if let Err(_) = tx.send(data).await {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error while reading compressed data: {e}");
                    drop(tx);
                    break;
                }
            }
        }
    });

    let compressed_data_blocking = BytesStreamReader { data_recv: rx };
    let decompressed_stream = zstd::Decoder::new(compressed_data_blocking).unwrap();

    let handle =
        tokio::task::spawn_blocking(move || visitor::visit_reader(decompressed_stream).unwrap());
    Ok(handle.await?)
}
