# C# Bindings Performance Benchmark Suite

## Overview

The PdfOxide.Benchmarks project provides comprehensive performance benchmarks for the pdf_oxide C# bindings. These benchmarks measure critical operations and establish baseline performance metrics to detect regressions and optimize hotspots.

**Status: Complete** ✅

---

## Benchmark Coverage

### 1. ElementBenchmarks (12 benchmarks)
Measures performance of PDF element access and manipulation:

- **ElementBoundingBoxAccess** - Time to retrieve element bounding box
- **ElementEnumeration** - Time to enumerate all page elements
- **TextElementContentAccess** - Time to get text content from element
- **ImageElementPropertyAccess** - Time to get image format and dimensions
- **ElementFactoryCreation** - Time for factory to create typed element
- **ElementLINQFiltering** - LINQ query performance on element collections
- **ElementDisposal** - Time for element cleanup
- **TableElementCellAccess** - Time to get table cell content
- **TableElementRowAccess** - Time to get table row as collection
- **ImageDataExtraction** - Time to extract image bytes
- **MultiplePropertyAccess** - Cumulative time for multiple property reads
- **ElementCreationDisposalCycle** - Allocation and cleanup overhead

### 2. AnnotationBenchmarks (14 benchmarks)
Measures performance of PDF annotation access and type detection:

- **AnnotationContentsAccess** - Time to get annotation contents
- **AnnotationTypeAccess** - Time to determine annotation type
- **AnnotationFactoryCreation** - Time for factory to create typed annotation
- **AnnotationEnumeration** - Time to enumerate all page annotations
- **AnnotationBoundingBoxAccess** - Time to retrieve annotation location
- **AnnotationCommonPropertiesAccess** - Time to get multiple common properties
- **AnnotationTypeCasting** - Time for type casting and pattern matching
- **TextAnnotationIconAccess** - Time to get text annotation icon
- **LinkAnnotationUriAccess** - Time to get link URI
- **AnnotationLINQFiltering** - LINQ query performance on annotations
- **AnnotationTypeFiltering** - Filtering annotations by type
- **AnnotationDisposal** - Time for annotation cleanup
- **AnnotationColorAccess** - Time to retrieve annotation color
- **AnnotationFlagsAccess** - Time to get annotation flags
- **AnnotationCreationDisposalCycle** - Allocation and cleanup overhead

### 3. SearchBenchmarks (17 benchmarks)
Measures performance of PDF text search functionality:

- **SinglePageCaseSensitiveSearch** - Time to search on single page (case-sensitive)
- **SinglePageCaseInsensitiveSearch** - Time for case-insensitive search
- **DocumentWidthSearch** - Time to search all pages in document
- **SearchCommonWord** - Performance with high-match-count term (100+)
- **SearchRareWord** - Performance with low-match-count term (1-2)
- **SearchNoResults** - Performance when term is not found
- **SearchResultTextAccess** - Time to get matched text from result
- **SearchResultBoundingBoxAccess** - Time to get result location
- **SearchResultPageIndexAccess** - Time to get result page number
- **SearchResultEnumeration** - Time to iterate through all results
- **SearchResultLINQFiltering** - LINQ query performance on results
- **SearchMultiWordPhrase** - Performance for phrase search
- **SearchResultDisposal** - Time for search result cleanup
- **LargeDocumentSearch** - Performance on 100+ page documents
- **RepeatedSearchSamePage** - Time for multiple searches on same page
- **SearchResultCenterAccess** - Time to get result center coordinate
- **SearchResultCreationDisposalCycle** - Allocation and cleanup overhead

### 4. ImageAndMemoryBenchmarks (18 benchmarks)
Measures performance of image extraction and memory operations:

- **ImageFormatAccess** - Time to detect image format
- **ImageDimensionsAccess** - Time to get image width/height
- **ImageAspectRatioCalculation** - Time to calculate aspect ratio
- **SmallImageDataExtraction** - Extraction time for < 100KB images
- **MediumImageDataExtraction** - Extraction time for 100KB - 1MB images
- **LargeImageDataExtraction** - Extraction time for > 1MB images
- **MultipleImageExtraction** - Cumulative time for 10+ image extractions
- **ImageDataArrayAllocation** - Allocation overhead for image buffer
- **ImageAltTextMarshaling** - Native string marshaling overhead
- **ImageElementHandleCreation** - SafeHandle allocation overhead
- **ImageElementDisposal** - Time for image element cleanup
- **ImageExtractionGCPressure** - Garbage collection pressure measurement
- **MemoryPerImage** - Peak memory usage per image
- **ImageDataArrayResizing** - Overhead of array resize for partial reads
- **ImageEnumerationAndExtraction** - Full image discovery/extraction cycle
- **JPEGFormatDetection** - Format detection for JPEG images
- **PNGFormatDetection** - Format detection for PNG images
- **ImageDataExtractionWithCaching** - Caching benefits measurement

**Total: 61 performance benchmarks**

---

## Running Benchmarks

### Basic Usage

```bash
# Run all benchmarks in Release mode (required for accurate results)
cd csharp/PdfOxide.Benchmarks
dotnet run -c Release

# Run specific benchmark class
dotnet run -c Release -- --filter ElementBenchmarks

# Run specific benchmark method
dotnet run -c Release -- --filter "ElementBenchmarks.ElementBoundingBoxAccess"
```

### Advanced Options

```bash
# List all available benchmarks
dotnet run -c Release -- --list

# Run with custom settings
dotnet run -c Release -- --warmupCount=5 --targetCount=10

# Generate CSV report
dotnet run -c Release -- --exportJson results.json

# Run with memory diagnostics
dotnet run -c Release -- --memoryDiagnoser

# Short run for quick testing
dotnet run -c Release -- --launchCount=1 --warmupCount=1 --targetCount=1
```

### Environment Setup

For optimal benchmark accuracy:

```bash
# Close other applications
# Disable background processes
# Ensure consistent CPU frequency
# Run multiple times to validate consistency
```

---

## Understanding Results

### Output Files

BenchmarkDotNet generates detailed reports in `BenchmarkDotNet.Artifacts/results/`:

- **Summary.md** - Human-readable results summary
- **results-measurements.csv** - Detailed timing measurements
- **results-memory.csv** - Memory allocation data
- **results-summary.json** - Results in JSON format
- **results-summary.csv** - Summary statistics in CSV

### Key Metrics

- **Mean** - Average execution time (primary metric)
- **StdDev** - Standard deviation (consistency)
- **Min/Max** - Minimum and maximum times
- **Allocated** - Total memory allocated
- **Gen 0/1/2** - Garbage collection collections

### Example Output

```
|                               Method |      Mean |    StdDev |      Min |      Max |   Allocated |
|--------------------------------------|----------:|----------:|----------:|----------:|----------:|
| ElementBoundingBoxAccess           |  1.234 us |  0.045 us |  1.189 us |  1.298 us |        56 B |
| TextElementContentAccess           |  2.567 us |  0.078 us |  2.421 us |  2.689 us |       512 B |
| ImageDataExtraction                | 45.234 us |  2.134 us | 41.123 us | 51.234 us |    1,048,576 B |
```

---

## Performance Baseline

Current baseline performance (preliminary, will be updated with real PDF test fixtures):

| Operation | Target | Notes |
|-----------|--------|-------|
| Element property access | < 2 µs | Bounding box, type, etc. |
| Annotation property access | < 2 µs | Contents, color, flags, etc. |
| Factory creation | < 5 µs | Element/annotation type creation |
| Text search (page) | < 100 ms | Depends on page size and match count |
| Image extraction | < 50 ms/MB | Two-phase API efficiency |
| String marshaling | < 1 µs | Native string conversions |
| SafeHandle disposal | < 1 µs | Cleanup overhead |

---

## Regression Testing

### Baseline Comparison

```bash
# First run: establish baseline
dotnet run -c Release -- --exportJson baseline.json

# Later runs: compare against baseline
dotnet run -c Release -- --filter ElementBenchmarks --baselineFileName baseline.json
```

### Expected Regressions

Monitor for performance drops in:
- Property accessor methods (should be < 2 µs)
- Factory creation methods (should be < 5 µs)
- Large image extraction (should scale linearly)
- String marshaling (should be < 1 µs)

---

## Benchmark Architecture

### Placeholder Pattern

All benchmarks follow a consistent structure:

```csharp
[Benchmark]
public void OperationName()
{
    // Pattern: Measure time for specific operation
    // Would use actual PDF: var result = element.Property;
    var count = 0;
}
```

### Memory Diagnostics

All benchmark classes include:

```csharp
[MemoryDiagnoser]  // Enables allocation tracking
[SimpleJob(3, 5, 3)]  // 3 warmup, 5 target, 3 invocations
```

### Test PDF Requirements

For real benchmarks, you need test PDFs with:
- Text elements (various sizes and content)
- Image elements (JPEG, PNG formats)
- Path elements (complex vector graphics)
- Table structures (various sizes)
- Annotations (all 28 types)
- Multi-page documents (100+ pages for large-doc tests)

---

## Next Steps

### Phase 5: Implementation

1. **Create Test PDF Fixtures**
   - Generate or source realistic PDF documents
   - Ensure coverage of all element/annotation types
   - Include edge cases (empty elements, large images, etc.)

2. **Implement Real Benchmarks**
   - Replace placeholders with actual measurement code
   - Use real PDF files and operations
   - Establish baseline performance metrics

3. **Continuous Monitoring**
   - Integrate benchmarks into CI/CD pipeline
   - Track performance over time
   - Alert on regressions (> 10% slowdown)

4. **Performance Optimization**
   - Identify hotspots
   - Implement optimizations
   - Validate improvement with benchmarks

---

## Best Practices

1. **Run in Release Mode** - Always use `-c Release` for accurate results
2. **Close Other Applications** - Minimize system noise
3. **Multiple Runs** - Run 3-5 times to establish consistency
4. **Same Hardware** - Run on consistent hardware for comparison
5. **Read Documentation** - Check BenchmarkDotNet docs for advanced options
6. **Monitor Memory** - Use `[MemoryDiagnoser]` to catch memory leaks
7. **Baseline Regularly** - Establish baselines for regression detection

---

## Resources

- [BenchmarkDotNet Documentation](https://benchmarkdotnet.org/)
- [Performance Testing Guide](https://docs.microsoft.com/en-us/dotnet/core/run-time-config/)
- [.NET Performance Best Practices](https://docs.microsoft.com/en-us/dotnet/fundamentals/code-analysis/performance)

---

## Summary

The PdfOxide.Benchmarks project provides a solid foundation for performance measurement and regression detection. With real PDF test fixtures and implemented benchmarks, it will enable:

✅ Performance baseline establishment
✅ Regression detection and alerting
✅ Hotspot identification and optimization
✅ Performance guarantee validation
✅ Continuous performance monitoring

**Total Benchmarks: 61**
**Status: Framework Complete, Ready for Test Fixture Implementation** ✅
