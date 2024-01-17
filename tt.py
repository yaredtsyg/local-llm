import llama_cpp
my_aweseome_llama_model = llama_cpp.Llama(
    model_path="/Users/yared/Documents/phi-2/phi-2.Q2_K.gguf", n_gpu_layers=1)

prompt = "hi"
max_tokens = 100
temperature = 0.3
top_p = 0.1
echo = True
# stop = ["Q", "\n"]
text = ""
print("begin answering")
# Define the parameters
for output in my_aweseome_llama_model(
    prompt,
    max_tokens=0,
    top_k=96,
    top_p=0.9,
    stream=True,

):
    text = output['choices'][0]["text"]

    print(text, end="")
    # print(output)
# final_result = model_output["choices"][0]["text"].strip()
