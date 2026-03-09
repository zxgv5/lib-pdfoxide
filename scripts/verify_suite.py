import os
import subprocess
import time
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor

CORPORA = {
    "veraPDF": Path(os.path.expanduser("~/projects/veraPDF-corpus")),
    "pdfjs": Path(os.path.expanduser("~/projects/pdf_oxide_tests/pdfs_pdfjs")),
    "safedocs": Path(os.path.expanduser("~/projects/pdf_oxide_tests/pdfs_safedocs")),
}

# 7 intentionally broken PDFs from veraPDF that should fail (matching README)
EXPECTED_FAILURES = {
    "isartor-6-1-2-t01-fail-a.pdf",
    "isartor-6-1-2-t02-fail-a.pdf",
    "isartor-6-1-3-t01-fail-a.pdf",
    "isartor-6-1-3-t02-fail-a.pdf",
    "isartor-6-1-3-t03-fail-a.pdf",
    "isartor-6-1-4-t01-fail-a.pdf",
    "isartor-6-1-4-t02-fail-a.pdf",
}

def verify_pdf(pdf_path, binary_path, mode):
    try:
        # 30s timeout per PDF/mode
        res = subprocess.run(
            [binary_path, str(pdf_path)],
            capture_output=True,
            text=True,
            timeout=30
        )
        if res.returncode == 0:
            return "PASS", None
        
        # Check for panic in stderr
        if "panic" in res.stderr.lower() or "panicked" in res.stderr.lower():
            return "PANIC", res.stderr
        
        return "FAIL", res.stderr
    except subprocess.TimeoutExpired:
        return "TIMEOUT", None
    except Exception as e:
        return "ERROR", str(e)

def run_suite():
    # Ensure binary is built
    print("Building release example...")
    subprocess.run(["cargo", "build", "--release", "--example", "extract_text_simple"], check=True)
    binary = "./target/release/examples/extract_text_simple"

    for name, path in CORPORA.items():
        if not path.exists():
            print(f"Skipping {name} (path not found: {path})")
            continue
        
        print(f"\nVerifying {name} corpus at {path}...")
        pdfs = list(path.rglob("*.pdf")) + list(path.rglob("*.PDF"))
        print(f"Found {len(pdfs)} files")
        
        stats = {"PASS": 0, "FAIL": 0, "PANIC": 0, "TIMEOUT": 0, "ERROR": 0}
        panics = []
        failures = []
        
        # Parallelize verification
        with ThreadPoolExecutor(max_workers=os.cpu_count()) as executor:
            futures = {executor.submit(verify_pdf, pdf, binary, "text"): pdf for pdf in pdfs}
            
            for i, future in enumerate(futures):
                pdf = futures[future]
                res, err = future.result()
                
                # Check if this failure was expected
                if res == "FAIL" and pdf.name in EXPECTED_FAILURES:
                    res = "PASS" # Treat as pass for stats if intentionally broken
                
                stats[res] += 1
                if res == "PANIC":
                    panics.append((pdf, err))
                elif res == "FAIL" and pdf.name not in EXPECTED_FAILURES:
                    failures.append((pdf, err))
                
                if (i + 1) % 100 == 0:
                    print(f"  [{i+1}/{len(pdfs)}] pass={stats['PASS']} fail={stats['FAIL']} panic={stats['PANIC']}")

        print(f"\nResults for {name}:")
        for k, v in stats.items():
            print(f"  {k}: {v}")
        
        if panics:
            print("\n!!! PANICS DETECTED !!!")
            for p, e in panics:
                print(f"  - {p}")
        
        if failures:
            print("\nFailures (unexpected):")
            for f, e in failures[:10]: # Show first 10
                print(f"  - {f}")

if __name__ == "__main__":
    run_suite()
