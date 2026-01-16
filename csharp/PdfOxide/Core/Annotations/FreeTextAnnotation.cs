using System;
using PdfOxide.Exceptions;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Represents a FreeText annotation (text box) on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// FreeText annotations display text directly on the page, optionally
    /// within a box. They can be used for callouts, typewriter-style notes,
    /// and permanent text overlays.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var freetext = annotation as FreeTextAnnotation;
    /// if (freetext != null)
    /// {
    ///     Console.WriteLine($"Text: {freetext.Contents}");
    ///     Console.WriteLine($"Font: {freetext.FontName} {freetext.FontSize}pt");
    ///     Console.WriteLine($"BBox: {freetext.BoundingBox}");
    /// }
    /// </code>
    /// </example>
    public sealed class FreeTextAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type => AnnotationType.FreeText;

        /// <summary>
        /// Gets the font name for the text.
        /// </summary>
        /// <value>The font name (e.g., "Helvetica", "Times-Roman").</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public string FontName
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfFreeTextAnnotationGetFontName(
                    _handle.DangerousGetHandle(), out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);

                if (ptr == IntPtr.Zero)
                    return string.Empty;

                try
                {
                    return StringMarshaler.PtrToString(ptr);
                }
                finally
                {
                    NativeMethods.FreeString(ptr);
                }
            }
        }

        /// <summary>
        /// Gets the font size for the text in points.
        /// </summary>
        /// <value>The font size.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public float FontSize
        {
            get
            {
                ThrowIfDisposed();
                var size = NativeMethods.PdfFreeTextAnnotationGetFontSize(
                    _handle.DangerousGetHandle(), out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return size;
            }
        }

        internal FreeTextAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
