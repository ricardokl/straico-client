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
   - Navigate to the `src` directory.
   - Run the server using the command:
     ```sh
     cargo run
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

### Endpoints

- **Chat Completions**: `/v1/chat/completions`
- **Image Generation**: `/v0/images/generate`
- **File Uploads**: `/v1/files/upload`
- **Model Information**: `/v1/models`
- **User Data**: `/v0/user`

### Response Format

The proxy server ensures that responses from the Straico API are formatted to be compatible with OpenAI's response structure. This includes handling of completion data, error messages, and other relevant fields.

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

## Dependencies

- `anyhow`: For error handling.
- `actix-web`: For building the proxy server.
- `reqwest`: For making HTTP requests to the Straico API.
- `serde`: For serialization and deserialization of JSON data.
- `tokio`: For asynchronous runtime.

## Contributing

Contributions to this repository are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
