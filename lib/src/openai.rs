use serde::Deserialize;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct Client {
    api_url: String,
    api_key: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_url: API_URL.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn query(&self, body: String) -> Result<String, anyhow::Error> {
        print!("Executing query: {}... ", &body);

        let body = serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role":"user", "content": body},
            ],
            "max_tokens": 500,
        });

        let response = reqwest::Client::new()
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("OpenAI-Organization", "org-SddYC41ywfVwSNFBNjkOyEZ6")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            println!("Failed to query OpenAI: {:?}", response);
            return Err(anyhow::anyhow!("Failed to query OpenAI: {:?}", response));
        }

        let response = response.json::<OpenAIResponse>().await?;
        let answer = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

        println!("{}", &answer);

        Ok(answer)
    }
}
