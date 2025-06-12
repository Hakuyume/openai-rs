// https://platform.openai.com/docs/api-reference/chat/create
#[test]
fn test_create_chat_completion_request_default() {
    assert_eq!(
        crate::CreateChatCompletionRequest::builder()
            .model("gpt-4.1".to_owned())
            .messages(vec![
                crate::ChatCompletionRequestMessage::Developer(
                    crate::ChatCompletionRequestDeveloperMessage::builder()
                        .content(crate::ChatCompletionRequestDeveloperMessageContent::_0(
                            "You are a helpful assistant.".to_owned()
                        ))
                        .build()
                ),
                crate::ChatCompletionRequestMessage::User(
                    crate::ChatCompletionRequestUserMessage::builder()
                        .content(crate::ChatCompletionRequestUserMessageContent::_0(
                            "Hello!".to_owned()
                        ))
                        .build()
                ),
            ])
            .build(),
        serde_json::from_str(
            r#"{
  "model": "gpt-4.1",
  "messages": [
    {
      "role": "developer",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "Hello!"
    }
  ]
}"#,
        )
        .unwrap()
    );
}

// https://platform.openai.com/docs/api-reference/chat/create
#[test]
fn test_create_chat_completion_response_default() {
    serde_json::from_str::<crate::CreateChatCompletionResponse>(
        r#"{
  "id": "chatcmpl-B9MBs8CjcvOU2jLn4n570S5qMJKcT",
  "object": "chat.completion",
  "created": 1741569952,
  "model": "gpt-4.1-2025-04-14",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I assist you today?",
        "refusal": null,
        "annotations": []
      },
      "logprobs": null,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 19,
    "completion_tokens": 10,
    "total_tokens": 29,
    "prompt_tokens_details": {
      "cached_tokens": 0,
      "audio_tokens": 0
    },
    "completion_tokens_details": {
      "reasoning_tokens": 0,
      "audio_tokens": 0,
      "accepted_prediction_tokens": 0,
      "rejected_prediction_tokens": 0
    }
  },
  "service_tier": "default"
}"#,
    )
    .unwrap();
}
