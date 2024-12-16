# ragx

[![Actions Status](https://github.com/ai-flowx/ragx/workflows/ci/badge.svg?branch=main&event=push)](https://github.com/ai-flowx/ragx/actions?query=workflow%3Aci)
[![License](https://img.shields.io/github/license/ai-flowx/ragx.svg?color=brightgreen)](https://github.com/ai-flowx/ragx/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/ai-flowx/ragx.svg?color=brightgreen)](https://github.com/ai-flowx/ragx/tags)



## Introduction

*ragx* is the AI RAG of *[flowx](https://github.com/ai-flowx/)* written in Rust.



## Prerequisites

- Rust >= 1.83.0



## Run

```bash
./ragx --config-file="config.yml" --listen-url=":8080"
```



## Usage

```
Usage: ragx --config-file <NAME> --listen-url <URL>

Options:
  -c, --config-file <NAME>  Config file (.yml) [default: config.yml]
  -l, --listen-url <URL>    Listen url (host:port) [default: :8080]
  -h, --help                Print help
  -V, --version             Print version
```



## Settings

*ragx* parameters can be set in the directory [config](https://github.com/ai-flowx/ragx/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/ai-flowx/ragx/blob/main/src/config/config.yml):

```yaml
TBD
```



## Architecture



## License

Project License can be found [here](LICENSE).



## Reference
