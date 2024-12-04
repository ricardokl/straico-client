# Straico Client

This repository contains a Rust client for interacting with the Straico API. The client is designed to be flexible and supports various endpoints such as chat completions, image generation, file uploads, model information, and user data retrieval.

## Features

- **Prompt Completions**: Send prompts to multiple models and receive responses.
- **Image Generation**: Generate images using the Straico API.
- **File Uploads**: Upload files to the Straico API.
- **Model Information**: Fetch available models from the Straico API.
- **User Data**: Retrieve user information from the Straico API.

## Proxy Server

The repository includes a proxy server that allows you to send OpenAI-compatible requests and receive compatible responses. This server acts as an intermediary, translating OpenAI-style requests into the format expected by the Straico API and vice versa.

### Usage

1. **Set Up Environment Variables**:
   - Ensure you have the `STRAICO_API_KEY` environment variable set with your Straico API key.

2. **Run the Proxy Server**:

Option 1: Download and run the release binary
- Download the latest release binary for your platform from the GitHub releases page

- For Linux:
  ```sh
  chmod +x straico-proxy
  ./straico-proxy
  ```

- For macOS:
  ```sh
  chmod +x straico-proxy
  ./straico-proxy
  ```

- For Windows:
  - Download the .exe file
  - Double click the executable or run from command prompt:
  ```cmd
  straico-proxy.exe
  ```

  Option 2: Install via cargo
  - Install directly from GitHub:
    ```sh
    cargo install --git https://github.com/straico/straico-client
    straico-proxy
    ```

  - Or clone and install locally:
    ```sh
    git clone https://github.com/straico/straico-client
    cd straico-client
    cargo install --path .
    straico-proxy
    ```

3. **Send Requests**:
   - Use any HTTP client to send requests to the proxy server.
   - Example request to the chat completion endpoint:
     ```sh
     curl -X POST http://localhost:8000/v1/chat/completions \
     -H "Content-Type: application/json" \
     -d '{
       "model": "gpt-3.5-turbo",
       "messages": [
         {"role": "user", "content": "What is the answer to life, the universe, and everything?"}
       ],
       "max_tokens": 1000,
       "temperature": 0.5
     }'
     ```

### Response Format

The proxy server ensures that responses from the Straico API are formatted to be compatible with OpenAI's response structure. This includes handling of completion data, error messages, and other relevant fields.

### Tools

Tools usage is currently considered experimental.

### Streaming

Streaming is currently not implemented.

## Examples

The `examples` directory contains several example programs demonstrating how to use the Straico client directly without the proxy server:

- **Chat Completions**: `completion.rs`
- **Image Generation**: `image.rs`
- **File Uploads**: `file.rs`
- **Model Information**: `models.rs`
- **User Data**: `user.rs`

### Running Examples

To run an example, navigate to the `examples` directory and use the following command:

```sh
cargo run --example <example_name>
```

Replace `<example_name>` with the name of the example you want to run (e.g., `completion`).
