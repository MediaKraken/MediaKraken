# mk_lib_ocr

OCR library supporting Tesseract and Surya backends.

## Prerequisites

### Tesseract Backend

Requires system libraries:

```bash
apt install libtesseract-dev libleptonica-dev tesseract-ocr
```

For additional language support:

```bash
apt install tesseract-ocr-<lang>
# e.g., tesseract-ocr-deu for German
```

Verify installation:

```bash
tesseract --version
pkg-config --modversion lept
```

### Surya Backend

Requires the Surya CLI tool. Install from [surya](https://github.com/VikParuchuri/surya):

```bash
pip install surya-ocr
# or via cargo if using the Rust implementation
```

Verify installation:

```bash
surya --version
```

## Usage

```rust
use mk_lib_ocr::tesseract_ocr::mk_ocr_tesseract;
use mk_lib_ocr::surya_ocr::mk_ocr_surya;

// Tesseract (auto-detect language)
let result = mk_ocr_tesseract("image.png").await?;
println!("Text: {}", result.text);

// Tesseract with specific language
let result = mk_ocr_tesseract_with_lang("image.png", "deu").await?;

// Surya
let result = mk_ocr_surya("image.png").await?;
println!("Text: {}", result.text);
```

## Build

```bash
cargo check
cargo build --release
```

Release profile is optimized: LTO enabled, symbols stripped, single codegen unit.
