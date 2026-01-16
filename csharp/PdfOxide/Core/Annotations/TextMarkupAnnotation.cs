using System;
using PdfOxide.Exceptions;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Text markup types for highlighting, underlining, striking out, and squiggly annotations.
    /// </summary>
    public enum TextMarkupType
    {
        /// <summary>Highlight markup (background color).</summary>
        Highlight = 0,

        /// <summary>Underline markup.</summary>
        Underline = 1,

        /// <summary>StrikeOut markup (strikethrough).</summary>
        StrikeOut = 2,

        /// <summary>Squiggly markup (wavy underline).</summary>
        Squiggly = 3,

        /// <summary>Unknown markup type.</summary>
        Unknown = -1,
    }

    /// <summary>
    /// Represents a text markup annotation on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Text markup annotations are used to highlight, underline, strike through,
    /// or mark text with squiggly lines. These are applied to specific text regions
    /// using quadrilateral points.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var markup = annotation as TextMarkupAnnotation;
    /// if (markup != null)
    /// {
    ///     Console.WriteLine($"Type: {markup.MarkupType}");
    ///     Console.WriteLine($"Color: {markup.Color}");
    ///     Console.WriteLine($"Opacity: {markup.Opacity}");
    /// }
    /// </code>
    /// </example>
    public sealed class TextMarkupAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type
        {
            get
            {
                return MarkupType switch
                {
                    TextMarkupType.Highlight => AnnotationType.Highlight,
                    TextMarkupType.Underline => AnnotationType.Underline,
                    TextMarkupType.StrikeOut => AnnotationType.StrikeOut,
                    TextMarkupType.Squiggly => AnnotationType.Squiggly,
                    _ => AnnotationType.Unknown,
                };
            }
        }

        /// <summary>
        /// Gets the markup type.
        /// </summary>
        /// <value>The markup type.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public TextMarkupType MarkupType
        {
            get
            {
                ThrowIfDisposed();
                var markupType = NativeMethods.PdfTextMarkupAnnotationGetType(
                    _handle.DangerousGetHandle(), out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return (TextMarkupType)markupType;
            }
        }

        /// <summary>
        /// Gets whether this is a highlight annotation.
        /// </summary>
        public bool IsHighlight => MarkupType == TextMarkupType.Highlight;

        /// <summary>
        /// Gets whether this is an underline annotation.
        /// </summary>
        public bool IsUnderline => MarkupType == TextMarkupType.Underline;

        /// <summary>
        /// Gets whether this is a strikeout annotation.
        /// </summary>
        public bool IsStrikeOut => MarkupType == TextMarkupType.StrikeOut;

        /// <summary>
        /// Gets whether this is a squiggly annotation.
        /// </summary>
        public bool IsSquiggly => MarkupType == TextMarkupType.Squiggly;

        internal TextMarkupAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
