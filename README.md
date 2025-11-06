

# ChainForge 🦀

**Enterprise LangChain Implementation in Rust**

Production-grade AI orchestration framework built with Rust, featuring LLM providers, RAG pipelines, agent systems, and advanced memory management.

---

##  Features

###  LLM Pipeline
- **Multi-Provider Support**: OpenAI, Ollama, Hugging Face
- **Model Selection**: Dynamic model switching
- **Parameter Control**: Temperature, max tokens, top_p, stop sequences
- **Token Tracking**: Automatic token counting and cost estimation

###  Prompt Chains
- **Simple Chains**: Single-step prompt execution
- **Sequential Chains**: Multi-step reasoning with output chaining
- **RAG Pipeline**: Context-aware generation with retrieval
- **Chain Management**: Registry system for dynamic chain loading

###  Memory Systems
- **Session Memory**: Redis-based conversation history
- **Vector Memory**: Qdrant for long-term semantic storage
- **Embedding Search**: FastEmbed integration for similarity search
- **Context Injection**: Automatic context retrieval and injection

### RAG (Retrieval-Augmented Generation)
- **Document Loaders**: Support for Text, PDF, and web URLs
- **Smart Chunking**: Fixed-size and sentence-based chunking
- **Vector Indexing**: Automatic embedding and indexing
- **Relevancy Search**: Top-K retrieval with similarity thresholds

###  Agent System
- **Tool Framework**: Pluggable tool system
- **Built-in Tools**: Calculator, web search, code execution
- **Agent Executor**: Reasoning loop with tool selection
- **Multi-Turn Conversations**: Stateful agent interactions

###  Monitoring & Observability
- **Prometheus Metrics**: Request counts, latency, token usage
- **Structured Logging**: JSON and pretty-print formats
- **Cost Tracking**: Real-time API cost monitoring
- **Health Endpoints**: System status and metrics

---

##  Architecture

```
ChainForge/
├── src/
│   ├── config/           # Configuration management
│   ├── llm/              # LLM providers (OpenAI, Ollama, HF)
│   ├── embeddings/       # Embedding models (FastEmbed)
│   ├── memory/           # Session (Redis) + Vector (Qdrant)
│   ├── rag/              # Document loading, chunking, retrieval
│   ├── chains/           # Chain implementations
│   ├── agents/           # Agent executor and tools
│   ├── monitoring/       # Metrics and logging
│   ├── api/              # REST API handlers
│   └── main.rs           # Server entry point
├── config.yaml           # Runtime configuration
├── .env.example          # Environment template
└── Cargo.toml            # Dependencies
```

---

##  Installation

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Redis (for session memory)
- Qdrant (for vector storage)

### Quick Start

1. **Clone the repository**
```bash
cd ChainForge
```

2. **Setup environment**
```bash
copy .env.example .env
```

Edit `.env` and add your API keys:
```env
OPENAI_API_KEY=sk-your-key-here
HF_API_KEY=hf_your-token-here
RUST_LOG=info
```

3. **Start dependencies** (Docker recommended)
```bash
# Redis
docker run -d -p 6379:6379 redis:latest

# Qdrant
docker run -d -p 6333:6333 qdrant/qdrant:latest
```

4. **Build the project**
```bash
build.bat
```

5. **Run the server**
```bash
run.bat
```

Server starts on `http://localhost:8000`

---

##  API Reference

### Health & Status
```bash
# Health check
GET /health

# System status
GET /status
```

### LLM Operations
```bash
# Generate text
POST /llm/generate
{
  "prompt": "What is Rust?",
  "provider": "openai",
  "temperature": 0.7,
  "max_tokens": 2048
}

# List available providers
GET /llm/providers
```

### Chain Execution
```bash
# List all chains
GET /chains

# Execute a chain
POST /chains/qa/execute
{
  "variables": {
    "question": "What is machine learning?"
  }
}
```

### RAG Operations
```bash
# Index a document
POST /rag/index
{
  "content": "Your document text...",
  "source": "document.txt",
  "metadata": {}
}

# Query with RAG
POST /rag/query
{
  "query": "What is the main topic?",
  "top_k": 5
}
```

### Agent Execution
```bash
# Execute agent task
POST /agent/execute
{
  "task": "Calculate 15 * 23 and explain the result",
  "tools": ["calculator"]
}
```

### Memory Management
```bash
# Get session messages
GET /memory/session/{session_id}

# Add message to session
POST /memory/session/{session_id}
{
  "role": "user",
  "content": "Hello!"
}

# Clear session
POST /memory/session/{session_id}/clear
```

### Monitoring
```bash
# Prometheus metrics
GET /metrics
```

---

##  Configuration

### `config.yaml`

```yaml
server:
  host: "0.0.0.0"
  port: 8000

llm:
  default_provider: "openai"
  providers:
    openai:
      api_key_env: "OPENAI_API_KEY"
      default_model: "gpt-4-turbo"
      temperature: 0.7
      max_tokens: 2048

embeddings:
  provider: "fastembed"
  model: "BAAI/bge-small-en-v1.5"
  batch_size: 32

memory:
  redis:
    url: "redis://localhost:6379"
    ttl_seconds: 3600
  qdrant:
    url: "http://localhost:6333"
    collection_name: "chainforge_memory"
    vector_size: 384

rag:
  chunk_size: 512
  chunk_overlap: 50
  retrieval_top_k: 5
  similarity_threshold: 0.7

monitoring:
  enable_metrics: true
  metrics_port: 9090
  log_level: "info"
  log_format: "json"
```

---

## 🔧 Development

### Run Tests
```bash
cargo test
```

### Build for Production
```bash
cargo build --release
```

### Run with Debug Logging
```bash
set RUST_LOG=debug
cargo run
```

---

##  Usage Examples

### Simple Chain Example
```rust
use chainforge::chains::simple::SimpleChain;

let chain = SimpleChain::new(
    "qa_chain",
    "Question answering",
    llm_provider,
    "Answer: {question}"
);

let input = ChainInput::new()
    .with_variable("question", "What is Rust?");

let output = chain.execute(input).await?;
```

### RAG Pipeline Example
```rust
use chainforge::rag::retriever::Retriever;

// Index a document
let document = Document::new(content, "source.txt");
retriever.index_document(&document).await?;

// Query with context
let context = retriever.build_context("query").await?;
```

### Agent Execution Example
```rust
use chainforge::agents::executor::AgentExecutor;

let mut agent = AgentExecutor::new(llm, 10);
agent.add_tool(Arc::new(CalculatorTool));

let result = agent.execute("Calculate 25 * 4").await?;
```

---

## 🎯 Roadmap

- [ ] LangGraph visual pipeline editor
- [ ] Plugin system with hot-reload
- [ ] GraphQL API support
- [ ] Multi-model ensemble support
- [ ] Advanced caching strategies
- [ ] Streaming responses
- [ ] WebSocket support
- [ ] Dashboard UI

---

##  Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## License

MIT License - feel free to use this project for any purpose.

---

## 🔗 Tech Stack

- **Language**: Rust 2021
- **Async Runtime**: Tokio
- **Web Framework**: Axum
- **LLM Integration**: async-openai, reqwest
- **Embeddings**: FastEmbed
- **Vector DB**: Qdrant
- **Session Store**: Redis
- **Metrics**: Prometheus
- **Logging**: tracing, tracing-subscriber

---

## Tips

### Performance Tuning
- Adjust `chunk_size` for optimal retrieval
- Use batch embedding for large documents
- Enable Redis persistence for session recovery
- Configure Qdrant collection optimization

### Cost Optimization
- Monitor token usage via `/metrics`
- Set appropriate `max_tokens` limits
- Use cheaper models for simple tasks
- Cache frequently used responses

### Security
- Never commit `.env` file
- Use environment variables for secrets
- Enable API authentication (add custom middleware)
- Rate limit endpoints in production

---

##  Troubleshooting

**Error: Redis connection failed**
```bash
# Start Redis
docker run -d -p 6379:6379 redis:latest
```

**Error: Qdrant not available**
```bash
# Start Qdrant
docker run -d -p 6333:6333 qdrant/qdrant:latest
```

**Error: OpenAI API key not found**
```bash
# Check .env file
echo %OPENAI_API_KEY%
```

---

**Built with ❤️ and 🦀 Rust**

*Where AI meets Rust performance* 🔥
