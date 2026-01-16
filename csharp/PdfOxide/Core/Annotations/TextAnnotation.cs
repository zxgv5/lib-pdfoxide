using System;
using PdfOxide.Exceptions;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Icon types for text annotations (sticky notes).
    /// </summary>
    public enum TextAnnotationIcon
    {
        /// <summary>Comment icon.</summary>
        Comment = 0,

        /// <summary>Key icon.</summary>
        Key = 1,

        /// <summary>Note icon.</summary>
        Note = 2,

        /// <summary>Help icon.</summary>
        Help = 3,

        /// <summary>NewParagraph icon.</summary>
        NewParagraph = 4,

        /// <summary>Paragraph icon.</summary>
        Paragraph = 5,

        /// <summary>Insert icon.</summary>
        Insert = 6,

        /// <summary>Unknown icon.</summary>
        Unknown = -1,
    }

    /// <summary>
    /// Represents a text annotation (sticky note) on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Text annotations display as small icons on the page that reveal
    /// their content when opened. Common uses include comments, notes,
    /// and markup feedback.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var annotation = annot as TextAnnotation;
    /// if (annotation != null)
    /// {
    ///     Console.WriteLine($"Icon: {annotation.Icon}");
    ///     Console.WriteLine($"Open: {annotation.IsOpen}");
    ///     Console.WriteLine($"Contents: {annotation.Contents}");
    /// }
    /// </code>
    /// </example>
    public sealed class TextAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type => AnnotationType.Text;

        /// <summary>
        /// Gets the icon type for this text annotation.
        /// </summary>
        /// <value>The icon type.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public TextAnnotationIcon Icon
        {
            get
            {
                ThrowIfDisposed();
                var icon = NativeMethods.PdfTextAnnotationGetIcon(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return (TextAnnotationIcon)icon;
            }
        }

        /// <summary>
        /// Gets whether the annotation is open/expanded.
        /// </summary>
        /// <value>True if open, false if closed.</value>
        public bool IsOpen
        {
            get
            {
                ThrowIfDisposed();
                var open = NativeMethods.PdfTextAnnotationGetOpen(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return open != 0;
            }
        }

        internal TextAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
