# TLDIR - The Large Directory Is Readable

TLDIR is a tool that generates summaries and embeddings for directories (and file enumerations too) on the CLI.

## WIP requirements

**User Stories and Acceptance Criteria for `tldir` Command-Line Tool**

---

### User Story 1: Scanning and Summarizing Directory Contents

**As a** user  
**I want to** scan a directory for UTF-8 encoded files, chunk and summarize their contents recursively  
**So that** I can get a concise summary and embeddings of the directory's content stored locally  

#### Acceptance Criteria:

- **Invocation**: When I run `tldir <dirname>`, the tool starts processing the specified directory.
  
- **File Scanning**:
  - The tool scans all UTF-8 encoded files within `<dirname>` recursively.
  - It ignores non-UTF-8 files and hidden directories/files unless explicitly specified.

- **Chunking**:
  - Files are chunked using **llm-chain-rs** to handle large files efficiently.
  - Chunk sizes are optimized for the summarization model's input requirements.

- **Summarization**:
  - Each chunk is summarized individually using **mistral-rs** and **phi-3.5-moe** models.
  - Summaries are aggregated to create a top-level summary of the directory.
  - The final summary does not exceed **8192 tokens** as specified by `SUMMARY_LENGTH`.

- **Embedding Generation**:
  - Embeddings for each chunk are generated using **fastembed-rs**.
  - Embeddings are stored locally for efficient retrieval.

- **Data Storage**:
  - All summaries and embeddings are stored in a `.tldir` folder within `<dirname>`.
  - Embeddings are saved using **SQLite** via **Chroma** for efficient querying.

- **Performance**:
  - The tool efficiently utilizes system resources and completes processing in a reasonable time for directories of average size.

- **Feedback**:
  - Progress is displayed in the console, showing which files are being processed.
  - Any errors or skipped files are reported to the user.

---

### User Story 2: Retrieving Information via Questions

**As a** user  
**I want to** ask questions about the directory's content using `tldir ask <dirname>`  
**So that** I can retrieve relevant information quickly without manually searching through files  

#### Acceptance Criteria:

- **Invocation**: When I run `tldir ask <dirname>`, the tool enters an interactive mode or accepts a question as an argument.

- **Question Handling**:
  - The tool accepts natural language questions from the user.
  - Supports both interactive mode and command-line arguments for questions.

- **Retrieval**:
  - Utilizes embeddings stored in `.tldir` to find relevant chunks.
  - Uses **fastembed-rs** for embedding the question and **Chroma/SQLite** for efficient retrieval.

- **Response Generation**:
  - Generates answers by summarizing relevant chunks using **mistral-rs** and **phi-3.5-moe**.
  - Ensures responses are concise and directly address the user's question.

- **Accuracy**:
  - Retrieves the most relevant information based on the embeddings and summaries.
  - Filters out unrelated content to provide accurate answers.

- **User Experience**:
  - Provides clear prompts and instructions in interactive mode.
  - Handles invalid inputs gracefully, prompting the user to rephrase if necessary.

---

### User Story 3: Customizing Tool Behavior

**As a** user  
**I want to** customize parameters like summary length and inclusion of hidden files  
**So that** I can control the output according to my needs  

#### Acceptance Criteria:

- **Command-Line Options**:
  - `--summary-length <tokens>`: Set a custom summary length (default is 8192 tokens).
  - `--include-hidden`: Include hidden files and directories in the scanning process.
  - `--help`: Display help information about the tool's usage.

- **Validation**:
  - The tool validates input parameters and provides meaningful error messages for invalid inputs.
  - Default values are used when optional parameters are not specified.

- **Persistence**:
  - Custom parameters can be stored in a configuration file within `.tldir` for repeated use.

---

**CLI Arguments and `tldir --help` Output**

---

```plaintext
Usage: tldir [OPTIONS] <COMMAND> <DIRECTORY>

Commands:
  scan      Scan and summarize the directory
  ask       Ask questions about the directory's content
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --summary-length <TOKENS>    Set the summary length in tokens (default: 8192)
  -i, --include-hidden             Include hidden files and directories
  -h, --help                       Print help information
  -v, --version                    Print version information

Examples:
  tldir scan /path/to/directory
  tldir scan -s 5000 /path/to/directory
  tldir ask /path/to/directory
  tldir ask "/path/to/directory" "What are the key functions in the code?"

Description:
  tldir is a command-line tool that scans a directory for UTF-8 encoded files,
  chunks and summarizes their content recursively, and allows you to ask questions
  about the content using locally stored embeddings.

Subcommands:
  scan      Scan and summarize the directory
  ask       Ask questions about the directory's content
  help      Print this message or the help of the given subcommand(s)
```

---

**Contents of the `.tldir` Folder**

---

The `.tldir` folder within the specified directory contains the following:

1. **Embeddings Database**:
   - A SQLite database (via **Chroma**) named `embeddings.db`.
   - Stores embeddings for each chunk generated using **fastembed-rs**.
   - Schema includes tables for chunks, embeddings, and metadata.

2. **Summaries**:
   - A top-level summary file named `summary.txt`.
     - Contains the aggregated summary of the entire directory.
     - Summary length is capped at the specified `SUMMARY_LENGTH` (default 8192 tokens).
   - Individual chunk summaries stored in a subfolder `chunks/` if needed.

3. **Configuration File**:
   - `config.json` or similar, storing user preferences like custom summary length.

4. **Logs** (optional):
   - A `logs/` directory containing processing logs for debugging purposes.

5. **Metadata**:
   - Files containing metadata about the processed files, timestamps, etc.

---

**Implementation Details**

---

- **Chunking**:
  - Implemented using **llm-chain-rs**, which efficiently handles large text data by chaining language model operations.
  - Optimizes processing by breaking down files into manageable chunks.

- **Summarization**:
  - Uses **mistral-rs** and **phi-3.5-moe** models for generating high-quality summaries.
  - Models are integrated to balance performance and accuracy.

- **Embeddings**:
  - **fastembed-rs** generates embeddings quickly, suitable for large datasets.
  - Embeddings are essential for semantic search and retrieval during the `ask` command.

- **Retrieval**:
  - **Chroma** is used with SQLite to enable efficient vector searches within embeddings.
  - Ensures quick retrieval of relevant chunks in response to user queries.

- **Inference and Summarization Models**:
  - **mistral-rs**: A Rust implementation for inference tasks, suitable for summarization.
  - **phi-3.5-moe**: Mixture-of-experts model enhancing the summarization quality.

---

**Note on Dependencies**

To ensure the tool runs smoothly, the following dependencies are required:

- Rust environment for running the tool and associated libraries.
- **llm-chain-rs**, **fastembed-rs**, **mistral-rs**, and **phi-3.5-moe** installed and properly configured.
- SQLite installed for database operations.

---

**Examples of Use**

---

1. **Scanning a Directory with Default Settings**:

   ```bash
   tldir scan /path/to/directory
   ```

   - Scans the directory and creates summaries and embeddings with default summary length.

2. **Scanning with Custom Summary Length**:

   ```bash
   tldir scan -s 5000 /path/to/directory
   ```

   - Sets the summary length to 5000 tokens.

3. **Including Hidden Files**:

   ```bash
   tldir scan -i /path/to/directory
   ```

   - Includes hidden files and directories in the scanning process.

4. **Asking a Question Interactively**:

   ```bash
   tldir ask /path/to/directory
   ```

   - Enters interactive mode to ask questions.

5. **Asking a Question via Command Line**:

   ```bash
   tldir ask "/path/to/directory" "What are the main topics covered in the documents?"
   ```

   - Provides an immediate answer without entering interactive mode.

---

**Testing and Validation**

---

- **Unit Tests**:
  - Ensure each component (scanning, chunking, summarization, embedding, retrieval) works as expected.
  - Test with a variety of file types and sizes.

- **Integration Tests**:
  - Validate the end-to-end functionality of the tool.
  - Test with directories containing different file structures.

- **Performance Tests**:
  - Measure processing time and resource utilization.
  - Optimize for directories of varying sizes.

- **User Acceptance Tests**:
  - Gather feedback from users to ensure the tool meets their needs.
  - Iterate based on usability and functionality.

---

By following these user stories and acceptance criteria, the `tldir` command-line tool will effectively scan directories, summarize content, and provide a powerful querying interface, all while utilizing efficient Rust-based libraries and adhering to the specified requirements.