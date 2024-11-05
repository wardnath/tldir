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
  - Files are chunked into manageable sizes for processing
  - Chunk sizes are optimized for the model's input requirements

- **Summarization**:
  - Each chunk is summarized using Candle's quantized Phi model
  - Summaries are aggregated to create a top-level summary
  - The final summary does not exceed 8192 tokens as specified by SUMMARY_LENGTH

- **Embedding Generation**:
  - Embeddings for each chunk are generated using fastembed-rs
  - Embeddings are stored locally for efficient retrieval

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

**Technical Implementation**

---

The tool uses:
- Candle for ML model operations with quantized Phi model for efficient CPU inference
- fastembed-rs for generating embeddings
- SQLite/Chroma for embedding storage and retrieval

---

**Note on Dependencies**

To ensure the tool runs smoothly, the following dependencies are required:

- Rust environment for running the tool and associated libraries.
- **chroma**, **text-splitter**, **clap**, **text-embedding-inference**, and **candle** installed and properly configured.
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

6. **Top Level Summary**:

   The top level summary is stored in `.tldir/summary.txt`.
   ```bash
   cat .tldir/summary.txt
   ```
   Convenience method to view the summary without entering interactive mode.
   ```bash
   tldir summarize /path/to/directory
   ```


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

--- 

devise a set of user stories / acceptance criteria for a command line tool that scans directories for utf-8 encoded files ; then chunks and summarizes recursively to folder .tldir eg if a directory contains a python file and md file it will map chunking then reduce by summarizing subchunks into folder for toplevel chunks. 
* the cli bin is invoked by tldir <dirname> 
* one of the parameters is SUMMARY_LENGTH which is set to 8192 tokens ala tiktoken 
please: 
* [ ] define the cli args and the output of tldr --help
* [ ] define the contents of the .tldir file ; it should contain locally stored embeddings via sqlite / chroma but also a top-level summary capped by summary length defined above 
* [ ] use text-embeddings-inference for embeddings, chroma / sqlite for retrieval questions tldir ask <dirname> 
* [ ] use text-splitter for chunking 
* [ ] use candle for quantized phi model for summarization 
* [ ] use clap for cli parsing 
