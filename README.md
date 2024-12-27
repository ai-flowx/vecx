# vecx

[![Actions Status](https://github.com/ai-flowx/vecx/workflows/ci/badge.svg?branch=main&event=push)](https://github.com/ai-flowx/vecx/actions?query=workflow%3Aci)
[![License](https://img.shields.io/github/license/ai-flowx/vecx.svg?color=brightgreen)](https://github.com/ai-flowx/vecx/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/ai-flowx/vecx.svg?color=brightgreen)](https://github.com/ai-flowx/vecx/tags)



## Introduction

*vecx* is the ai vector store of [flowx](https://github.com/ai-flowx/) written in Rust.



## Prerequisites

- Rust >= 1.83.0



## Run

```bash
```



## Usage

```
```



## Settings

*vecx* parameters can be set in the directory [config](https://github.com/ai-flowx/vecx/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/ai-flowx/vecx/blob/main/src/config/config.yml):

```yaml
```



## Architecture

![arch](./arch.png "Architecture")



## Features

- **Compatibility**
  - vikingdb


- **Data**
  - multimodal data


- **Deployment**
  - cloud-side
  - device-side


- **Indexing**
  - millisecond-level retrieval
  - second-level index update
  - real-time data writing
  - real-time indexing
  - automatic indexing


- **Model**
    - inline vector model


- **Storage**
    - graph storage



## License

Project License can be found [here](LICENSE).



## Reference

### Framework

- [letta](https://github.com/letta-ai/letta)
- [sandboxfusion](https://github.com/bytedance/SandboxFusion)

### LangChain

- [api](https://github.com/langchain-ai/langchain/blob/master/libs/core/langchain_core/vectorstores/base.py)
- [vikingdb](https://github.com/langchain-ai/langchain/blob/master/libs/community/langchain_community/vectorstores/vikingdb.py)

### Memory

- [long-term-memory](https://langchain-ai.github.io/long-term-memory/)
- [mem0](https://github.com/mem0ai/mem0)
- [memgraph](https://github.com/memgraph/memgraph)
- [zep](https://www.getzep.com/)

### RAG

- [aliyun-graphrag](https://developer.aliyun.com/article/1540097)
- [codegraphrag](https://mp.weixin.qq.com/s/hYJsWDkh6GnM1xqpg4gnXw)
- [tree-sitter-graph](https://github.com/tree-sitter/tree-sitter-graph)
- [vanna](https://github.com/vanna-ai/vanna)

### Toolchains

- [rustup-android](https://github.com/rust-lang/rustup/issues/2872)

### Vector

- [benchmark](https://github.com/zilliztech/VectorDBBench)
- [qdrant](https://github.com/qdrant/qdrant)
- [similarity-search](https://github.com/facebookresearch/faiss)
- [vikingdb](https://www.volcengine.com/docs/82379/1263276)
