// https://platform.openai.com/docs/api-reference/chat/create
#[test]
fn test_create_chat_completion_request_default() {
    let a = serde_json::json!({
        "model": "gpt-4.1",
        "messages": [
            {
                "role": "developer",
                "content": "You are a helpful assistant.",
            },
            {
                "role": "user",
                "content": "Hello!",
            },
        ],
    });
    let b = crate::CreateChatCompletionRequest::builder()
        .model("gpt-4.1".to_owned())
        .messages(vec![
            crate::ChatCompletionRequestMessage::Developer(
                crate::ChatCompletionRequestDeveloperMessage::builder()
                    .content(crate::ChatCompletionRequestDeveloperMessageContent::_0(
                        "You are a helpful assistant.".to_owned(),
                    ))
                    .build(),
            ),
            crate::ChatCompletionRequestMessage::User(
                crate::ChatCompletionRequestUserMessage::builder()
                    .content(crate::ChatCompletionRequestUserMessageContent::_0(
                        "Hello!".to_owned(),
                    ))
                    .build(),
            ),
        ])
        .build();
    assert_eq!(a, serde_json::to_value(&b).unwrap());
    assert_eq!(b, serde_json::from_value(a.clone()).unwrap());
}

// https://platform.openai.com/docs/api-reference/chat/create
#[test]
fn test_create_chat_completion_response_default() {
    let a = serde_json::json!({
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
                    // "refusal": null,
                    "annotations": [],
                },
                // "logprobs": null,
                "finish_reason": "stop",
            },
        ],
        "usage": {
            "prompt_tokens": 19,
            "completion_tokens": 10,
            "total_tokens": 29,
            "prompt_tokens_details": {
                "cached_tokens": 0,
                "audio_tokens": 0,
            },
            "completion_tokens_details": {
                "reasoning_tokens": 0,
                "audio_tokens": 0,
                "accepted_prediction_tokens": 0,
                "rejected_prediction_tokens": 0,
            },
        },
        "service_tier": "default",
    });
    let b = crate::CreateChatCompletionResponse::builder()
        .id("chatcmpl-B9MBs8CjcvOU2jLn4n570S5qMJKcT".to_owned())
        .created(1741569952)
        .model("gpt-4.1-2025-04-14".to_owned())
        .choices(vec![
            crate::CreateChatCompletionResponseChoicesItem::builder()
                .index(0)
                .message(
                    crate::ChatCompletionResponseMessage::builder()
                        .content(Some("Hello! How can I assist you today?".to_owned()))
                        .annotations(Some(Vec::new()))
                        .build(),
                )
                .finish_reason(crate::CreateChatCompletionResponseChoicesItemFinishReason::Stop)
                .build(),
        ])
        .usage(Some(
            crate::CompletionUsage::builder()
                .prompt_tokens(19)
                .completion_tokens(10)
                .total_tokens(29)
                .prompt_tokens_details(Some(
                    crate::CompletionUsagePromptTokensDetails::builder()
                        .cached_tokens(Some(0))
                        .audio_tokens(Some(0))
                        .build(),
                ))
                .completion_tokens_details(Some(
                    crate::CompletionUsageCompletionTokensDetails::builder()
                        .reasoning_tokens(Some(0))
                        .audio_tokens(Some(0))
                        .accepted_prediction_tokens(Some(0))
                        .rejected_prediction_tokens(Some(0))
                        .build(),
                ))
                .build(),
        ))
        .service_tier(Some(crate::ServiceTier::Default))
        .build();
    assert_eq!(a, serde_json::to_value(&b).unwrap());
    assert_eq!(b, serde_json::from_value(a.clone()).unwrap());
}
