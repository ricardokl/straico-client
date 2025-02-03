pub mod completion;
pub mod create;
pub mod list;
pub mod rag_by_id;

// Delete Rag doesn't need anything, there's no request body or response data other than a message
// Update Rag uses the same structure as Create
