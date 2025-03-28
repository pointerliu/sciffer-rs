# sciffer-rs

> sci sniffer witten in Rust 🦀.

`sciffer-rs` is a cutting-edge research trend sniffer written in Rust 🦀, designed to analyze and track emerging directions in academic fields.

## TODOs

- [x] add arxiv keys to TopicData
- [ ] analyze extracted keywords
- [ ] registry system
- [ ] save to database

## How to Use

### Setup Rust

To get started, you'll need to have Rust installed on your machine. Please follow the official installation guide here: [Rust Installation](https://www.rust-lang.org/tools/install).

If you’re using Linux, you can install Rust by running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Set Your API_KEY in the .env File

1. Obtain your LLMs API key from the service provider (e.g., OpenAI, Siliconflow or another data source you are using).
2. Create a `.env` file in the root directory of your project (if it doesn’t exist already).
3. Add your API key to the file in the following format:

```env
API_KEY=your_api_key_here
```

### Fetch Papers from Arxiv

Once your environment is set up and the API key is configured, you can start fetching papers from Arxiv. Use the following command to fetch papers based on a query:

```bash
cargo run --bin sciffer_cli -- --num 3 --query "machine learning"
```

- `--num 3`: Specifies the number of papers to retrieve (in this case, 3 papers).
- `--query "machine learning"`: Specifies the search query. Replace `"machine learning"` with any other search term you want.

This will fetch the latest results for the given query and display them in the terminal.

### Sciffer Server

```bash
cargo run --bin sciffer_server -- --num 3 --query "machine learning"
```