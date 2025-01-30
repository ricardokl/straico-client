# User information
Method: GET
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"}]
Url: .https://api.straico.com/v0/user

## Snippet
null

## Response
{
    "data": {
        "first_name": "Jane",
        "last_name": "Doe",
        "coins": 562621.19,
        "plan": "Ultimate Pack"
    },
    "success": true
}
# Models information (v.0)
Method: GET
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"}]
Url: .https://api.straico.com/v0/models

## Snippet
null

## Response
{
    "data": [
        {
            "name": "Anthropic: Claude 3 Haiku",
            "model": "anthropic/claude-3-haiku:beta",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "Anthropic: Claude 3 Opus",
            "model": "anthropic/claude-3-opus",
            "pricing": {
                "coins": 24,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "Anthropic: Claude 3 Sonnet",
            "model": "anthropic/claude-3-sonnet",
            "pricing": {
                "coins": 5,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "Anthropic: Claude 3.5 Sonnet",
            "model": "anthropic/claude-3.5-sonnet",
            "pricing": {
                "coins": 5,
                "words": 100
            },
            "max_output": 8192
        },
        {
            "name": "Cohere: Command R (08-2024)",
            "model": "cohere/command-r-08-2024",
            "pricing": {
                "coins": 0.2,
                "words": 100
            },
            "max_output": 4000
        },
        {
            "name": "Cohere: Command R+ (08-2024)",
            "model": "cohere/command-r-plus-08-2024",
            "pricing": {
                "coins": 3.4,
                "words": 100
            },
            "max_output": 4000
        },
        {
            "name": "Dolphin 2.6 Mixtral 8x7B",
            "model": "cognitivecomputations/dolphin-mixtral-8x7b",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Goliath 120B",
            "model": "alpindale/goliath-120b",
            "pricing": {
                "coins": 5,
                "words": 100
            },
            "max_output": 400
        },
        {
            "name": "Google: Gemini Pro 1.5",
            "model": "google/gemini-pro-1.5",
            "pricing": {
                "coins": 3.7,
                "words": 100
            },
            "max_output": 8192
        },
        {
            "name": "Google: Gemma 2 27B",
            "model": "google/gemma-2-27b-it",
            "pricing": {
                "coins": 0.4,
                "words": 100
            },
            "max_output": 8192
        },
        {
            "name": "Gryphe: MythoMax L2 13B 8k",
            "model": "gryphe/mythomax-l2-13b",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "Meta: Llama 3 70B Instruct (nitro)",
            "model": "meta-llama/llama-3-70b-instruct:nitro",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 8192
        },
        {
            "name": "Meta: Llama 3 8B Instruct",
            "model": "meta-llama/llama-3-8b-instruct",
            "pricing": {
                "coins": 0.5,
                "words": 100
            },
            "max_output": 8192
        },
        {
            "name": "Meta: Llama 3.1 405B Instruct",
            "model": "meta-llama/llama-3.1-405b-instruct",
            "pricing": {
                "coins": 1.6,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Meta: Llama 3.1 70B Instruct",
            "model": "meta-llama/llama-3.1-70b-instruct",
            "pricing": {
                "coins": 0.7,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Mistral: Codestral Mamba",
            "model": "mistralai/codestral-mamba",
            "pricing": {
                "coins": 0.2,
                "words": 100
            },
            "max_output": 256000
        },
        {
            "name": "Mistral: Large",
            "model": "mistralai/mistral-large",
            "pricing": {
                "coins": 3,
                "words": 100
            },
            "max_output": 128000
        },
        {
            "name": "Mistral: Mixtral 8x7B",
            "model": "mistralai/mixtral-8x7b-instruct",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Nous: Hermes 3 405B Instruct",
            "model": "nousresearch/hermes-3-llama-3.1-405b",
            "pricing": {
                "coins": 0.3,
                "words": 100
            },
            "max_output": 18000
        },
        {
            "name": "OpenAI: GPT-3.5 Turbo 16k",
            "model": "openai/gpt-3.5-turbo-0125",
            "pricing": {
                "coins": 1,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "OpenAI: GPT-4",
            "model": "openai/gpt-4",
            "pricing": {
                "coins": 20,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "OpenAI: GPT-4 Turbo 128k",
            "model": "openai/gpt-4-turbo-2024-04-09",
            "pricing": {
                "coins": 8,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "OpenAI: GPT-4 Vision",
            "model": "openai/gpt-4-vision-preview",
            "pricing": {
                "coins": 10,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "OpenAI: GPT-4o - New (Aug-06)",
            "model": "openai/gpt-4o-2024-08-06",
            "pricing": {
                "coins": 3,
                "words": 100
            },
            "max_output": 16384
        },
        {
            "name": "OpenAI: GPT-4o - Old",
            "model": "openai/gpt-4o",
            "pricing": {
                "coins": 4,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "OpenAI: GPT-4o mini",
            "model": "openai/gpt-4o-mini",
            "pricing": {
                "coins": 0.4,
                "words": 100
            },
            "max_output": 16384
        },
        {
            "name": "OpenAI: o1-mini (Beta)",
            "model": "openai/o1-mini",
            "pricing": {
                "coins": 4,
                "words": 100
            },
            "max_output": 65536
        },
        {
            "name": "OpenAI: o1-preview (Beta)",
            "model": "openai/o1-preview",
            "pricing": {
                "coins": 20,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Perplexity: Llama 3.1 Sonar 405B Online",
            "model": "perplexity/llama-3.1-sonar-huge-128k-online",
            "pricing": {
                "coins": 2.7,
                "words": 100
            },
            "max_output": 127072
        },
        {
            "name": "Perplexity: Llama 3.1 Sonar 70B Online",
            "model": "perplexity/llama-3.1-sonar-large-128k-online",
            "pricing": {
                "coins": 0.6,
                "words": 100
            },
            "max_output": 127072
        },
        {
            "name": "Perplexity: Llama 3.1 Sonar 8B Online",
            "model": "perplexity/llama-3.1-sonar-small-128k-online",
            "pricing": {
                "coins": 0.2,
                "words": 100
            },
            "max_output": 127072
        },
        {
            "name": "Qwen 2 72B Instruct",
            "model": "qwen/qwen-2-72b-instruct",
            "pricing": {
                "coins": 0.5,
                "words": 100
            },
            "max_output": 32768
        },
        {
            "name": "Qwen2-VL 72B Instruct",
            "model": "qwen/qwen-2-vl-72b-instruct",
            "pricing": {
                "coins": 0.2,
                "words": 100
            },
            "max_output": 4096
        },
        {
            "name": "Qwen2.5 72B Instruct",
            "model": "qwen/qwen-2.5-72b-instruct",
            "pricing": {
                "coins": 0.2,
                "words": 100
            },
            "max_output": 32768
        }
    ],
    "success": true
}
# Models information (v.1)
Method: GET
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"}]
Url: .https://api.straico.com/v1/models

## Snippet
null

## Response
{
    "data": {
        "chat": [
            {
                "name": "Anthropic: Claude 3 Haiku",
                "model": "anthropic/claude-3-haiku:beta",
                "word_limit": 150000,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "Anthropic: Claude 3 Opus",
                "model": "anthropic/claude-3-opus",
                "word_limit": 150000,
                "pricing": {
                    "coins": 24,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "Anthropic: Claude 3 Sonnet",
                "model": "anthropic/claude-3-sonnet",
                "word_limit": 150000,
                "pricing": {
                    "coins": 5,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "Anthropic: Claude 3.5 Sonnet",
                "model": "anthropic/claude-3.5-sonnet",
                "word_limit": 150000,
                "pricing": {
                    "coins": 5,
                    "words": 100
                },
                "max_output": 8192
            },
            {
                "name": "Cohere: Command R (08-2024)",
                "model": "cohere/command-r-08-2024",
                "word_limit": 96000,
                "pricing": {
                    "coins": 0.2,
                    "words": 100
                },
                "max_output": 4000
            },
            {
                "name": "Cohere: Command R+ (08-2024)",
                "model": "cohere/command-r-plus-08-2024",
                "word_limit": 96000,
                "pricing": {
                    "coins": 3.4,
                    "words": 100
                },
                "max_output": 4000
            },
            {
                "name": "Dolphin 2.6 Mixtral 8x7B",
                "model": "cognitivecomputations/dolphin-mixtral-8x7b",
                "word_limit": 24000,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Goliath 120B",
                "model": "alpindale/goliath-120b",
                "word_limit": 4608,
                "pricing": {
                    "coins": 5,
                    "words": 100
                },
                "max_output": 400
            },
            {
                "name": "Google: Gemini Pro 1.5",
                "model": "google/gemini-pro-1.5",
                "word_limit": 1500000,
                "pricing": {
                    "coins": 3.7,
                    "words": 100
                },
                "max_output": 8192
            },
            {
                "name": "Google: Gemma 2 27B",
                "model": "google/gemma-2-27b-it",
                "word_limit": 3072,
                "pricing": {
                    "coins": 0.4,
                    "words": 100
                },
                "max_output": 8192
            },
            {
                "name": "Gryphe: MythoMax L2 13B 8k",
                "model": "gryphe/mythomax-l2-13b",
                "word_limit": 6144,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "Meta: Llama 3 70B Instruct (nitro)",
                "model": "meta-llama/llama-3-70b-instruct:nitro",
                "word_limit": 6144,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 8192
            },
            {
                "name": "Meta: Llama 3 8B Instruct",
                "model": "meta-llama/llama-3-8b-instruct",
                "word_limit": 6144,
                "pricing": {
                    "coins": 0.5,
                    "words": 100
                },
                "max_output": 8192
            },
            {
                "name": "Meta: Llama 3.1 405B Instruct",
                "model": "meta-llama/llama-3.1-405b-instruct",
                "word_limit": 98000,
                "pricing": {
                    "coins": 1.6,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Meta: Llama 3.1 70B Instruct",
                "model": "meta-llama/llama-3.1-70b-instruct",
                "word_limit": 98000,
                "pricing": {
                    "coins": 0.7,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Mistral: Codestral Mamba",
                "model": "mistralai/codestral-mamba",
                "word_limit": 192000,
                "pricing": {
                    "coins": 0.2,
                    "words": 100
                },
                "max_output": 256000
            },
            {
                "name": "Mistral: Large",
                "model": "mistralai/mistral-large",
                "word_limit": 96000,
                "pricing": {
                    "coins": 3,
                    "words": 100
                },
                "max_output": 128000
            },
            {
                "name": "Mistral: Mixtral 8x7B",
                "model": "mistralai/mixtral-8x7b-instruct",
                "word_limit": 24576,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Nous: Hermes 3 405B Instruct",
                "model": "nousresearch/hermes-3-llama-3.1-405b",
                "word_limit": 13500,
                "pricing": {
                    "coins": 0.3,
                    "words": 100
                },
                "max_output": 18000
            },
            {
                "name": "OpenAI: GPT-3.5 Turbo 16k",
                "model": "openai/gpt-3.5-turbo-0125",
                "word_limit": 12000,
                "pricing": {
                    "coins": 1,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "OpenAI: GPT-4",
                "model": "openai/gpt-4",
                "word_limit": 6000,
                "pricing": {
                    "coins": 20,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "OpenAI: GPT-4 Turbo 128k",
                "model": "openai/gpt-4-turbo-2024-04-09",
                "word_limit": 96000,
                "pricing": {
                    "coins": 8,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "OpenAI: GPT-4 Vision",
                "model": "openai/gpt-4-vision-preview",
                "word_limit": 75000,
                "pricing": {
                    "coins": 10,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "OpenAI: GPT-4o - New (Aug-06)",
                "model": "openai/gpt-4o-2024-08-06",
                "word_limit": 96000,
                "pricing": {
                    "coins": 3,
                    "words": 100
                },
                "max_output": 16384
            },
            {
                "name": "OpenAI: GPT-4o - Old",
                "model": "openai/gpt-4o",
                "word_limit": 96000,
                "pricing": {
                    "coins": 4,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "OpenAI: GPT-4o mini",
                "model": "openai/gpt-4o-mini",
                "word_limit": 96000,
                "pricing": {
                    "coins": 0.4,
                    "words": 100
                },
                "max_output": 16384
            },
            {
                "name": "OpenAI: o1-mini (Beta)",
                "model": "openai/o1-mini",
                "word_limit": 96000,
                "pricing": {
                    "coins": 4,
                    "words": 100
                },
                "max_output": 65536
            },
            {
                "name": "OpenAI: o1-preview (Beta)",
                "model": "openai/o1-preview",
                "word_limit": 96000,
                "pricing": {
                    "coins": 20,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Perplexity: Llama 3.1 Sonar 405B Online",
                "model": "perplexity/llama-3.1-sonar-huge-128k-online",
                "word_limit": 95304,
                "pricing": {
                    "coins": 2.7,
                    "words": 100
                },
                "max_output": 127072
            },
            {
                "name": "Perplexity: Llama 3.1 Sonar 70B Online",
                "model": "perplexity/llama-3.1-sonar-large-128k-online",
                "word_limit": 95000,
                "pricing": {
                    "coins": 0.6,
                    "words": 100
                },
                "max_output": 127072
            },
            {
                "name": "Perplexity: Llama 3.1 Sonar 8B Online",
                "model": "perplexity/llama-3.1-sonar-small-128k-online",
                "word_limit": 95000,
                "pricing": {
                    "coins": 0.2,
                    "words": 100
                },
                "max_output": 127072
            },
            {
                "name": "Qwen 2 72B Instruct",
                "model": "qwen/qwen-2-72b-instruct",
                "word_limit": 24576,
                "pricing": {
                    "coins": 0.5,
                    "words": 100
                },
                "max_output": 32768
            },
            {
                "name": "Qwen2-VL 72B Instruct",
                "model": "qwen/qwen-2-vl-72b-instruct",
                "word_limit": 96000,
                "pricing": {
                    "coins": 0.2,
                    "words": 100
                },
                "max_output": 4096
            },
            {
                "name": "Qwen2.5 72B Instruct",
                "model": "qwen/qwen-2.5-72b-instruct",
                "word_limit": 98304,
                "pricing": {
                    "coins": 0.2,
                    "words": 100
                },
                "max_output": 32768
            }
        ],
        "image": [
            {
                "name": "OpenAI: Dall-E 3",
                "model": "openai/dall-e-3",
                "pricing": {
                    "square": {
                        "coins": 90,
                        "size": "1024x1024"
                    },
                    "landscape": {
                        "coins": 120,
                        "size": "1792x1024"
                    },
                    "portrait": {
                        "coins": 120,
                        "size": "1024x1792"
                    }
                }
            }
        ]
    },
    "success": true
}
# Prompt completion (v.0)
Method: POST
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"},{"key":"Content-Type","value":"application/json","type":"text"}]
Url: .https://api.straico.com/v0/prompt/completion

## Snippet
null

## Response
{
    "data": {
        "completion": {
            "id": "gen-kb0IF9xqPuSKLCUWThBDAHEOD6ms",
            "model": "anthropic/claude-3-haiku:beta",
            "object": "chat.completion",
            "created": 1721400470,
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "# 5 Practical Tips for Recycling at Home\n\nRecycling has become an essential part of our efforts to create a more sustainable future. As individuals, we can make a significant impact by adopting simple recycling practices in our homes. Here are five practical tips to help you recycle more effectively at home:\n\n1. **Separate and Sort**: Establish a dedicated recycling station in your home, with clearly labeled containers for different types of recyclables, such as paper, plastic, glass, and metal. Sorting your waste makes it easier for recycling facilities to process and repurpose the materials.\n\n2. **Reduce Waste**: Before reaching for new items, consider whether you can reuse or repurpose existing ones. This helps minimize the amount of material that ends up in landfills or requires recycling. Look for opportunities to buy in bulk, opt for products with minimal packaging, and choose reusable alternatives whenever possible.\n\n3. **Stay Informed**: Keep up-to-date with your local recycling guidelines and regulations. Different regions may have specific requirements for what can and cannot be accepted. Check with your municipal or county recycling program to ensure you are recycling correctly and avoiding contamination.\n\n4. **Rinse and Clean**: Ensure that your recyclables are clean and free of food residue. This helps prevent contamination and ensures that the materials can be properly processed. Rinse out containers before placing them in the recycling bin.\n\n5. **Educate and Involve**: Encourage your family members, especially children, to participate in the recycling process. Educate them on the importance of recycling and the impact it can have on the environment. Make it a shared responsibility and involve everyone in the household to develop a culture of sustainable living.\n\nBy implementing these five practical tips, you can make a meaningful contribution to the recycling effort and help create a cleaner, more sustainable world for future generations."
                    },
                    "finish_reason": "end_turn"
                }
            ],
            "usage": {
                "prompt_tokens": 25,
                "completion_tokens": 416,
                "total_tokens": 441
            }
        },
        "price": {
            "input": 0.15,
            "output": 3.03,
            "total": 3.18
        },
        "words": {
            "input": 15,
            "output": 303,
            "total": 318
        }
    },
    "success": true
}
# Prompt completion (v.1)
Method: POST
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"},{"key":"Content-Type","value":"application/json","type":"text"}]
Url: .https://api.straico.com/v1/prompt/completion

## Snippet
null

## Response
{
    "data": {
        "overall_price": {
            "input": 114.36,
            "output": 1.25,
            "total": 115.61
        },
        "overall_words": {
            "input": 11436,
            "output": 125,
            "total": 11561
        },
        "completions": {
            "anthropic/claude-3-haiku:beta": {
                "completion": {
                    "id": "gen-WmC4MvpDgJwAClluDeFYf7apGGIT",
                    "model": "anthropic/claude-3-haiku:beta",
                    "object": "chat.completion",
                    "created": 1721329567,
                    "choices": [
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Based on the transcript of the YouTube video \"Why does the universe exist? | Jim Holt | TED\", the following key terms from the 100-word \"space words\" document also appear in the video:\n\n- universe\n- cosmos\n- black hole\n- big bang\n- planet\n- gravity\n- star\n- galaxy\n- space\n- light year\n- meteor\n- eclipse\n- solar system\n- asteroid\n\nThe video discusses the origins and nature of the universe, covering topics like the big bang, black holes, and cosmic phenomena, which overlap with the space-related vocabulary from the \"space words\" document."
                            },
                            "finish_reason": "end_turn"
                        }
                    ],
                    "usage": {
                        "prompt_tokens": 11122,
                        "completion_tokens": 136,
                        "total_tokens": 11258
                    }
                },
                "price": {
                    "input": 57.18,
                    "output": 0.97,
                    "total": 58.15
                },
                "words": {
                    "input": 5718,
                    "output": 97,
                    "total": 5815
                }
            },
            "openai/gpt-3.5-turbo-0125": {
                "completion": {
                    "id": "chatcmpl-9mQk5Ei3vEiGbzwZkTc55nUt35sng",
                    "object": "chat.completion",
                    "created": 1721329569,
                    "model": "gpt-3.5-turbo-0125",
                    "choices": [
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Key terms from the space_words.csv document that also appear in the YouTube video transcript about the universe are: universe, existence, reality, world, mystery, question, physics, God, and cosmic."
                            },
                            "logprobs": null,
                            "finish_reason": "stop"
                        }
                    ],
                    "usage": {
                        "prompt_tokens": 10364,
                        "completion_tokens": 40,
                        "total_tokens": 10404
                    },
                    "system_fingerprint": null
                },
                "price": {
                    "input": 57.18,
                    "output": 0.28,
                    "total": 57.46
                },
                "words": {
                    "input": 5718,
                    "output": 28,
                    "total": 5746
                }
            }
        }
    },
    "success": true
}
# File upload
Method: POST
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"},{"key":"Content-Type","value":"multipart/form-data","type":"text"}]
Url: .https://api.straico.com/v0/file/upload

## Snippet
null

## Response
{
    "data": {
        "url": "https://prompt-rack.s3.amazonaws.com/api/1721329178731_space_words.csv"
    },
    "success": true
}
# Image generation
Method: POST
Header: [{"key":"Authorization","value":"Bearer $STRAICO_API_KEY","type":"text"},{"key":"Content-Type","value":"application/json","type":"text"}]
Url: .https://api.straico.com/v0/image/generation

## Snippet
null

## Response
{
    "data": {
        "zip": "https://prompt-rack.s3.amazonaws.com/api/1721333310153_e8gn2Z4K.zip",
        "images": [
            "https://prompt-rack.s3.amazonaws.com/api/1721333307376_bSyyTpYn.png",
            "https://prompt-rack.s3.amazonaws.com/api/1721333308709_9kVx2vm9.png"
        ],
        "price": {
            "price_per_image": 120,
            "quantity_images": 2,
            "total": 240
        }
    },
    "success": true
}
# Create Agent
Method: POST
Header: []
Url: .https://api.straico.com/v0/agent

## Snippet
null

## Response

# Add RAG to Agent
Method: POST
Header: []
Url: .https://stapi.straico.com/v0/agent/<agent-id>/rag

## Snippet
null

## Response

# Agent details
Method: GET
Header: []
Url: .https://api.straico.com/v0/agent/<agent-id>

## Snippet
null

## Response

# List of agents
Method: GET
Header: []
Url: .https://api.straico.com/v0/agent/

## Snippet
null

## Response

# Update agent
Method: PUT
Header: []
Url: .https://api.straico.com/v0/agent/<agent-id>

## Snippet
null

## Response

# Agent prompt completion
Method: POST
Header: []
Url: .https://api.straico.com/v0/agent/<agent-id>/prompt

## Snippet
null

## Response

# Delete agent
Method: DELETE
Header: []
Url: .https://api.straico.com/v0/agent/<agent-id>

## Snippet
null

## Response

# Create RAG
Method: POST
Header: []
Url: .https://api.straico.com/v0/rag

## Snippet
null

## Response

# List of RAG's
Method: GET
Header: []
Url: .https://api.straico.com/v0/rag/user

## Snippet
null

## Response

# RAG by ID
Method: GET
Header: []
Url: .https://api.straico.com/v0/rag/<rag-id>

## Snippet
null

## Response

# Delete RAG
Method: DELETE
Header: []
Url: .https://api.straico.com/v0/rag/<rag-id>

## Snippet
null

## Response

# RAG prompt completion
Method: POST
Header: []
Url: .https://api.straico.com/v0/rag/<rag-id>/prompt

## Snippet
null

## Response

