pub mod file;
pub mod hash;

// There are files who have their own unique index.
// The files are divided into chunks, which also have their own unique index.
// The chunk index of a Xth file's Yth chunk is X * MAX_CHUNK_COUNT + Y
// That means a size of each file cannot exceed CHUNK_SIZE * MAX_CHUNK_COUNT bytes.