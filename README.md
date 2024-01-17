## How to run Local-llm

add the model path from you local pc. if your are using ollama you can get the downloaded path from `ollama show --modelfile {model_name}`
```
   let llama = LLama::new(
        "{path to model in your pc}".into(),
        &model_options,
    )
    .unwrap();
```
To change the prompt change the text in predict
```
    llama
        .predict(
            "what are the national animals of india".into(),
            predict_options,
        )
        .unwrap();
```

to run just type `cargo run`
