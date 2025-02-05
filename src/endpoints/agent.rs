pub mod agent_details;
pub mod completion;
pub mod create;
pub mod list_agents;
pub mod rag_to_agent;

// Delete Agents does not require implementation since it responds just a message
// Update Agent just uses the same requests fields as Create Agent, and the same response as Agent Details
