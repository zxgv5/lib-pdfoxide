# Phase 3 Plan: Advanced DOM Manipulation & Annotations

## Overview

Phase 3 extends Phase 2 with advanced PDF manipulation capabilities:
- DOM element access and iteration
- Text element finding and modification
- Image element operations
- Annotation support (20+ types)
- Advanced page operations

## Phase 3 Goals

### 1. DOM Navigation & Element Access

**Rust FFI (`src/ffi/dom_elements.rs`)**
```rust
// Element enumeration
pub fn pdf_page_find_elements(handle: *const PdfPageHandle, element_type: i32, 
    out_ids: *mut *mut c_char, error_code: *mut i32) -> i32

// Element access
pub fn pdf_page_get_element(handle: *const PdfPageHandle, element_id: *const c_char,
    error_code: *mut i32) -> *mut PdfElementHandle

pub fn pdf_element_get_type(handle: *const PdfElementHandle) -> i32
pub fn pdf_element_get_bbox(handle: *const PdfElementHandle,
    x: *mut f32, y: *mut f32, width: *mut f32, height: *mut f32)
```

**C# Wrapper (`csharp/PdfOxide/Core/PdfElement.cs`)**
```csharp
public abstract class PdfElement : IDisposable
{
    public string Id { get; }
    public ElementType Type { get; }
    public Rect BoundingBox { get; }
    
    public static PdfElement FromNative(IntPtr handle);
}

public sealed class TextElement : PdfElement
{
    public string Text { get; set; }
    public float FontSize { get; }
    public string FontName { get; }
    public Color Color { get; set; }
}

public sealed class ImageElement : PdfElement
{
    public byte[] ImageData { get; }
    public ImageFormat Format { get; }
    public (int Width, int Height) Dimensions { get; }
}
```

### 2. Text Element Operations

**Capabilities**
- Find text by pattern/substring
- Replace text
- Modify font properties
- Change color
- Get/set properties

**API**
```csharp
var textElements = page.FindElements(ElementType.Text);
foreach (var text in textElements)
{
    if (text.Text.Contains("old"))
    {
        text.Text = text.Text.Replace("old", "new");
        text.Color = Color.Red;
    }
}
```

### 3. Image Element Operations

**Capabilities**
- Access image data
- Get format (JPEG, PNG, etc.)
- Get dimensions
- Get DPI/resolution
- Alternative text

**API**
```csharp
var images = page.FindElements(ElementType.Image);
foreach (var img in images)
{
    Console.WriteLine($"Image: {img.Dimensions.Width}x{img.Dimensions.Height}");
    Console.WriteLine($"Format: {img.Format}");
    
    byte[] data = img.ImageData;
    File.WriteAllBytes($"extracted_{img.Id}.jpg", data);
}
```

### 4. Annotation Support

**Annotation Types (20+)**
- Text (sticky note)
- Link
- FreeText
- Line
- Square/Circle
- Polygon/PolyLine
- Highlight/Underline/Squiggly/Strikeout
- Stamp
- Caret
- Ink
- Popup
- File attachment
- Sound
- Movie
- Widget (forms)
- Screen
- PrinterMark
- TrapNet
- Watermark
- 3D
- Redaction

**API Design**
```csharp
public abstract class Annotation : IDisposable
{
    public string Id { get; }
    public AnnotationType Type { get; }
    public Rect Rect { get; set; }
    public string Contents { get; set; }
    public Color Color { get; set; }
    public DateTime Created { get; }
    public DateTime Modified { get; set; }
}

public sealed class TextAnnotation : Annotation
{
    public TextAnnotationIcon Icon { get; set; }
    public bool IsOpen { get; set; }
}

public sealed class LinkAnnotation : Annotation
{
    public Uri Uri { get; set; }
}

public sealed class HighlightAnnotation : Annotation
{
    public Rect[] QuadPoints { get; set; }
}
```

### 5. Page Element Operations

**Add/Remove/Access**
```csharp
// Add text
page.AddText("New text", new Rect(100, 100, 200, 120), 
    fontSize: 12, color: Color.Black);

// Add rectangle
page.AddShape(ShapeType.Rectangle, new Rect(50, 50, 150, 150),
    fillColor: Color.White, strokeColor: Color.Black, strokeWidth: 1);

// Remove element
page.RemoveElement(elementId);

// Get elements with filter
var allText = page.GetElements(ElementType.Text);
var redText = page.GetElements(e => e is TextElement t && t.Color == Color.Red);
```

### 6. Search Functionality

**Text Search Across Pages**
```csharp
public class PdfSearchResult
{
    public int PageIndex { get; set; }
    public TextElement Element { get; set; }
    public int StartIndex { get; set; }
    public int Length { get; set; }
}

public IEnumerable<PdfSearchResult> SearchText(string query, 
    SearchOptions options = null)
{
    // Find all occurrences across all pages
}
```

## Implementation Schedule

### Phase 3a: DOM Elements (Week 1)
- [ ] Element enumeration FFI
- [ ] Element type mapping
- [ ] TextElement wrapper
- [ ] ImageElement wrapper
- [ ] Examples: finding and iterating elements

### Phase 3b: Text Operations (Week 2)
- [ ] Text find/replace FFI
- [ ] Font and color modification
- [ ] Search functionality
- [ ] Examples: text replacement workflows

### Phase 3c: Annotations (Week 3)
- [ ] Annotation enumeration FFI
- [ ] Annotation base class and subtypes
- [ ] Add/remove annotation APIs
- [ ] Examples: annotating PDFs

### Phase 3d: Integration & Examples (Week 4)
- [ ] Advanced examples (batch operations, workflows)
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Documentation finalization

## Rust FFI Modules Structure

```
src/ffi/
├── dom_elements.rs (NEW - 400 lines)
│   ├── Element enumeration
│   ├── Element access
│   ├── Element property getters/setters
│   └── Type conversions
├── annotations.rs (NEW - 300 lines)
│   ├── Annotation enumeration
│   ├── Annotation creation
│   ├── Annotation property access
│   └── Annotation removal
├── text_operations.rs (NEW - 250 lines)
│   ├── Text find/replace
│   ├── Text property modification
│   └── Search support
└── image_operations.rs (NEW - 150 lines)
    ├── Image data extraction
    ├── Image properties
    └── Format conversion
```

## C# Wrapper Classes

```
csharp/PdfOxide/Core/
├── Elements/
│   ├── PdfElement.cs (abstract base)
│   ├── TextElement.cs
│   ├── ImageElement.cs
│   ├── PathElement.cs
│   └── TableElement.cs
├── Annotations/
│   ├── Annotation.cs (abstract base)
│   ├── TextAnnotation.cs
│   ├── LinkAnnotation.cs
│   ├── HighlightAnnotation.cs
│   ├── [20+ annotation types]
│   └── AnnotationType.cs (enum)
└── Search/
    ├── SearchOptions.cs
    ├── SearchResult.cs
    └── SearchEngine.cs
```

## API Compatibility

**Breaking Changes:** None
- All Phase 3 APIs are additive
- Phase 1 & 2 APIs remain unchanged
- Backward compatible

**Target Frameworks:**
- .NET Standard 2.0+
- .NET Framework 4.7.2+
- .NET 5.0+
- .NET 6.0+
- .NET 7.0+
- .NET 8.0+

## Success Criteria

✅ All DOM element types accessible via C#
✅ Text finding and modification working
✅ Image extraction functional
✅ 20+ annotation types supported
✅ Search working across pages
✅ Comprehensive examples provided
✅ Performance: <5% overhead vs Rust
✅ All tests passing
✅ 100% API documentation
✅ Zero memory leaks

## Risk Mitigation

**Potential Issues & Solutions:**

| Issue | Risk | Mitigation |
|-------|------|-----------|
| Complex type mappings | High | Comprehensive enum mapping tests |
| Memory allocation patterns | High | Extensive memory cleanup testing |
| Circular dependencies | Medium | Careful ref counting in FFI |
| Thread safety | Medium | Document non-thread-safe handles |
| Performance degradation | Low | Benchmark critical paths |

## Deliverables Summary

- Rust FFI: 4 new modules, ~1,100 lines
- C# Wrappers: ~25 new classes, ~3,500 lines
- Documentation: Examples, API docs, migration guide
- Tests: Comprehensive coverage for new features
- Total: ~4,600 lines of new code

## Next Phases (Post-Phase 3)

**Phase 4: Advanced Features**
- Digital signatures
- Form field handling
- PDF compliance (PDF/A, PDF/UA)
- Advanced graphics operations

**Phase 5: Distribution**
- NuGet package creation
- Multi-platform native libraries
- CI/CD pipeline
- Release automation

**Phase 6: Ecosystem**
- Sample applications
- Tutorial documentation
- Community contributions
- Performance optimizations
