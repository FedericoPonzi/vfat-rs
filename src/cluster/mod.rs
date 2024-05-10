//! Helpers to read and write cluster chains
//! these two struct are very similar yet different.
//! The most noticeable difference is that writer will allocate new clusters
//! as we keep writing to it, whereas the reader will stop when have finished reading the chain.
pub mod cluster_reader;
pub mod cluster_writer;
