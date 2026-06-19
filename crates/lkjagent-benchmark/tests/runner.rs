use lkjagent_benchmark::runner::endpoint_config;

#[test]
fn endpoint_config_fails_without_model_and_url() {
    let result = endpoint_config(|_| None, None);

    assert_eq!(
        result,
        Err(
            "endpoint configuration missing: set LKJAGENT_MODEL and LKJAGENT_ENDPOINT_URL"
                .to_string()
        )
    );
}

#[test]
fn endpoint_config_reads_dotenv_and_redacts_to_host() {
    let dotenv = "LKJAGENT_MODEL=local\nLKJAGENT_ENDPOINT_URL=http://user:pass@example:8080/v1\n";
    let config = endpoint_config(|_| None, Some(dotenv));

    assert_eq!(
        config,
        Ok(lkjagent_benchmark::runner::EndpointConfig {
            model_label: "local".to_string(),
            endpoint_host: "example:8080".to_string(),
        })
    );
}
