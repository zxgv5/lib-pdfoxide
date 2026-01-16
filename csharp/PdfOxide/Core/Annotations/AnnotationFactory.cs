using System;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Factory for creating appropriate Annotation subclass instances.
    /// </summary>
    /// <remarks>
    /// This factory creates the correct annotation type based on the
    /// annotation type constant returned from the native FFI layer.
    /// </remarks>
    internal static class AnnotationFactory
    {
        /// <summary>
        /// Creates an annotation of the appropriate type from a native handle.
        /// </summary>
        /// <param name="handle">The native annotation handle.</param>
        /// <returns>An Annotation instance of the appropriate type, or null if unable to determine type.</returns>
        public static Annotation Create(NativeHandle handle)
        {
            if (handle == null)
                return null;

            var annotationType = NativeMethods.PdfAnnotationGetType(handle.DangerousGetHandle());

            return annotationType switch
            {
                // Text annotations (sticky notes)
                (int)AnnotationType.Text => new TextAnnotation(handle),

                // Link annotations
                (int)AnnotationType.Link => new LinkAnnotation(handle),

                // Text markup (Highlight, Underline, StrikeOut, Squiggly)
                (int)AnnotationType.Highlight or
                (int)AnnotationType.Underline or
                (int)AnnotationType.StrikeOut or
                (int)AnnotationType.Squiggly => new TextMarkupAnnotation(handle),

                // FreeText annotations (text boxes)
                (int)AnnotationType.FreeText => new FreeTextAnnotation(handle),

                // Shape annotations (Square, Circle, Line, Polygon, PolyLine)
                (int)AnnotationType.Square or
                (int)AnnotationType.Circle or
                (int)AnnotationType.Line or
                (int)AnnotationType.Polygon or
                (int)AnnotationType.PolyLine => new ShapeAnnotation(handle),

                // All other types (Stamp, Popup, Ink, FileAttachment, Redact, Watermark, etc.)
                _ => new SpecialAnnotation(handle),
            };
        }

        /// <summary>
        /// Creates an annotation of the appropriate type from a page.
        /// </summary>
        /// <param name="pageHandle">The page handle.</param>
        /// <param name="index">The index of the annotation on the page.</param>
        /// <returns>An Annotation instance, or null if the annotation cannot be retrieved.</returns>
        public static Annotation CreateFromPage(IntPtr pageHandle, int index)
        {
            // Placeholder: full implementation would enumerate annotations on page
            // and return the one at the specified index
            return null;
        }
    }
}
