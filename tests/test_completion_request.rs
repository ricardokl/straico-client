use straico_client::endpoints::completion::completion_request::CompletionRequest;

#[test]
fn test_completion_request_builder_all_fields() {
    let request = CompletionRequest::new()
        .models(["openai/gpt-3.5-turbo"])
        .message("Hello, world!")
        .file_urls(&["http://example.com/file.txt"])
        .youtube_urls(&["http://youtube.com/watch?v=123"])
        .display_transcripts(true)
        .temperature(0.8)
        .max_tokens(1024)
        .build();

    assert_eq!(request.get_max_tokens(), Some(&1024));
    assert_eq!(request.get_temperature(), Some(&0.8));
    assert_eq!(request.get_display_transcripts(), Some(&true));
    assert_eq!(request.get_file_urls().unwrap(), &vec!["http://example.com/file.txt"]);
    assert_eq!(request.get_youtube_urls().unwrap(), &vec!["http://youtube.com/watch?v=123"]);
}

#[test]
fn test_completion_request_builder_required_fields_only() {
    let request = CompletionRequest::new()
        .models(["openai/gpt-3.5-turbo"])
        .message("Hello, world!")
        .build();

    assert_eq!(request.get_max_tokens(), None);
    assert_eq!(request.get_temperature(), None);
    assert_eq!(request.get_display_transcripts(), None);
    assert_eq!(request.get_file_urls(), None);
    assert_eq!(request.get_youtube_urls(), None);
}
