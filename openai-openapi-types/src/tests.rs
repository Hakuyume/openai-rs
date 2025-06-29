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
        .model(crate::ModelIdsShared::Gpt4_1)
        .messages(vec![
            crate::ChatCompletionRequestMessage::Developer(
                crate::ChatCompletionRequestDeveloperMessage::builder()
                    .content(crate::ChatCompletionRequestDeveloperMessageContent::String(
                        "You are a helpful assistant.".to_owned(),
                    ))
                    .build(),
            ),
            crate::ChatCompletionRequestMessage::User(
                crate::ChatCompletionRequestUserMessage::builder()
                    .content(crate::ChatCompletionRequestUserMessageContent::String(
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
            crate::CreateChatCompletionResponseChoice::builder()
                .index(0)
                .message(
                    crate::ChatCompletionResponseMessage::builder()
                        .content(Some("Hello! How can I assist you today?".to_owned()))
                        .annotations(Some(Vec::new()))
                        .build(),
                )
                .finish_reason(crate::CreateChatCompletionResponseChoiceFinishReason::Stop)
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

// https://platform.openai.com/docs/api-reference/responses
#[test]
fn test_response_stream_event_stream() {
    let a = serde_json::json!({
        "type": "response.created",
        "response": {
            "id": "resp_67c9fdcecf488190bdd9a0409de3a1ec07b8b0ad4e5eb654",
            "object": "response",
            "created_at": 1741290958,
            "status": "in_progress",
            // "error": null,
            // "incomplete_details": null,
            "instructions": "You are a helpful assistant.",
            // "max_output_tokens": null,
            "model": "gpt-4.1-2025-04-14",
            "output": [],
            "parallel_tool_calls": true,
            // "previous_response_id": null,
            "reasoning": {
                // "effort": null,
                // "summary": null,
            },
            // "store": true,
            "temperature": 1.0,
            "text": {
                "format": {
                    "type": "text",
                },
            },
            "tool_choice": "auto",
            "tools": [],
            "top_p": 1.0,
            "truncation": "disabled",
            // "usage": null,
            // "user": null,
            "metadata": {},
        },
        "sequence_number": 1,
    });
    let b = crate::ResponseStreamEvent::ResponseCreated(
        crate::ResponseCreatedEvent::builder()
            .response(
                crate::Response::builder()
                    .id("resp_67c9fdcecf488190bdd9a0409de3a1ec07b8b0ad4e5eb654".to_owned())
                    .created_at(1741290958.into())
                    .status(Some(crate::ResponseStatus::InProgress))
                    .instructions(Some(crate::ResponseInstructions::String(
                        "You are a helpful assistant.".to_owned(),
                    )))
                    .model(crate::ModelIdsResponses::ModelIdsShared(
                        crate::ModelIdsShared::Gpt4_1_2025_04_14,
                    ))
                    .output(Vec::new())
                    .parallel_tool_calls(true)
                    .reasoning(Some(crate::Reasoning::default()))
                    .temperature(Some(serde_json::Number::from_f64(1.).unwrap()))
                    .text(Some(
                        crate::ResponsePropertiesText::builder()
                            .format(Some(crate::TextResponseFormatConfiguration::Text(
                                crate::ResponseFormatText::default(),
                            )))
                            .build(),
                    ))
                    .tool_choice(crate::ResponsePropertiesToolChoice::ToolChoiceOptions(
                        crate::ToolChoiceOptions::Auto,
                    ))
                    .tools(Vec::new())
                    .top_p(Some(serde_json::Number::from_f64(1.).unwrap()))
                    .truncation(Some(crate::ResponsePropertiesTruncation::Disabled))
                    .metadata(Some(crate::Metadata::new()))
                    .build(),
            )
            .sequence_number(1)
            .build(),
    );
    assert_eq!(a, serde_json::to_value(&b).unwrap());
    assert_eq!(b, serde_json::from_value(a.clone()).unwrap());
}
